//! This module specifies the type based interner for constants.
//!
//! After a const evaluation has computed a value, before we destroy the const evaluator's session
//! memory, we need to extract all memory allocations to the global memory pool so they stay around.
//!
//! In principle, this is not very complicated: we recursively walk the final value, follow all the
//! pointers, and move all reachable allocations to the global `tcx` memory. The only complication
//! is picking the right mutability: the outermost allocation generally has a clear mutability, but
//! what about the other allocations it points to that have also been created with this value? We
//! don't want to do guesswork here. The rules are: `static`, `const`, and promoted can only create
//! immutable allocations that way. `static mut` can be initialized with expressions like `&mut 42`,
//! so all inner allocations are marked mutable. Some of them could potentially be made immutable,
//! but that would require relying on type information, and given how many ways Rust has to lie
//! about type information, we want to avoid doing that.

use rustc_ast::Mutability;
use rustc_data_structures::fx::{FxHashSet, FxIndexMap};
use rustc_errors::ErrorGuaranteed;
use rustc_hir as hir;
use rustc_middle::mir::interpret::{CtfeProvenance, InterpResult};
use rustc_middle::ty::layout::TyAndLayout;
use rustc_session::lint;

use super::{AllocId, Allocation, InterpCx, MPlaceTy, Machine, MemoryKind, PlaceTy};
use crate::const_eval;
use crate::errors::{DanglingPtrInFinal, MutablePtrInFinal};

pub trait CompileTimeMachine<'mir, 'tcx: 'mir, T> = Machine<
        'mir,
        'tcx,
        MemoryKind = T,
        Provenance = CtfeProvenance,
        ExtraFnVal = !,
        FrameExtra = (),
        AllocExtra = (),
        MemoryMap = FxIndexMap<AllocId, (MemoryKind<T>, Allocation)>,
    >;

/// Intern an allocation. Returns `Err` if the allocation does not exist in the local memory.
///
/// `mutability` can be used to force immutable interning: if it is `Mutability::Not`, the
/// allocation is interned immutably; if it is `Mutability::Mut`, then the allocation *must be*
/// already mutable (as a sanity check).
///
/// `recursive_alloc` is called for all recursively encountered allocations.
fn intern_shallow<'rt, 'mir, 'tcx, T, M: CompileTimeMachine<'mir, 'tcx, T>>(
    ecx: &'rt mut InterpCx<'mir, 'tcx, M>,
    alloc_id: AllocId,
    mutability: Mutability,
    mut recursive_alloc: impl FnMut(&InterpCx<'mir, 'tcx, M>, CtfeProvenance),
) -> Result<(), ()> {
    trace!("intern_shallow {:?}", alloc_id);
    // remove allocation
    let Some((_kind, mut alloc)) = ecx.memory.alloc_map.remove(&alloc_id) else {
        return Err(());
    };
    // Set allocation mutability as appropriate. This is used by LLVM to put things into
    // read-only memory, and also by Miri when evaluating other globals that
    // access this one.
    match mutability {
        Mutability::Not => {
            alloc.mutability = Mutability::Not;
        }
        Mutability::Mut => {
            // This must be already mutable, we won't "un-freeze" allocations ever.
            assert_eq!(alloc.mutability, Mutability::Mut);
        }
    }
    // record child allocations
    for &(_, prov) in alloc.provenance().ptrs().iter() {
        recursive_alloc(ecx, prov);
    }
    // link the alloc id to the actual allocation
    let alloc = ecx.tcx.mk_const_alloc(alloc);
    ecx.tcx.set_alloc_id_memory(alloc_id, alloc);
    Ok(())
}

/// How a constant value should be interned.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum InternKind {
    /// The `mutability` of the static, ignoring the type which may have interior mutability.
    Static(hir::Mutability),
    /// A `const` item
    Constant,
    Promoted,
}

/// Intern `ret` and everything it references.
///
/// This *cannot raise an interpreter error*. Doing so is left to validation, which
/// tracks where in the value we are and thus can show much better error messages.
#[instrument(level = "debug", skip(ecx))]
pub fn intern_const_alloc_recursive<
    'mir,
    'tcx: 'mir,
    M: CompileTimeMachine<'mir, 'tcx, const_eval::MemoryKind>,
>(
    ecx: &mut InterpCx<'mir, 'tcx, M>,
    intern_kind: InternKind,
    ret: &MPlaceTy<'tcx>,
) -> Result<(), ErrorGuaranteed> {
    // We are interning recursively, and for mutability we are distinguishing the "root" allocation
    // that we are starting in, and all other allocations that we are encountering recursively.
    let (base_mutability, inner_mutability) = match intern_kind {
        InternKind::Constant | InternKind::Promoted => {
            // Completely immutable. Interning anything mutably here can only lead to unsoundness,
            // since all consts are conceptually independent values but share the same underlying
            // memory.
            (Mutability::Not, Mutability::Not)
        }
        InternKind::Static(Mutability::Not) => {
            (
                // Outermost allocation is mutable if `!Freeze`.
                if ret.layout.ty.is_freeze(*ecx.tcx, ecx.param_env) {
                    Mutability::Not
                } else {
                    Mutability::Mut
                },
                // Inner allocations are never mutable. They can only arise via the "tail
                // expression" / "outer scope" rule, and we treat them consistently with `const`.
                Mutability::Not,
            )
        }
        InternKind::Static(Mutability::Mut) => {
            // Just make everything mutable. We accept code like
            // `static mut X = &mut [42]`, so even inner allocations need to be mutable.
            (Mutability::Mut, Mutability::Mut)
        }
    };

    // Initialize recursive interning.
    let base_alloc_id = ret.ptr().provenance.unwrap().alloc_id();
    let mut todo = vec![(base_alloc_id, base_mutability)];
    // We need to distinguish "has just been interned" from "was already in `tcx`",
    // so we track this in a separate set.
    let mut just_interned = FxHashSet::default();
    // Whether we encountered a bad mutable pointer.
    // We want to first report "dangling" and then "mutable", so we need to delay reporting these
    // errors.
    let mut found_bad_mutable_pointer = false;

    // Keep interning as long as there are things to intern.
    // We show errors if there are dangling pointers, or mutable pointers in immutable contexts
    // (i.e., everything except for `static mut`). When these errors affect references, it is
    // unfortunate that we show these errors here and not during validation, since validation can
    // show much nicer errors. However, we do need these checks to be run on all pointers, including
    // raw pointers, so we cannot rely on validation to catch them -- and since interning runs
    // before validation, and interning doesn't know the type of anything, this means we can't show
    // better errors. Maybe we should consider doing validation before interning in the future.
    while let Some((alloc_id, mutability)) = todo.pop() {
        if ecx.tcx.try_get_global_alloc(alloc_id).is_some() {
            // Already interned.
            debug_assert!(!ecx.memory.alloc_map.contains_key(&alloc_id));
            continue;
        }
        just_interned.insert(alloc_id);
        intern_shallow(ecx, alloc_id, mutability, |ecx, prov| {
            let alloc_id = prov.alloc_id();
            if intern_kind != InternKind::Promoted
                && inner_mutability == Mutability::Not
                && !prov.immutable()
            {
                if ecx.tcx.try_get_global_alloc(alloc_id).is_some()
                    && !just_interned.contains(&alloc_id)
                {
                    // This is a pointer to some memory from another constant. We encounter mutable
                    // pointers to such memory since we do not always track immutability through
                    // these "global" pointers. Allowing them is harmless; the point of these checks
                    // during interning is to justify why we intern the *new* allocations immutably,
                    // so we can completely ignore existing allocations. We also don't need to add
                    // this to the todo list, since after all it is already interned.
                    return;
                }
                // Found a mutable pointer inside a const where inner allocations should be
                // immutable. We exclude promoteds from this, since things like `&mut []` and
                // `&None::<Cell<i32>>` lead to promotion that can produce mutable pointers. We rely
                // on the promotion analysis not screwing up to ensure that it is sound to intern
                // promoteds as immutable.
                found_bad_mutable_pointer = true;
            }
            // We always intern with `inner_mutability`, and furthermore we ensured above that if
            // that is "immutable", then there are *no* mutable pointers anywhere in the newly
            // interned memory -- justifying that we can indeed intern immutably. However this also
            // means we can *not* easily intern immutably here if `prov.immutable()` is true and
            // `inner_mutability` is `Mut`: there might be other pointers to that allocation, and
            // we'd have to somehow check that they are *all* immutable before deciding that this
            // allocation can be made immutable. In the future we could consider analyzing all
            // pointers before deciding which allocations can be made immutable; but for now we are
            // okay with losing some potential for immutability here. This can anyway only affect
            // `static mut`.
            todo.push((alloc_id, inner_mutability));
        })
        .map_err(|()| {
            ecx.tcx.dcx().emit_err(DanglingPtrInFinal { span: ecx.tcx.span, kind: intern_kind })
        })?;
    }
    if found_bad_mutable_pointer {
        let err_diag = MutablePtrInFinal { span: ecx.tcx.span, kind: intern_kind };
        ecx.tcx.emit_node_span_lint(
            lint::builtin::CONST_EVAL_MUTABLE_PTR_IN_FINAL_VALUE,
            ecx.best_lint_scope(),
            err_diag.span,
            err_diag,
        )
    }

    Ok(())
}

/// Intern `ret`. This function assumes that `ret` references no other allocation.
#[instrument(level = "debug", skip(ecx))]
pub fn intern_const_alloc_for_constprop<
    'mir,
    'tcx: 'mir,
    T,
    M: CompileTimeMachine<'mir, 'tcx, T>,
>(
    ecx: &mut InterpCx<'mir, 'tcx, M>,
    alloc_id: AllocId,
) -> InterpResult<'tcx, ()> {
    if ecx.tcx.try_get_global_alloc(alloc_id).is_some() {
        // The constant is already in global memory. Do nothing.
        return Ok(());
    }
    // Move allocation to `tcx`.
    intern_shallow(ecx, alloc_id, Mutability::Not, |_ecx, _| {
        // We are not doing recursive interning, so we don't currently support provenance.
        // (If this assertion ever triggers, we should just implement a
        // proper recursive interning loop -- or just call `intern_const_alloc_recursive`.
        panic!("`intern_const_alloc_for_constprop` called on allocation with nested provenance")
    })
    .map_err(|()| err_ub!(DeadLocal).into())
}

impl<'mir, 'tcx: 'mir, M: super::intern::CompileTimeMachine<'mir, 'tcx, !>>
    InterpCx<'mir, 'tcx, M>
{
    /// A helper function that allocates memory for the layout given and gives you access to mutate
    /// it. Once your own mutation code is done, the backing `Allocation` is removed from the
    /// current `Memory` and interned as read-only into the global memory.
    pub fn intern_with_temp_alloc(
        &mut self,
        layout: TyAndLayout<'tcx>,
        f: impl FnOnce(
            &mut InterpCx<'mir, 'tcx, M>,
            &PlaceTy<'tcx, M::Provenance>,
        ) -> InterpResult<'tcx, ()>,
    ) -> InterpResult<'tcx, AllocId> {
        // `allocate` picks a fresh AllocId that we will associate with its data below.
        let dest = self.allocate(layout, MemoryKind::Stack)?;
        f(self, &dest.clone().into())?;
        let alloc_id = dest.ptr().provenance.unwrap().alloc_id(); // this was just allocated, it must have provenance
        intern_shallow(self, alloc_id, Mutability::Not, |ecx, prov| {
            // We are not doing recursive interning, so we don't currently support provenance.
            // (If this assertion ever triggers, we should just implement a
            // proper recursive interning loop -- or just call `intern_const_alloc_recursive`.
            if !ecx.tcx.try_get_global_alloc(prov.alloc_id()).is_some() {
                panic!("`intern_with_temp_alloc` with nested allocations");
            }
        })
        .unwrap();
        Ok(alloc_id)
    }
}
