use crate::coverage::reporting::*;
use crate::coverage::*;
use crate::instrumentation_profile::types::*;
use crate::util::*;
use anyhow::{bail, Result};
use nom::error::Error as NomError;
use object::{Endian, Endianness, Object, ObjectSection, ReadCache, ReadRef, Section};
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tracing::{debug, error, trace, warn};

/// Stores the instrumentation profile and information from the coverage mapping sections in the
/// object files in order to construct a coverage report. Inspired, from the LLVM implementation
/// with some differences/simplifications due to the fact this only hands instrumentation profiles
/// for coverage.
///
/// So what the LLVM one has that this one doesn't yet:
///
/// 1. DenseMap<size_t, DenseSet<size_t>> RecordProvenance
/// 2. std::vector<FunctionRecord> functions (this is probably taken straight from
/// InstrumentationProfile
/// 3. DenseMap<size_t, SmallVector<unsigned, 0>> FilenameHash2RecordIndices
/// 4. Vec<Pair<String, u64>> FuncHashMismatches
#[derive(Debug)]
pub struct CoverageMapping<'a> {
    profile: &'a InstrumentationProfile,
    object_files: &'a [PathBuf],
    allow_parsing_failures: bool,
    version: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum LlvmSection {
    CoverageMap,
    ProfileNames,
    ProfileCounts,
    ProfileData,
    CoverageFunctions,
}

#[derive(Copy, Clone, Debug)]
pub enum SectionReadError {
    EmptySection(LlvmSection),
    MissingSection(LlvmSection),
    InvalidPathList,
}

impl fmt::Display for SectionReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySection(s) => write!(f, "empty section: {:?}", s),
            Self::MissingSection(s) => write!(f, "missing section: {:?}", s),
            Self::InvalidPathList => write!(f, "unable to read path list"),
        }
    }
}

impl Error for SectionReadError {}

pub fn read_object_file(object: &Path, version: u64) -> Result<CoverageMappingInfo> {
    // I believe vnode sections added by llvm are unnecessary

    let binary_data = ReadCache::new(BufReader::new(fs::File::open(object)?));
    let object_file = object::File::parse(&binary_data)?;

    let prof_counts = object_file
        .section_by_name("__llvm_prf_cnts")
        .or(object_file.section_by_name(".lprfc"))
        .and_then(|x| parse_profile_counters(object_file.endianness(), &x).ok());

    debug!("Parsed prf_cnts: {:?}", prof_counts);

    let prof_data = object_file
        .section_by_name("__llvm_prf_data")
        .or(object_file.section_by_name(".lprfd"))
        .and_then(|x| parse_profile_data(object_file.endianness(), &x).ok());

    debug!("Parsed prf_data section: {:?}", prof_data);

    let cov_fun = object_file
        .section_by_name("__llvm_covfun")
        .or(object_file.section_by_name(".lcovfun"))
        .map(|x| parse_coverage_functions(object_file.endianness(), &x))
        .ok_or(SectionReadError::MissingSection(
            LlvmSection::CoverageFunctions,
        ))??;

    debug!("Parsed covfun section: {:?}", cov_fun);

    let cov_map = object_file
        .section_by_name("__llvm_covmap")
        .or(object_file.section_by_name(".lcovmap"))
        .map(|x| parse_coverage_mapping(object_file.endianness(), &x, version))
        .ok_or(SectionReadError::MissingSection(LlvmSection::CoverageMap))??;

    debug!("Parsed covmap section: {:?}", cov_map);

    Ok(CoverageMappingInfo {
        cov_map,
        cov_fun,
        prof_counts,
        prof_data,
    })
}

impl<'a> CoverageMapping<'a> {
    pub fn new(
        object_files: &'a [PathBuf],
        profile: &'a InstrumentationProfile,
        allow_parsing_failures: bool,
    ) -> Result<Self> {
        let version = match profile.version() {
            Some(v) => v,
            None => bail!("Invalid profile instrumentation, no version number provided"),
        };

        Ok(Self {
            profile,
            object_files,
            allow_parsing_failures,
            version,
        })
    }

    /// All counters of type `CounterKind::ProfileInstrumentation` can be used in function regions
    /// other than their own (particulary for functions which are only called from one location).
    /// This gathers them all to use as a base list of counters.
    pub(crate) fn get_simple_counters(&self, func: &FunctionRecordV3) -> FxHashMap<Counter, i64> {
        let mut result = FxHashMap::default();
        result.insert(Counter::default(), 0);
        let record = self.profile.find_record_by_hash(func.header.name_hash);
        if let Some(func_record) = record.as_ref() {
            result.reserve(func_record.record.counts.len());
            for (id, count) in func_record.record.counts.iter().enumerate() {
                result.insert(Counter::instrumentation(id as u64), *count as i64);
            }
        }
        result
    }

    pub fn generate_subreport<P>(&self, mut predicate: P) -> Result<CoverageReport>
    where
        P: FnMut(&[PathBuf]) -> bool,
    {
        let mut report = CoverageReport::default();
        //let base_region_ids = info.get_simple_counters(self.profile);

        for result in self.mapping_info_iter() {
            let info = result?;
            for func in &info.cov_fun {
                let base_region_ids = self.get_simple_counters(func);
                let paths = info.get_files_from_id(func.header.filenames_ref);
                if paths.is_empty() || !predicate(&paths) {
                    continue;
                }

                let mut region_ids = base_region_ids.clone();

                for region in func.regions.iter().filter(|x| !x.count.is_expression()) {
                    let count = region_ids.get(&region.count).copied().unwrap_or_default();
                    let result = report
                        .files
                        .entry(paths[region.file_id].clone())
                        .or_default();
                    result.insert(region.loc.clone(), count as usize);
                }

                let mut pending_exprs = vec![];

                for (expr_index, expr) in func.expressions.iter().enumerate().rev() {
                    let lhs = region_ids.get(&expr.lhs);
                    let rhs = region_ids.get(&expr.rhs);
                    match (lhs, rhs) {
                        (Some(lhs), Some(rhs)) => {
                            let count: i64 = match expr.kind {
                                ExprKind::Subtract => {
                                    trace!("Subtracting counts: {} - {}", lhs, rhs);
                                    // TODO these saturating ops need investigation. Potentially
                                    // something is wrong.
                                    lhs.saturating_sub(*rhs)
                                }
                                ExprKind::Add => {
                                    trace!("Adding counts: {} + {}", lhs, rhs);
                                    lhs.saturating_add(*rhs)
                                }
                            };

                            let counter = Counter {
                                kind: CounterType::Expression(expr.kind),
                                id: expr_index as _,
                            };

                            region_ids.insert(counter, count);
                            for expr_region in func.regions.iter().filter(|x| {
                                x.count.is_expression() && x.count.id == expr_index as u64
                            }) {
                                let result = report
                                    .files
                                    .entry(paths[expr_region.file_id].clone())
                                    .or_default();
                                result.insert(expr_region.loc.clone(), count as _);
                            }
                        }
                        _ => {
                            let lhs_none = lhs.is_none();
                            let rhs_none = rhs.is_none();
                            // These counters have been optimised out, so just add then in as 0
                            if lhs_none && expr.lhs.is_instrumentation() {
                                region_ids.insert(expr.lhs, 0);
                            }
                            if rhs_none && expr.rhs.is_instrumentation() {
                                region_ids.insert(expr.rhs, 0);
                            }
                            pending_exprs.push(Some((expr_index, expr)));
                            continue;
                        }
                    }
                }
                let mut index = 0;
                let mut cleared_expressions = 0;
                let mut tries_left = pending_exprs.len() + 1;
                while pending_exprs.len() != cleared_expressions {
                    assert!(tries_left > 0);
                    if index >= pending_exprs.len() {
                        index = 0;
                        tries_left -= 1;
                    }
                    let (expr_index, expr) = match pending_exprs[index].as_ref() {
                        Some((idx, expr)) => (idx, expr),
                        None => {
                            index += 1;
                            continue;
                        }
                    };
                    let lhs = region_ids.get(&expr.lhs);
                    let rhs = region_ids.get(&expr.rhs);
                    match (lhs, rhs) {
                        (Some(lhs), Some(rhs)) => {
                            let count = match expr.kind {
                                ExprKind::Subtract => lhs - rhs,
                                ExprKind::Add => lhs + rhs,
                            };

                            let counter = Counter {
                                kind: CounterType::Expression(expr.kind),
                                id: *expr_index as _,
                            };

                            region_ids.insert(counter, count);
                            for expr_region in func.regions.iter().filter(|x| {
                                x.count.is_expression() && x.count.id == *expr_index as u64
                            }) {
                                let result = report
                                    .files
                                    .entry(paths[expr_region.file_id].clone())
                                    .or_default();
                                result.insert(expr_region.loc.clone(), count as _);
                            }
                            pending_exprs[index] = None;
                            cleared_expressions += 1;
                        }
                        _ => {
                            index += 1;
                            continue;
                        }
                    }
                }
            }
        }
        Ok(report)
    }

    pub fn generate_report(&self) -> Result<CoverageReport> {
        self.generate_subreport(|_| true)
    }

    pub fn mapping_info_iter(&self) -> impl Iterator<Item = Result<CoverageMappingInfo>> + '_ {
        self.object_files
            .iter()
            .map(move |object| (object, read_object_file(object, self.version)))
            .skip_while(move |(object, result)| match result {
                Err(ref e) if !self.allow_parsing_failures => {
                    error!("{} couldn't be interpreted: {}", object.display(), e);
                    true
                }
                _ => false,
            })
            .map(|(_, result)| result)
    }
}

fn parse_coverage_mapping<'data, R: ReadRef<'data>>(
    endian: Endianness,
    section: &Section<'data, '_, R>,
    version: u64,
) -> Result<FxHashMap<u64, Vec<PathBuf>>, SectionReadError> {
    if let Ok(mut data) = section.data() {
        let mut result = FxHashMap::default();
        while !data.is_empty() {
            let data_len = data.len();
            // Read the number of affixed function records (now just 0 as not in this header)
            debug_assert_eq!(endian.read_i32_bytes(data[0..4].try_into().unwrap()), 0);
            let filename_data_len = endian.read_i32_bytes(data[4..8].try_into().unwrap());
            // Read the length of the affixed string that contains encoded coverage mapping data (now 0
            // as not in this header)
            debug_assert_eq!(endian.read_i32_bytes(data[8..12].try_into().unwrap()), 0);
            let _format_version = endian.read_i32_bytes(data[12..16].try_into().unwrap());

            let hash = md5::compute(&data[16..(filename_data_len as usize + 16)]);
            let hash = endian.read_u64_bytes(hash.0[..8].try_into().unwrap());

            //let bytes = &data[16..(16 + filename_data_len as usize)];
            let bytes = &data[16..];
            let (bytes, file_strings) = parse_path_list(bytes, version)
                .map_err(|_: nom::Err<NomError<_>>| SectionReadError::InvalidPathList)?;
            result.insert(hash, file_strings);
            let read_len = data_len - bytes.len();
            let padding = if !bytes.is_empty() && (read_len & 0x07) != 0 {
                8 - (read_len & 0x07)
            } else {
                0
            };
            if padding > bytes.len() {
                break;
            }
            data = &bytes[padding..];
        }
        Ok(result)
    } else {
        Err(SectionReadError::EmptySection(LlvmSection::CoverageMap))
    }
}

// Strictly speaking, parsing __llvm_covfun ought to depend on the version number
// observed in the __llvm_covmap headers.
//
// But in practice there haven’t been any non-additive changes to the covfun format
// in a long time, which is why you can get away with parsing them independently,
// at least for now.
fn parse_coverage_functions<'data, R: ReadRef<'data>>(
    endian: Endianness,
    section: &Section<'data, '_, R>,
) -> Result<Vec<FunctionRecordV3>, SectionReadError> {
    debug!("Parsing coverage functions");
    if let Ok(original_data) = section.data() {
        let mut bytes = original_data;
        let mut res = vec![];
        let section_len = bytes.len();
        while !bytes.is_empty() {
            let name_hash = endian.read_u64_bytes(bytes[0..8].try_into().unwrap());
            let data_len = endian.read_u32_bytes(bytes[8..12].try_into().unwrap());
            let fn_hash = endian.read_u64_bytes(bytes[12..20].try_into().unwrap());
            let filenames_ref = endian.read_u64_bytes(bytes[20..28].try_into().unwrap());
            let header = FunctionRecordHeader {
                name_hash,
                data_len,
                fn_hash,
                filenames_ref,
            };
            let _start_len = bytes[28..].len();
            bytes = &bytes[28..];

            let (data, id_len) = parse_leb128::<NomError<_>>(bytes).unwrap();
            bytes = data;
            let mut filename_indices = vec![];
            for _ in 0..id_len {
                let (data, id) = parse_leb128::<NomError<_>>(bytes).unwrap(); // Issue
                filename_indices.push(id);
                bytes = data;
            }

            let (data, expr_len) = parse_leb128::<NomError<_>>(bytes).unwrap();
            let expr_len = expr_len as usize;
            bytes = data;
            let mut exprs = vec![Expression::default(); expr_len];
            for i in 0..expr_len {
                let (data, lhs) = parse_leb128::<NomError<_>>(bytes).unwrap();
                let (data, rhs) = parse_leb128::<NomError<_>>(data).unwrap();
                let lhs = parse_counter(lhs, &mut exprs);
                let rhs = parse_counter(rhs, &mut exprs);
                exprs[i].lhs = lhs;
                exprs[i].rhs = rhs;
                bytes = data;
            }

            let (data, regions) =
                parse_mapping_regions(bytes, &filename_indices, &mut exprs).unwrap();

            res.push(FunctionRecordV3 {
                header,
                regions,
                expressions: exprs,
            });

            // Todo set couners for expansion regions - counter of expansion region is the counter
            // of the first region from the expanded file. This requires multiple passes to
            // correctly propagate across all nested regions. N.B. I haven't seen any expansion
            // regions in use so may not be an issue!

            bytes = data;
            let function_len = section_len - bytes.len(); // this should match header

            let padding = if function_len < section_len && (function_len & 0x07) != 0 {
                8 - (function_len & 0x07)
            } else {
                0
            };

            if padding > bytes.len() {
                break;
            }
            // Now apply padding, and if hash is 0 move on as it's a dummy otherwise add to result
            // And decide what end type will be
            bytes = &bytes[padding..];
        }
        Ok(res)
    } else {
        error!("Can't read data for coverage function section");
        Err(SectionReadError::EmptySection(
            LlvmSection::CoverageFunctions,
        ))
    }
}

/// This code is ported from `RawCoverageMappingReader::readMappingRegionsSubArray`
fn parse_mapping_regions<'a>(
    mut bytes: &'a [u8],
    file_indices: &[u64],
    expressions: &mut Vec<Expression>,
) -> IResult<&'a [u8], Vec<CounterMappingRegion>> {
    let mut mapping = vec![];
    for i in file_indices {
        let (data, regions_len) = parse_leb128(bytes)?;
        bytes = data;
        let mut last_line = 0;
        for _ in 0..regions_len {
            let mut mcdc_params = None;

            let mut false_count = Counter::default();
            let mut kind = RegionKind::Code;
            let (data, raw_header) = parse_leb128(bytes)?;
            bytes = data;
            let mut expanded_file_id = 0;
            let mut counter = parse_counter(raw_header, expressions);
            if counter.is_zero() {
                if raw_header & Counter::ENCODING_EXPANSION_REGION_BIT > 0 {
                    kind = RegionKind::Expansion;
                    expanded_file_id = raw_header >> Counter::ENCODING_TAG_AND_EXP_REGION_BITS;
                    if expanded_file_id >= file_indices.len() as u64 {
                        panic!("expanded_file_id is invalid");
                    }
                } else {
                    let shifted_counter = raw_header >> Counter::ENCODING_TAG_AND_EXP_REGION_BITS;
                    match shifted_counter.try_into() {
                        Ok(RegionKind::Code) | Ok(RegionKind::Skipped) => {}
                        Ok(RegionKind::Branch) => {
                            kind = RegionKind::Branch;
                            let (data, c1) = parse_leb128(bytes)?;
                            let (data, c2) = parse_leb128(data)?;

                            counter = parse_counter(c1, expressions);
                            false_count = parse_counter(c2, expressions);
                            bytes = data;
                        }
                        Ok(RegionKind::MCDCBranch) => {
                            kind = RegionKind::MCDCBranch;
                            let (data, c1) = parse_leb128(bytes)?;
                            let (data, c2) = parse_leb128(data)?;

                            counter = parse_counter(c1, expressions);
                            false_count = parse_counter(c2, expressions);

                            let (data, id1) = parse_leb128(data)?;
                            let (data, tid1) = parse_leb128(data)?;
                            let (data, fid1) = parse_leb128(data)?;
                            bytes = data;

                            mcdc_params = Some(MCDCParams::Branch(BranchParameters {
                                id: id1.try_into().unwrap(),
                                false_cond: fid1.try_into().unwrap(),
                                true_cond: tid1.try_into().unwrap(),
                            }));
                        }
                        Ok(RegionKind::MCDCDecision) => {
                            kind = RegionKind::MCDCDecision;
                            let (data, bidx) = parse_leb128(data)?;
                            let (data, nc) = parse_leb128(data)?;
                            bytes = data;

                            mcdc_params = Some(MCDCParams::Decision(DecisionParameters {
                                bitmap_idx: bidx.try_into().unwrap(),
                                num_conditions: nc.try_into().unwrap(),
                            }));
                        }
                        e => panic!("Malformed: {:?}", e),
                    }
                }
            }

            let (data, delta_line) = parse_leb128(bytes)?;
            let (data, column_start) = parse_leb128(data)?;
            let (data, lines_len) = parse_leb128(data)?;
            let (data, column_end) = parse_leb128(data)?;
            bytes = data;

            let (column_start, column_end) = if column_start == 0 && column_end == 0 {
                (1usize, usize::MAX)
            } else {
                (column_start as usize, column_end as usize)
            };

            let line_start = last_line + delta_line as usize;
            let line_end = line_start + lines_len as usize;
            last_line = line_start;

            // Add region working-out-stuff
            mapping.push(CounterMappingRegion {
                kind,
                count: counter,
                false_count,
                file_id: *i as usize,
                expanded_file_id: expanded_file_id as _,
                loc: SourceLocation {
                    line_start,
                    line_end,
                    column_start,
                    column_end,
                },
                mcdc_params,
            });
        }
    }
    Ok((bytes, mapping))
}

fn parse_profile_data<'data, R: ReadRef<'data>>(
    endian: Endianness,
    section: &Section<'data, '_, R>,
) -> Result<Vec<ProfileData>, SectionReadError> {
    if let Ok(data) = section.data() {
        let mut bytes = data;
        let mut res = vec![];
        while !bytes.is_empty() {
            // bytes.len() >= 24 {
            let name_md5 = endian.read_u64_bytes(bytes[..8].try_into().unwrap());
            let structural_hash = endian.read_u64_bytes(bytes[8..16].try_into().unwrap());

            let _counter_ptr = endian.read_u64_bytes(bytes[16..24].try_into().unwrap());
            let counters_location = 24 + 16;
            if bytes.len() <= counters_location {
                bytes = &bytes[counters_location..];
                let counters_len = endian.read_u32_bytes(bytes[..4].try_into().unwrap());
                // TODO Might need to get the counter offset and get the list of counters from this?
                // And potentially check against the maximum number of counters just to make sure that
                // it's not being exceeded?
                //
                // Also counters_len >= 1 so this should be checked to make sure it's not malformed

                bytes = &bytes[8..];

                res.push(ProfileData {
                    name_md5,
                    structural_hash,
                    counters_len,
                });
            } else {
                bytes = &[];
            }
        }
        if !bytes.is_empty() {
            warn!("{} bytes left in profile data", bytes.len());
        }
        Ok(res)
    } else {
        Err(SectionReadError::EmptySection(LlvmSection::ProfileData))
    }
}

fn parse_profile_counters<'data, R: ReadRef<'data>>(
    endian: Endianness,
    section: &Section<'data, '_, R>,
) -> Result<Vec<u64>, SectionReadError> {
    if let Ok(data) = section.data() {
        let mut result = vec![];
        for i in (0..data.len()).step_by(8) {
            if data.len() < (i + 8) {
                break;
            }
            result.push(endian.read_u64_bytes(data[i..(i + 8)].try_into().unwrap()));
        }
        Ok(result)
    } else {
        Err(SectionReadError::EmptySection(LlvmSection::ProfileCounts))
    }
}

/// The equivalent llvm function is `RawCoverageMappingReader::decodeCounter`. This makes it
/// stateless as I don't want to be maintaining an expression vector and clearing it and
/// repopulating for every function record.
fn parse_counter(input: u64, exprs: &mut Vec<Expression>) -> Counter {
    let ty = (Counter::ENCODING_TAG_MASK & input) as u8;
    let id = input >> 2; // For zero we don't actually care about this but we'll still do it
    let kind = match ty {
        0 => CounterType::Zero,
        1 => CounterType::ProfileInstrumentation,
        2 | 3 => {
            let expr_kind = if ty == 2 {
                ExprKind::Subtract
            } else {
                ExprKind::Add
            };
            let id = id as usize;
            if exprs.len() <= id {
                debug!(
                    "Not enough expressions resizing {}->{}",
                    exprs.len(),
                    id + 1
                );
                exprs.resize(id + 1, Expression::default());
            }
            exprs[id].set_kind(expr_kind);
            CounterType::Expression(expr_kind)
        }
        _ => unreachable!(),
    };
    Counter { kind, id }
}
