//! Implements calling functions from a native library.

use std::ops::Deref;

use libffi::high::call as ffi;
use libffi::low::CodePtr;
use rustc_abi::{BackendRepr, HasDataLayout, Size};
use rustc_middle::mir::interpret::Pointer;
use rustc_middle::ty::{self as ty, IntTy, UintTy};
use rustc_span::Symbol;

#[cfg_attr(
    not(all(
        target_os = "linux",
        target_env = "gnu",
        any(target_arch = "x86", target_arch = "x86_64")
    )),
    path = "trace/stub.rs"
)]
pub mod trace;

use crate::*;

/// The final results of an FFI trace, containing every relevant event detected
/// by the tracer.
#[allow(dead_code)]
#[cfg_attr(target_os = "linux", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct MemEvents {
    /// An list of memory accesses that occurred, in the order they occurred in.
    pub acc_events: Vec<AccessEvent>,
}

/// A single memory access.
#[allow(dead_code)]
#[cfg_attr(target_os = "linux", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum AccessEvent {
    /// A read occurred on this memory range.
    Read(AccessRange),
    /// A write may have occurred on this memory range.
    /// Some instructions *may* write memory without *always* doing that,
    /// so this can be an over-approximation.
    /// The range info, however, is reliable if the access did happen.
    /// If the second field is true, the access definitely happened.
    Write(AccessRange, bool),
}

impl AccessEvent {
    fn get_range(&self) -> AccessRange {
        match self {
            AccessEvent::Read(access_range) => access_range.clone(),
            AccessEvent::Write(access_range, _) => access_range.clone(),
        }
    }
}

/// The memory touched by a given access.
#[allow(dead_code)]
#[cfg_attr(target_os = "linux", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct AccessRange {
    /// The base address in memory where an access occurred.
    pub addr: usize,
    /// The number of bytes affected from the base.
    pub size: usize,
}

impl AccessRange {
    fn end(&self) -> usize {
        self.addr.strict_add(self.size)
    }
}

impl<'tcx> EvalContextExtPriv<'tcx> for crate::MiriInterpCx<'tcx> {}
trait EvalContextExtPriv<'tcx>: crate::MiriInterpCxExt<'tcx> {
    /// Call native host function and return the output as an immediate.
    fn call_native_with_args<'a>(
        &mut self,
        link_name: Symbol,
        dest: &MPlaceTy<'tcx>,
        ptr: CodePtr,
        libffi_args: Vec<libffi::high::Arg<'a>>,
    ) -> InterpResult<'tcx, (crate::ImmTy<'tcx>, Option<MemEvents>)> {
        let this = self.eval_context_mut();
        #[cfg(target_os = "linux")]
        let alloc = this.machine.allocator.as_ref().unwrap();
        #[cfg(not(target_os = "linux"))]
        // Placeholder value.
        let alloc = ();

        trace::Supervisor::do_ffi(alloc, || {
            // Call the function (`ptr`) with arguments `libffi_args`, and obtain the return value
            // as the specified primitive integer type
            let scalar = match dest.layout.ty.kind() {
                // ints
                ty::Int(IntTy::I8) => {
                    // Unsafe because of the call to native code.
                    // Because this is calling a C function it is not necessarily sound,
                    // but there is no way around this and we've checked as much as we can.
                    let x = unsafe { ffi::call::<i8>(ptr, libffi_args.as_slice()) };
                    Scalar::from_i8(x)
                }
                ty::Int(IntTy::I16) => {
                    let x = unsafe { ffi::call::<i16>(ptr, libffi_args.as_slice()) };
                    Scalar::from_i16(x)
                }
                ty::Int(IntTy::I32) => {
                    let x = unsafe { ffi::call::<i32>(ptr, libffi_args.as_slice()) };
                    Scalar::from_i32(x)
                }
                ty::Int(IntTy::I64) => {
                    let x = unsafe { ffi::call::<i64>(ptr, libffi_args.as_slice()) };
                    Scalar::from_i64(x)
                }
                ty::Int(IntTy::Isize) => {
                    let x = unsafe { ffi::call::<isize>(ptr, libffi_args.as_slice()) };
                    Scalar::from_target_isize(x.try_into().unwrap(), this)
                }
                // uints
                ty::Uint(UintTy::U8) => {
                    let x = unsafe { ffi::call::<u8>(ptr, libffi_args.as_slice()) };
                    Scalar::from_u8(x)
                }
                ty::Uint(UintTy::U16) => {
                    let x = unsafe { ffi::call::<u16>(ptr, libffi_args.as_slice()) };
                    Scalar::from_u16(x)
                }
                ty::Uint(UintTy::U32) => {
                    let x = unsafe { ffi::call::<u32>(ptr, libffi_args.as_slice()) };
                    Scalar::from_u32(x)
                }
                ty::Uint(UintTy::U64) => {
                    let x = unsafe { ffi::call::<u64>(ptr, libffi_args.as_slice()) };
                    Scalar::from_u64(x)
                }
                ty::Uint(UintTy::Usize) => {
                    let x = unsafe { ffi::call::<usize>(ptr, libffi_args.as_slice()) };
                    Scalar::from_target_usize(x.try_into().unwrap(), this)
                }
                // Functions with no declared return type (i.e., the default return)
                // have the output_type `Tuple([])`.
                ty::Tuple(t_list) if (*t_list).deref().is_empty() => {
                    unsafe { ffi::call::<()>(ptr, libffi_args.as_slice()) };
                    return interp_ok(ImmTy::uninit(dest.layout));
                }
                ty::RawPtr(..) => {
                    let x = unsafe { ffi::call::<*const ()>(ptr, libffi_args.as_slice()) };
                    let ptr = Pointer::new(Provenance::Wildcard, Size::from_bytes(x.addr()));
                    Scalar::from_pointer(ptr, this)
                }
                _ =>
                    return Err(err_unsup_format!(
                        "unsupported return type for native call: {:?}",
                        link_name
                    ))
                    .into(),
            };
            interp_ok(ImmTy::from_scalar(scalar, dest.layout))
        })
    }

    /// Get the pointer to the function of the specified name in the shared object file,
    /// if it exists. The function must be in one of the shared object files specified:
    /// we do *not* return pointers to functions in dependencies of libraries.
    fn get_func_ptr_explicitly_from_lib(&mut self, link_name: Symbol) -> Option<CodePtr> {
        let this = self.eval_context_mut();
        // Try getting the function from one of the shared libraries.
        for (lib, lib_path) in &this.machine.native_lib {
            let Ok(func): Result<libloading::Symbol<'_, unsafe extern "C" fn()>, _> =
                (unsafe { lib.get(link_name.as_str().as_bytes()) })
            else {
                continue;
            };
            #[expect(clippy::as_conversions)] // fn-ptr to raw-ptr cast needs `as`.
            let fn_ptr = *func.deref() as *mut std::ffi::c_void;

            // FIXME: this is a hack!
            // The `libloading` crate will automatically load system libraries like `libc`.
            // On linux `libloading` is based on `dlsym`: https://docs.rs/libloading/0.7.3/src/libloading/os/unix/mod.rs.html#202
            // and `dlsym`(https://linux.die.net/man/3/dlsym) looks through the dependency tree of the
            // library if it can't find the symbol in the library itself.
            // So, in order to check if the function was actually found in the specified
            // `machine.external_so_lib` we need to check its `dli_fname` and compare it to
            // the specified SO file path.
            // This code is a reimplementation of the mechanism for getting `dli_fname` in `libloading`,
            // from: https://docs.rs/libloading/0.7.3/src/libloading/os/unix/mod.rs.html#411
            // using the `libc` crate where this interface is public.
            let mut info = std::mem::MaybeUninit::<libc::Dl_info>::zeroed();
            unsafe {
                let res = libc::dladdr(fn_ptr, info.as_mut_ptr());
                assert!(res != 0, "failed to load info about function we already loaded");
                let info = info.assume_init();
                #[cfg(target_os = "cygwin")]
                let fname_ptr = info.dli_fname.as_ptr();
                #[cfg(not(target_os = "cygwin"))]
                let fname_ptr = info.dli_fname;
                assert!(!fname_ptr.is_null());
                if std::ffi::CStr::from_ptr(fname_ptr).to_str().unwrap()
                    != lib_path.to_str().unwrap()
                {
                    // The function is not actually in this .so, check the next one.
                    continue;
                }
            }

            // Return a pointer to the function.
            return Some(CodePtr(fn_ptr));
        }
        None
    }

    /// Applies the `events` to Miri's internal state. The event vector must be
    /// ordered sequentially by when the accesses happened, and the sizes are
    /// assumed to be exact.
    fn tracing_apply_accesses(&mut self, events: MemEvents) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();

        for evt in events.acc_events {
            let evt_rg = evt.get_range();
            // LLVM at least permits vectorising accesses to adjacent allocations,
            // so we cannot assume 1 access = 1 allocation. :(
            let mut rg = evt_rg.addr..evt_rg.end();
            while let Some(curr) = rg.next() {
                let Some(alloc_id) = this.alloc_id_from_addr(
                    curr.to_u64(),
                    rg.len().try_into().unwrap(),
                    /* only_exposed_allocations */ true,
                ) else {
                    throw_ub_format!("Foreign code did an out-of-bounds access!")
                };
                let alloc = this.get_alloc_raw(alloc_id)?;
                // The logical and physical address of the allocation coincide, so we can use
                // this instead of `addr_from_alloc_id`.
                let alloc_addr = alloc.get_bytes_unchecked_raw().addr();

                // Determine the range inside the allocation that this access covers. This range is
                // in terms of offsets from the start of `alloc`. The start of the overlap range
                // will be `curr`; the end will be the minimum of the end of the allocation and the
                // end of the access' range.
                let overlap = curr.strict_sub(alloc_addr)
                    ..std::cmp::min(alloc.len(), rg.end.strict_sub(alloc_addr));
                // Skip forward however many bytes of the access are contained in the current
                // allocation, subtracting 1 since the overlap range includes the current addr
                // that was already popped off of the range.
                rg.advance_by(overlap.len().strict_sub(1)).unwrap();

                match evt {
                    AccessEvent::Read(_) => {
                        // FIXME: ProvenanceMap should have something like get_range().
                        let p_map = alloc.provenance();
                        for idx in overlap {
                            // If a provenance was read by the foreign code, expose it.
                            if let Some(prov) = p_map.get(Size::from_bytes(idx), this) {
                                this.expose_provenance(prov)?;
                            }
                        }
                    }
                    AccessEvent::Write(_, certain) => {
                        // Sometimes we aren't certain if a write happened, in which case we
                        // only initialise that data if the allocation is mutable.
                        if certain || alloc.mutability.is_mut() {
                            let (alloc, cx) = this.get_alloc_raw_mut(alloc_id)?;
                            alloc.process_native_write(
                                &cx.tcx,
                                Some(AllocRange {
                                    start: Size::from_bytes(overlap.start),
                                    size: Size::from_bytes(overlap.len()),
                                }),
                            )
                        }
                    }
                }
            }
        }

        interp_ok(())
    }
}

impl<'tcx> EvalContextExt<'tcx> for crate::MiriInterpCx<'tcx> {}
pub trait EvalContextExt<'tcx>: crate::MiriInterpCxExt<'tcx> {
    /// Call the native host function, with supplied arguments.
    /// Needs to convert all the arguments from their Miri representations to
    /// a native form (through `libffi` call).
    /// Then, convert the return value from the native form into something that
    /// can be stored in Miri's internal memory.
    fn call_native_fn(
        &mut self,
        link_name: Symbol,
        dest: &MPlaceTy<'tcx>,
        args: &[OpTy<'tcx>],
    ) -> InterpResult<'tcx, bool> {
        let this = self.eval_context_mut();
        // Get the pointer to the function in the shared object file if it exists.
        let code_ptr = match this.get_func_ptr_explicitly_from_lib(link_name) {
            Some(ptr) => ptr,
            None => {
                // Shared object file does not export this function -- try the shims next.
                return interp_ok(false);
            }
        };

        // Do we have ptrace?
        let tracing = trace::Supervisor::is_enabled();

        // Get the function arguments, and convert them to `libffi`-compatible form.
        let mut libffi_args = Vec::<CArg>::with_capacity(args.len());
        for arg in args.iter() {
            if !matches!(arg.layout.backend_repr, BackendRepr::Scalar(_)) {
                throw_unsup_format!("only scalar argument types are supported for native calls")
            }
            let imm = this.read_immediate(arg)?;
            libffi_args.push(imm_to_carg(&imm, this)?);
            // If we are passing a pointer, expose its provenance. Below, all exposed memory
            // (previously exposed and new exposed) will then be properly prepared.
            if matches!(arg.layout.ty.kind(), ty::RawPtr(..)) {
                let ptr = imm.to_scalar().to_pointer(this)?;
                let Some(prov) = ptr.provenance else {
                    // Pointer without provenance may not access any memory anyway, skip.
                    continue;
                };
                // The first time this happens, print a warning.
                if !this.machine.native_call_mem_warned.replace(true) {
                    // Newly set, so first time we get here.
                    this.emit_diagnostic(NonHaltingDiagnostic::NativeCallSharedMem { tracing });
                }

                this.expose_provenance(prov)?;
            }
        }
        // Convert arguments to `libffi::high::Arg` type.
        let libffi_args = libffi_args
            .iter()
            .map(|arg| arg.arg_downcast())
            .collect::<Vec<libffi::high::Arg<'_>>>();

        // Prepare all exposed memory (both previously exposed, and just newly exposed since a
        // pointer was passed as argument). Uninitialised memory is left as-is, but any data
        // exposed this way is garbage anyway.
        this.visit_reachable_allocs(this.exposed_allocs(), |this, alloc_id, info| {
            // If there is no data behind this pointer, skip this.
            if !matches!(info.kind, AllocKind::LiveData) {
                return interp_ok(());
            }
            // It's okay to get raw access, what we do does not correspond to any actual
            // AM operation, it just approximates the state to account for the native call.
            let alloc = this.get_alloc_raw(alloc_id)?;
            // Also expose the provenance of the interpreter-level allocation, so it can
            // be read by FFI. The `black_box` is defensive programming as LLVM likes
            // to (incorrectly) optimize away ptr2int casts whose result is unused.
            std::hint::black_box(alloc.get_bytes_unchecked_raw().expose_provenance());

            if !tracing {
                // Expose all provenances in this allocation, since the native code can do $whatever.
                // Can be skipped when tracing; in that case we'll expose just the actually-read parts later.
                for prov in alloc.provenance().provenances() {
                    this.expose_provenance(prov)?;
                }
            }

            // Prepare for possible write from native code if mutable.
            if info.mutbl.is_mut() {
                let (alloc, cx) = this.get_alloc_raw_mut(alloc_id)?;
                // These writes could initialize everything and wreck havoc with the pointers.
                // We can skip that when tracing; in that case we'll later do that only for the memory that got actually written.
                if !tracing {
                    alloc.process_native_write(&cx.tcx, None);
                }
                // Also expose *mutable* provenance for the interpreter-level allocation.
                std::hint::black_box(alloc.get_bytes_unchecked_raw_mut().expose_provenance());
            }

            interp_ok(())
        })?;

        // Call the function and store output, depending on return type in the function signature.
        let (ret, maybe_memevents) =
            this.call_native_with_args(link_name, dest, code_ptr, libffi_args)?;

        if tracing {
            this.tracing_apply_accesses(maybe_memevents.unwrap())?;
        }

        this.write_immediate(*ret, dest)?;
        interp_ok(true)
    }
}

#[derive(Debug, Clone)]
/// Enum of supported arguments to external C functions.
// We introduce this enum instead of just calling `ffi::arg` and storing a list
// of `libffi::high::Arg` directly, because the `libffi::high::Arg` just wraps a reference
// to the value it represents: https://docs.rs/libffi/latest/libffi/high/call/struct.Arg.html
// and we need to store a copy of the value, and pass a reference to this copy to C instead.
enum CArg {
    /// 8-bit signed integer.
    Int8(i8),
    /// 16-bit signed integer.
    Int16(i16),
    /// 32-bit signed integer.
    Int32(i32),
    /// 64-bit signed integer.
    Int64(i64),
    /// isize.
    ISize(isize),
    /// 8-bit unsigned integer.
    UInt8(u8),
    /// 16-bit unsigned integer.
    UInt16(u16),
    /// 32-bit unsigned integer.
    UInt32(u32),
    /// 64-bit unsigned integer.
    UInt64(u64),
    /// usize.
    USize(usize),
    /// Raw pointer, stored as C's `void*`.
    RawPtr(*mut std::ffi::c_void),
}

impl<'a> CArg {
    /// Convert a `CArg` to a `libffi` argument type.
    fn arg_downcast(&'a self) -> libffi::high::Arg<'a> {
        match self {
            CArg::Int8(i) => ffi::arg(i),
            CArg::Int16(i) => ffi::arg(i),
            CArg::Int32(i) => ffi::arg(i),
            CArg::Int64(i) => ffi::arg(i),
            CArg::ISize(i) => ffi::arg(i),
            CArg::UInt8(i) => ffi::arg(i),
            CArg::UInt16(i) => ffi::arg(i),
            CArg::UInt32(i) => ffi::arg(i),
            CArg::UInt64(i) => ffi::arg(i),
            CArg::USize(i) => ffi::arg(i),
            CArg::RawPtr(i) => ffi::arg(i),
        }
    }
}

/// Extract the scalar value from the result of reading a scalar from the machine,
/// and convert it to a `CArg`.
fn imm_to_carg<'tcx>(v: &ImmTy<'tcx>, cx: &impl HasDataLayout) -> InterpResult<'tcx, CArg> {
    interp_ok(match v.layout.ty.kind() {
        // If the primitive provided can be converted to a type matching the type pattern
        // then create a `CArg` of this primitive value with the corresponding `CArg` constructor.
        // the ints
        ty::Int(IntTy::I8) => CArg::Int8(v.to_scalar().to_i8()?),
        ty::Int(IntTy::I16) => CArg::Int16(v.to_scalar().to_i16()?),
        ty::Int(IntTy::I32) => CArg::Int32(v.to_scalar().to_i32()?),
        ty::Int(IntTy::I64) => CArg::Int64(v.to_scalar().to_i64()?),
        ty::Int(IntTy::Isize) =>
            CArg::ISize(v.to_scalar().to_target_isize(cx)?.try_into().unwrap()),
        // the uints
        ty::Uint(UintTy::U8) => CArg::UInt8(v.to_scalar().to_u8()?),
        ty::Uint(UintTy::U16) => CArg::UInt16(v.to_scalar().to_u16()?),
        ty::Uint(UintTy::U32) => CArg::UInt32(v.to_scalar().to_u32()?),
        ty::Uint(UintTy::U64) => CArg::UInt64(v.to_scalar().to_u64()?),
        ty::Uint(UintTy::Usize) =>
            CArg::USize(v.to_scalar().to_target_usize(cx)?.try_into().unwrap()),
        ty::RawPtr(..) => {
            let s = v.to_scalar().to_pointer(cx)?.addr();
            // This relies on the `expose_provenance` in the `visit_reachable_allocs` callback
            // above.
            CArg::RawPtr(std::ptr::with_exposed_provenance_mut(s.bytes_usize()))
        }
        _ => throw_unsup_format!("unsupported argument type for native call: {}", v.layout.ty),
    })
}
