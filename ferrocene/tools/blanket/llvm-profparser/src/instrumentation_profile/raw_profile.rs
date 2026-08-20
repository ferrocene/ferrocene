use crate::instrumentation_profile::types::*;
use crate::instrumentation_profile::*;
use crate::util::parse_string_ref;
use core::hash::Hash;
use nom::bytes::complete::take;
use nom::error::ParseError;
use nom::lib::std::ops::RangeFrom;
use nom::number::streaming::{u16 as nom_u16, u32 as nom_u32, u64 as nom_u64};
use nom::number::Endianness;
use nom::{
    error::{ContextError, ErrorKind},
    Err,
};
use nom::{InputIter, InputLength, Slice};
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::mem::size_of;
use tracing::{debug, error, trace};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum RawProfileError {
    Eof,
    UnrecognizedFormat,
    BadMagic(u64),
    UnsupportedVersion(usize),
    UnsupportedHashType,
    TooLarge,
    Truncated,
    Malformed,
    UnknownFunction,
    HashMismatch,
    CountMismatch,
    CounterOverflow,
    ValueSiteCountMismatch,
    CompressFailed,
    UncompressFailed,
    EmptyRawProfile,
}

const INSTR_PROF_NAME_SEP: char = '\u{1}';

pub type RawInstrProf32 = RawInstrProf<u32>;
pub type RawInstrProf64 = RawInstrProf<u64>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RawInstrProf<T>
where
    T: MemoryWidthExt,
{
    header: Header,
    data: Vec<ProfileData<T>>,
    records: Vec<InstrProfRecord>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header {
    endianness: Endianness,
    pub version: u64,
    pub binary_ids_len: u64,
    pub data_len: u64,
    pub padding_bytes_before_counters: u64,
    pub counters_len: u64,
    pub padding_bytes_after_counters: u64,
    pub names_len: u64,
    pub counters_delta: u64,
    pub names_delta: u64,
    pub value_kind_last: u64,
    pub num_bitmap_bytes: u64,
    pub padding_bytes_after_bitmap_bytes: u64,
    pub bitmap_delta: u64,
    pub num_vtables: u64,
    pub vnames_size: u64,
}

impl Header {
    #[inline(always)]
    fn version(&self) -> u64 {
        self.version & !VARIANT_MASKS_ALL
    }

    #[inline(always)]
    fn has_byte_coverage(&self) -> bool {
        (self.version & VARIANT_MASK_BYTE_COVERAGE) != 0
    }

    #[inline(always)]
    fn ir_profile(&self) -> bool {
        (self.version & VARIANT_MASK_IR_PROF) != 0
    }

    #[inline(always)]
    fn csir_profile(&self) -> bool {
        (self.version & VARIANT_MASK_CSIR_PROF) != 0
    }

    #[inline(always)]
    fn function_entry_only(&self) -> bool {
        (self.version & VARIANT_MASK_FUNCTION_ENTRY_ONLY) != 0
    }

    #[inline(always)]
    fn memory_profile(&self) -> bool {
        (self.version & VARIANT_MASK_MEMORY_PROFILE) != 0
    }

    #[inline(always)]
    fn counter_size(&self) -> usize {
        if self.has_byte_coverage() {
            1
        } else {
            8
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ProfileData<T> {
    name_ref: u64,
    func_hash: u64,
    counter_ptr: T,
    bitmap_ptr: Option<T>,
    function_addr: T,
    values_ptr_expr: T,
    num_counters: u32,
    /// This might just be two values?
    num_value_sites: [u16; ValueKind::MemOpSize as usize + 1],
    num_bitmap_bytes: u32,
}

impl<T> ProfileData<T> {
    fn len(&self) -> usize {
        16 + 4 + (2 * (ValueKind::MemOpSize as usize + 1)) + 3 * size_of::<T>()
    }
}

impl Header {
    pub fn max_counters_len(&self) -> i64 {
        ((8 * self.counters_len) + self.padding_bytes_after_counters) as i64
    }
}

/// Trait to represent memory widths. Currently just 32 or 64 bit. This implements Into<u64> so if
/// we ever move beyond 64 bit systems this code will have to change to Into<u128> or whatever the
/// next thing is.
pub trait MemoryWidthExt:
    Debug + Copy + Clone + Eq + PartialEq + Hash + Ord + PartialOrd + Display + Into<u64>
{
    const MAGIC: u64;

    fn nom_parse_fn<I>(endianness: Endianness) -> fn(_: I) -> IResult<I, Self, VerboseError<I>>
    where
        I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength;
}

impl MemoryWidthExt for u32 {
    const MAGIC: u64 = (255 << 56)
        | ('l' as u64) << 48
        | ('p' as u64) << 40
        | ('r' as u64) << 32
        | ('o' as u64) << 24
        | ('f' as u64) << 16
        | ('R' as u64) << 8
        | 129;

    fn nom_parse_fn<I>(endianness: Endianness) -> fn(_: I) -> IResult<I, Self, VerboseError<I>>
    where
        I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
    {
        nom_u32(endianness)
    }
}
impl MemoryWidthExt for u64 {
    const MAGIC: u64 = (255 << 56)
        | ('l' as u64) << 48
        | ('p' as u64) << 40
        | ('r' as u64) << 32
        | ('o' as u64) << 24
        | ('f' as u64) << 16
        | ('r' as u64) << 8
        | 129;

    fn nom_parse_fn<I>(endianness: Endianness) -> fn(_: I) -> IResult<I, Self, VerboseError<I>>
    where
        I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
    {
        nom_u64(endianness)
    }
}

fn file_endianness<T>(magic: &[u8; 8]) -> Endianness
where
    T: MemoryWidthExt,
{
    // native endian and reversed endian
    let provided = u64::from_le_bytes(*magic);
    if provided == T::MAGIC {
        Endianness::Little
    } else if provided.swap_bytes() == T::MAGIC {
        Endianness::Big
    } else {
        unreachable!("Invalid magic provided");
    }
}

impl<T> RawInstrProf<T>
where
    T: MemoryWidthExt,
{
    fn read_raw_counts<'a>(
        header: &Header,
        data: &ProfileData<T>,
        counter_offset: i64,
        mut bytes: &'a [u8],
    ) -> ParseResult<'a, InstrProfRecord> {
        let max_counters = header.max_counters_len();
        // From LLVM coverage mapping version 8 relative counter offsets are allowed which can be
        // signed
        // num 2 max 24 offset 7 counters len 3
        trace!(
            "Reading raw counts offset: {} max: {}. data {:?}",
            counter_offset,
            max_counters,
            data
        );
        if data.num_counters == 0
            || max_counters < 0
            || counter_offset < 0
            || counter_offset as u64 >= (header.counters_len * header.counter_size() as u64)
            || data.num_counters as i64 > max_counters
            || (header.version < 8 && counter_offset < 0)
            || counter_offset > max_counters
            || counter_offset + data.num_counters as i64 > max_counters
        {
            error!("consistency check for reading counts failed");
            //Err(Err::Failure(Error::new(bytes, ErrorKind::Satisfy))) TODO
            Err(Err::Failure(VerboseError::from_error_kind(
                bytes,
                ErrorKind::Satisfy,
            )))
        } else if counter_offset as usize > bytes.len() {
            let pos = &bytes[bytes.len()..];
            let inner = VerboseError::from_error_kind(pos, ErrorKind::Eof);
            Err(Err::Failure(VerboseError::add_context(
                pos,
                "end of file reached before counters offset",
                inner,
            )))
        } else {
            let mut counts = Vec::<u64>::with_capacity(data.num_counters as usize);
            bytes = &bytes[(counter_offset as usize)..];
            for _ in 0..(data.num_counters as usize) {
                let counter = if header.has_byte_coverage() {
                    let counter = bytes[0];
                    bytes = &bytes[1..];
                    (counter == 0) as u64
                } else {
                    let (b, counter) = nom_u64(header.endianness)(bytes)?;
                    bytes = b;
                    counter
                };
                counts.push(counter);
            }
            let record = InstrProfRecord {
                counts,
                ..Default::default()
            };
            Ok((bytes, record))
        }
    }

    fn read_value_profiling_data<'a>(
        header: &Header,
        data: &ProfileData<T>,
        bytes: &'a [u8],
        _record: &mut InstrProfRecord,
    ) -> ParseResult<'a, ()> {
        // record clear value data
        if data.num_value_sites.iter().all(|x| *x == 0) {
            // Okay so there's no value profiling data. So the next byte is actually a header
            // wewww
            Ok((bytes, ()))
        } else {
            let (_bytes, _total_size) = nom_u32(header.endianness)(bytes)?;
            todo!()
        }
    }
}

impl<T> InstrProfReader for RawInstrProf<T>
where
    T: MemoryWidthExt,
{
    type Header = Header;

    fn parse_bytes(mut input: &[u8]) -> ParseResult<'_, InstrumentationProfile> {
        if !input.is_empty() {
            let mut result = InstrumentationProfile::default();
            let (bytes, header) = Self::parse_header(input)?;
            // LLVM 11 and 12 are version 5. LLVM 13 is version 7
            let version_num = header.version();
            result.version = Some(version_num);
            result.is_ir = header.ir_profile();
            result.has_csir = header.csir_profile();
            if version_num > 7 {
                result.is_byte_coverage = header.has_byte_coverage();
                result.fn_entry_only = header.function_entry_only();
                result.memory_profiling = header.memory_profile();
            }
            if bytes.len() < header.binary_ids_len as usize {
                return Err(nom::Err::Failure(VerboseError::from_error_kind(
                    &bytes[bytes.len()..],
                    ErrorKind::Eof,
                )));
            }
            input = &bytes[(header.binary_ids_len as usize)..];
            let mut data_section = vec![];
            for _ in 0..header.data_len {
                let (bytes, data) = ProfileData::<T>::parse(input, &header)?;
                debug!("Parsed data section {:?}", data);
                data_section.push(data);
                if version_num > 8 {
                    let (bytes, v) = take(4usize)(bytes)?; // TODO WHAT AM I MISSING HERE?
                    debug!("Got those padding? bytes {:?}", v);
                    input = bytes;
                } else {
                    input = bytes;
                }
            }
            let bytes = match take(header.padding_bytes_before_counters as usize)(input) {
                Ok((b, _)) => b,
                Err(e) => {
                    error!("Failed to skip padding bytes");
                    return Err(e);
                }
            };
            input = bytes;
            let mut counters = vec![];
            let mut counters_delta = header.counters_delta;

            // Okay so the counters section looks a bit hairy. So as a brief explanation.
            // 1. The base offset is from CountersStart pointer to entry of the record. Meaning
            //    doing a nom type parsing we need to keep track of the total offset as counter
            //    records can be offset in the middle of the counter list.
            // 2. Also there may be some padding bytes before the last counter and end of counters
            //    section. This needs to be applied as well as padding_bytes_after_counters for
            //    total padding
            let mut total_offset = 0;
            let remaining_before_counters = input.len();
            for data in &data_section {
                let counters_offset = if header.version() == 8 {
                    // Why oh why!?
                    (data.counter_ptr.into() as i64 - counters_delta as i64) - total_offset
                } else {
                    0
                };
                let (bytes, record) = Self::read_raw_counts(&header, data, counters_offset, input)?;
                debug!("Read counter record {:?}", record);
                total_offset +=
                    counters_offset + (record.counts.len() * header.counter_size()) as i64;
                counters_delta -= data.len() as u64;
                counters.push(record);
                input = bytes;
            }
            let counters_end = header.padding_bytes_after_counters as usize
                + (header.counters_len as usize * header.counter_size())
                - (remaining_before_counters - input.len());
            debug!("Applying padding bytes after counters");
            let (bytes, _) = take(counters_end)(input)?;
            input = bytes;
            let end_length = input.len() - header.names_len as usize;
            let mut symtab = Symtab::default();
            while input.len() > end_length {
                let (new_bytes, names) = parse_string_ref(input)?;
                debug!(
                    "Complete names string: '{}'. Read {} bytes",
                    names,
                    input.len() - new_bytes.len()
                );
                input = new_bytes;
                for name in names.split(INSTR_PROF_NAME_SEP) {
                    debug!("Symbol name parsed: '{}'", name);
                    symtab.add_func_name(name.to_string(), Some(header.endianness));
                }
            }
            let padding = get_num_padding_bytes(header.names_len);
            let (bytes, _) = take(padding)(input)?;
            input = bytes;
            for (data, mut record) in data_section.iter().zip(counters.drain(..)) {
                let (bytes, _) =
                    Self::read_value_profiling_data(&header, data, input, &mut record)?;
                input = bytes;
                let name = symtab.names.get(&data.name_ref).cloned();
                let (hash, name_hash) = if name.is_some() {
                    // Previously this function calculated the function hash itself to be
                    // ultra-defensive against the profraw format changing hash calculation method
                    // so we try not to rely on reimplementing it. However, md5::compute was more
                    // expensive than initially assumed and using the precomputed one reduces
                    // runtime by 25% on benchmarks
                    (Some(data.func_hash), Some(data.name_ref))
                } else {
                    (None, None)
                };
                debug!("Parsed record: {:?} {:?} {:?}", name, hash, record);

                result.push_record(NamedInstrProfRecord {
                    name,
                    name_hash,
                    hash,
                    record,
                });
            }
            result.symtab = symtab;
            Ok((input, result))
        } else {
            // Okay return an error here
            todo!()
        }
    }

    fn parse_header(input: &[u8]) -> ParseResult<'_, Self::Header> {
        if Self::has_format(input) {
            let endianness = file_endianness::<T>(&input[..8].try_into().unwrap());
            let (bytes, version) = nom_u64(endianness)(&input[8..])?;
            debug!("Profraw version: {}", version & !VARIANT_MASKS_ALL);
            let (bytes, binary_ids_len) = if (version & !VARIANT_MASKS_ALL) >= 7 {
                nom_u64(endianness)(bytes)?
            } else {
                (bytes, 0)
            };
            let (bytes, data_len) = nom_u64(endianness)(bytes)?;
            let (bytes, padding_bytes_before_counters) = nom_u64(endianness)(bytes)?;
            let (bytes, counters_len) = nom_u64(endianness)(bytes)?;
            let (bytes, padding_bytes_after_counters) = nom_u64(endianness)(bytes)?;

            let (bytes, num_bitmap_bytes, padding_bytes_after_bitmap_bytes) =
                if (version & !VARIANT_MASKS_ALL) >= 9 {
                    let (bytes, num_bitmap_bytes) = nom_u64(endianness)(bytes)?;
                    let (bytes, num_bitmap_padding_bytes) = nom_u64(endianness)(bytes)?;
                    (bytes, num_bitmap_bytes, num_bitmap_padding_bytes)
                } else {
                    (bytes, 0, 0)
                };

            let (bytes, names_len) = nom_u64(endianness)(bytes)?;
            let (bytes, counters_delta) = nom_u64(endianness)(bytes)?;

            let (bytes, bitmap_delta) = if (version & !VARIANT_MASKS_ALL) >= 9 {
                let (bytes, bitmap_delta) = nom_u64(endianness)(bytes)?;
                (bytes, bitmap_delta)
            } else {
                (bytes, 0)
            };

            let (bytes, names_delta) = nom_u64(endianness)(bytes)?;

            let (bytes, num_vtables, vnames_size) = if (version & !VARIANT_MASKS_ALL) >= 10 {
                let (bytes, num_vtables) = nom_u64(endianness)(bytes)?;
                let (bytes, vnames_len) = nom_u64(endianness)(bytes)?;
                (bytes, num_vtables, vnames_len)
            } else {
                (bytes, 0, 0)
            };

            let (bytes, value_kind_last) = nom_u64(endianness)(bytes)?;

            let result = Header {
                endianness,
                version,
                binary_ids_len,
                data_len,
                padding_bytes_before_counters,
                counters_len,
                padding_bytes_after_counters,
                names_len,
                counters_delta,
                names_delta,
                value_kind_last,
                num_bitmap_bytes,
                padding_bytes_after_bitmap_bytes,
                bitmap_delta,
                num_vtables,
                vnames_size,
            };
            debug!("Read header {:?}", result);
            Ok((bytes, result))
        } else {
            //Err(Err::Failure(Error::new(input, ErrorKind::IsNot)))
            Err(Err::Failure(VerboseError::from_error_kind(
                input,
                ErrorKind::IsNot,
            )))
        }
    }

    fn has_format(mut input: impl Read) -> bool {
        let mut buffer: [u8; 8] = [0; 8];
        if input.read_exact(&mut buffer).is_ok() {
            let magic = u64::from_ne_bytes(buffer);
            T::MAGIC == magic || T::MAGIC == magic.swap_bytes()
        } else {
            false
        }
    }
}

impl<T> ProfileData<T>
where
    T: MemoryWidthExt,
{
    fn parse<'a>(
        bytes: &'a [u8],
        header: &Header,
    ) -> IResult<&'a [u8], Self, VerboseError<&'a [u8]>> {
        let endianness = header.endianness;
        let parse = T::nom_parse_fn(endianness);

        let (bytes, name_ref) = nom_u64(endianness)(bytes)?;
        let (bytes, func_hash) = nom_u64(endianness)(bytes)?;
        let (bytes, counter_ptr) = parse(bytes)?;
        let (bytes, bitmap_ptr) = if header.version() > 8 {
            let (bytes, bitmap_ptr) = parse(bytes)?;
            (bytes, Some(bitmap_ptr))
        } else {
            (bytes, None)
        };
        let (bytes, function_addr) = parse(bytes)?;
        let (bytes, values_ptr_expr) = parse(bytes)?;
        let (bytes, num_counters) = nom_u32(endianness)(bytes)?;
        let (bytes, value_0) = nom_u16(endianness)(bytes)?;
        let (bytes, value_1) = nom_u16(endianness)(bytes)?;
        let (bytes, num_bitmap_bytes) = if header.version() > 8 {
            nom_u32(endianness)(bytes)?
        } else {
            (bytes, 0)
        };

        Ok((
            bytes,
            Self {
                name_ref,
                func_hash,
                counter_ptr,
                bitmap_ptr,
                function_addr,
                values_ptr_expr,
                num_counters,
                num_value_sites: [value_0, value_1],
                num_bitmap_bytes,
            },
        ))
    }
}
