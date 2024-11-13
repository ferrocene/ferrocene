use std::cell::{OnceCell, RefCell};
use std::ffi::{CStr, CString};

use libc::c_uint;
use rustc_codegen_ssa::traits::{
    BuilderMethods, ConstCodegenMethods, CoverageInfoBuilderMethods, MiscCodegenMethods,
};
use rustc_data_structures::fx::{FxHashMap, FxIndexMap};
use rustc_llvm::RustString;
use rustc_middle::mir::coverage::CoverageKind;
use rustc_middle::ty::Instance;
use rustc_middle::ty::layout::HasTyCtxt;
use rustc_target::abi::Size;
use tracing::{debug, instrument};

use crate::builder::Builder;
use crate::common::{AsCCharPtr, CodegenCx};
use crate::coverageinfo::map_data::FunctionCoverageCollector;
use crate::llvm;

pub(crate) mod ffi;
pub(crate) mod map_data;
mod mapgen;

/// A context object for maintaining all state needed by the coverageinfo module.
pub(crate) struct CrateCoverageContext<'ll, 'tcx> {
    /// Coverage data for each instrumented function identified by DefId.
    pub(crate) function_coverage_map:
        RefCell<FxIndexMap<Instance<'tcx>, FunctionCoverageCollector<'tcx>>>,
    pub(crate) pgo_func_name_var_map: RefCell<FxHashMap<Instance<'tcx>, &'ll llvm::Value>>,
    pub(crate) mcdc_condition_bitmap_map: RefCell<FxHashMap<Instance<'tcx>, Vec<&'ll llvm::Value>>>,

    covfun_section_name: OnceCell<CString>,
}

impl<'ll, 'tcx> CrateCoverageContext<'ll, 'tcx> {
    pub(crate) fn new() -> Self {
        Self {
            function_coverage_map: Default::default(),
            pgo_func_name_var_map: Default::default(),
            mcdc_condition_bitmap_map: Default::default(),
            covfun_section_name: Default::default(),
        }
    }

    fn take_function_coverage_map(
        &self,
    ) -> FxIndexMap<Instance<'tcx>, FunctionCoverageCollector<'tcx>> {
        self.function_coverage_map.replace(FxIndexMap::default())
    }

    /// LLVM use a temp value to record evaluated mcdc test vector of each decision, which is
    /// called condition bitmap. In order to handle nested decisions, several condition bitmaps can
    /// be allocated for a function body. These values are named `mcdc.addr.{i}` and are a 32-bit
    /// integers. They respectively hold the condition bitmaps for decisions with a depth of `i`.
    fn try_get_mcdc_condition_bitmap(
        &self,
        instance: &Instance<'tcx>,
        decision_depth: u16,
    ) -> Option<&'ll llvm::Value> {
        self.mcdc_condition_bitmap_map
            .borrow()
            .get(instance)
            .and_then(|bitmap_map| bitmap_map.get(decision_depth as usize))
            .copied() // Dereference Option<&&Value> to Option<&Value>
    }
}

impl<'ll, 'tcx> CodegenCx<'ll, 'tcx> {
    pub(crate) fn coverageinfo_finalize(&self) {
        mapgen::finalize(self)
    }

    /// Returns the section name to use when embedding per-function coverage information
    /// in the object file, according to the target's object file format. LLVM's coverage
    /// tools use information from this section when producing coverage reports.
    ///
    /// Typical values are:
    /// - `__llvm_covfun` on Linux
    /// - `__LLVM_COV,__llvm_covfun` on macOS (includes `__LLVM_COV,` segment prefix)
    /// - `.lcovfun$M` on Windows (includes `$M` sorting suffix)
    fn covfun_section_name(&self) -> &CStr {
        self.coverage_cx().covfun_section_name.get_or_init(|| {
            CString::new(llvm::build_byte_buffer(|s| unsafe {
                llvm::LLVMRustCoverageWriteFuncSectionNameToString(self.llmod, s);
            }))
            .expect("covfun section name should not contain NUL")
        })
    }

    /// For LLVM codegen, returns a function-specific `Value` for a global
    /// string, to hold the function name passed to LLVM intrinsic
    /// `instrprof.increment()`. The `Value` is only created once per instance.
    /// Multiple invocations with the same instance return the same `Value`.
    fn get_pgo_func_name_var(&self, instance: Instance<'tcx>) -> &'ll llvm::Value {
        debug!("getting pgo_func_name_var for instance={:?}", instance);
        let mut pgo_func_name_var_map = self.coverage_cx().pgo_func_name_var_map.borrow_mut();
        pgo_func_name_var_map
            .entry(instance)
            .or_insert_with(|| create_pgo_func_name_var(self, instance))
    }
}

impl<'tcx> CoverageInfoBuilderMethods<'tcx> for Builder<'_, '_, 'tcx> {
    fn init_coverage(&mut self, instance: Instance<'tcx>) {
        let Some(function_coverage_info) =
            self.tcx.instance_mir(instance.def).function_coverage_info.as_deref()
        else {
            return;
        };

        // If there are no MC/DC bitmaps to set up, return immediately.
        if function_coverage_info.mcdc_bitmap_bits == 0 {
            return;
        }

        let fn_name = self.get_pgo_func_name_var(instance);
        let hash = self.const_u64(function_coverage_info.function_source_hash);
        let bitmap_bits = self.const_u32(function_coverage_info.mcdc_bitmap_bits as u32);
        self.mcdc_parameters(fn_name, hash, bitmap_bits);

        // Create pointers named `mcdc.addr.{i}` to stack-allocated condition bitmaps.
        let mut cond_bitmaps = vec![];
        for i in 0..function_coverage_info.mcdc_num_condition_bitmaps {
            // MC/DC intrinsics will perform loads/stores that use the ABI default
            // alignment for i32, so our variable declaration should match.
            let align = self.tcx.data_layout.i32_align.abi;
            let cond_bitmap = self.alloca(Size::from_bytes(4), align);
            llvm::set_value_name(cond_bitmap, format!("mcdc.addr.{i}").as_bytes());
            self.store(self.const_i32(0), cond_bitmap, align);
            cond_bitmaps.push(cond_bitmap);
        }

        self.coverage_cx().mcdc_condition_bitmap_map.borrow_mut().insert(instance, cond_bitmaps);
    }

    #[instrument(level = "debug", skip(self))]
    fn add_coverage(&mut self, instance: Instance<'tcx>, kind: &CoverageKind) {
        // Our caller should have already taken care of inlining subtleties,
        // so we can assume that counter/expression IDs in this coverage
        // statement are meaningful for the given instance.
        //
        // (Either the statement was not inlined and directly belongs to this
        // instance, or it was inlined *from* this instance.)

        let bx = self;

        let Some(function_coverage_info) =
            bx.tcx.instance_mir(instance.def).function_coverage_info.as_deref()
        else {
            debug!("function has a coverage statement but no coverage info");
            return;
        };

        let mut coverage_map = bx.coverage_cx().function_coverage_map.borrow_mut();
        let func_coverage = coverage_map
            .entry(instance)
            .or_insert_with(|| FunctionCoverageCollector::new(instance, function_coverage_info));

        match *kind {
            CoverageKind::SpanMarker | CoverageKind::BlockMarker { .. } => unreachable!(
                "marker statement {kind:?} should have been removed by CleanupPostBorrowck"
            ),
            CoverageKind::CounterIncrement { id } => {
                func_coverage.mark_counter_id_seen(id);
                // We need to explicitly drop the `RefMut` before calling into
                // `instrprof_increment`, as that needs an exclusive borrow.
                drop(coverage_map);

                // The number of counters passed to `llvm.instrprof.increment` might
                // be smaller than the number originally inserted by the instrumentor,
                // if some high-numbered counters were removed by MIR optimizations.
                // If so, LLVM's profiler runtime will use fewer physical counters.
                let num_counters =
                    bx.tcx().coverage_ids_info(instance.def).max_counter_id.as_u32() + 1;
                assert!(
                    num_counters as usize <= function_coverage_info.num_counters,
                    "num_counters disagreement: query says {num_counters} but function info only has {}",
                    function_coverage_info.num_counters
                );

                let fn_name = bx.get_pgo_func_name_var(instance);
                let hash = bx.const_u64(function_coverage_info.function_source_hash);
                let num_counters = bx.const_u32(num_counters);
                let index = bx.const_u32(id.as_u32());
                debug!(
                    "codegen intrinsic instrprof.increment(fn_name={:?}, hash={:?}, num_counters={:?}, index={:?})",
                    fn_name, hash, num_counters, index,
                );
                bx.instrprof_increment(fn_name, hash, num_counters, index);
            }
            CoverageKind::ExpressionUsed { id } => {
                func_coverage.mark_expression_id_seen(id);
            }
            CoverageKind::CondBitmapUpdate { index, decision_depth } => {
                drop(coverage_map);
                let cond_bitmap = bx
                    .coverage_cx()
                    .try_get_mcdc_condition_bitmap(&instance, decision_depth)
                    .expect("mcdc cond bitmap should have been allocated for updating");
                let cond_index = bx.const_i32(index as i32);
                bx.mcdc_condbitmap_update(cond_index, cond_bitmap);
            }
            CoverageKind::TestVectorBitmapUpdate { bitmap_idx, decision_depth } => {
                drop(coverage_map);
                let cond_bitmap = bx.coverage_cx()
                                    .try_get_mcdc_condition_bitmap(&instance, decision_depth)
                                    .expect("mcdc cond bitmap should have been allocated for merging into the global bitmap");
                assert!(
                    bitmap_idx as usize <= function_coverage_info.mcdc_bitmap_bits,
                    "bitmap index of the decision out of range"
                );

                let fn_name = bx.get_pgo_func_name_var(instance);
                let hash = bx.const_u64(function_coverage_info.function_source_hash);
                let bitmap_index = bx.const_u32(bitmap_idx);
                bx.mcdc_tvbitmap_update(fn_name, hash, bitmap_index, cond_bitmap);
                bx.mcdc_condbitmap_reset(cond_bitmap);
            }
        }
    }
}

/// Calls llvm::createPGOFuncNameVar() with the given function instance's
/// mangled function name. The LLVM API returns an llvm::GlobalVariable
/// containing the function name, with the specific variable name and linkage
/// required by LLVM InstrProf source-based coverage instrumentation. Use
/// `bx.get_pgo_func_name_var()` to ensure the variable is only created once per
/// `Instance`.
fn create_pgo_func_name_var<'ll, 'tcx>(
    cx: &CodegenCx<'ll, 'tcx>,
    instance: Instance<'tcx>,
) -> &'ll llvm::Value {
    let mangled_fn_name: &str = cx.tcx.symbol_name(instance).name;
    let llfn = cx.get_fn(instance);
    unsafe {
        llvm::LLVMRustCoverageCreatePGOFuncNameVar(
            llfn,
            mangled_fn_name.as_c_char_ptr(),
            mangled_fn_name.len(),
        )
    }
}

pub(crate) fn write_filenames_section_to_buffer<'a>(
    filenames: impl IntoIterator<Item = &'a str>,
    buffer: &RustString,
) {
    let (pointers, lengths) = filenames
        .into_iter()
        .map(|s: &str| (s.as_c_char_ptr(), s.len()))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    unsafe {
        llvm::LLVMRustCoverageWriteFilenamesSectionToBuffer(
            pointers.as_ptr(),
            pointers.len(),
            lengths.as_ptr(),
            lengths.len(),
            buffer,
        );
    }
}

pub(crate) fn write_mapping_to_buffer(
    virtual_file_mapping: Vec<u32>,
    expressions: Vec<ffi::CounterExpression>,
    code_regions: &[ffi::CodeRegion],
    branch_regions: &[ffi::BranchRegion],
    mcdc_branch_regions: &[ffi::MCDCBranchRegion],
    mcdc_decision_regions: &[ffi::MCDCDecisionRegion],
    buffer: &RustString,
) {
    unsafe {
        llvm::LLVMRustCoverageWriteMappingToBuffer(
            virtual_file_mapping.as_ptr(),
            virtual_file_mapping.len() as c_uint,
            expressions.as_ptr(),
            expressions.len() as c_uint,
            code_regions.as_ptr(),
            code_regions.len() as c_uint,
            branch_regions.as_ptr(),
            branch_regions.len() as c_uint,
            mcdc_branch_regions.as_ptr(),
            mcdc_branch_regions.len() as c_uint,
            mcdc_decision_regions.as_ptr(),
            mcdc_decision_regions.len() as c_uint,
            buffer,
        );
    }
}

pub(crate) fn hash_bytes(bytes: &[u8]) -> u64 {
    unsafe { llvm::LLVMRustCoverageHashByteArray(bytes.as_c_char_ptr(), bytes.len()) }
}

pub(crate) fn mapping_version() -> u32 {
    unsafe { llvm::LLVMRustCoverageMappingVersion() }
}
