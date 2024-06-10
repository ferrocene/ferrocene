use either::Either;

use rustc_data_structures::fx::FxHashSet;

use crate::*;

pub type VisitWith<'a> = dyn FnMut(Option<AllocId>, Option<BorTag>) + 'a;

pub trait VisitProvenance {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>);
}

// Trivial impls for types that do not contain any provenance
macro_rules! no_provenance {
    ($($ty:ident)+) => {
        $(
            impl VisitProvenance for $ty {
                fn visit_provenance(&self, _visit: &mut VisitWith<'_>) {}
            }
        )+
    }
}
no_provenance!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize ThreadId);

impl<T: VisitProvenance> VisitProvenance for Option<T> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        if let Some(x) = self {
            x.visit_provenance(visit);
        }
    }
}

impl<T: VisitProvenance> VisitProvenance for std::cell::RefCell<T> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        self.borrow().visit_provenance(visit)
    }
}

impl VisitProvenance for BorTag {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        visit(None, Some(*self))
    }
}

impl VisitProvenance for AllocId {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        visit(Some(*self), None)
    }
}

impl VisitProvenance for Provenance {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        if let Provenance::Concrete { alloc_id, tag, .. } = self {
            visit(Some(*alloc_id), Some(*tag));
        }
    }
}

impl VisitProvenance for StrictPointer {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        let (prov, _offset) = self.into_parts();
        prov.visit_provenance(visit);
    }
}

impl VisitProvenance for Pointer {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        let (prov, _offset) = self.into_parts();
        prov.visit_provenance(visit);
    }
}

impl VisitProvenance for Scalar {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        match self {
            Scalar::Ptr(ptr, _) => ptr.visit_provenance(visit),
            Scalar::Int(_) => (),
        }
    }
}

impl VisitProvenance for Immediate<Provenance> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        match self {
            Immediate::Scalar(s) => {
                s.visit_provenance(visit);
            }
            Immediate::ScalarPair(s1, s2) => {
                s1.visit_provenance(visit);
                s2.visit_provenance(visit);
            }
            Immediate::Uninit => {}
        }
    }
}

impl VisitProvenance for MemPlaceMeta<Provenance> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        match self {
            MemPlaceMeta::Meta(m) => m.visit_provenance(visit),
            MemPlaceMeta::None => {}
        }
    }
}

impl VisitProvenance for ImmTy<'_> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        (**self).visit_provenance(visit)
    }
}

impl VisitProvenance for MPlaceTy<'_> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        self.ptr().visit_provenance(visit);
        self.meta().visit_provenance(visit);
    }
}

impl VisitProvenance for PlaceTy<'_> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        match self.as_mplace_or_local() {
            Either::Left(mplace) => mplace.visit_provenance(visit),
            Either::Right(_) => (),
        }
    }
}

impl VisitProvenance for OpTy<'_> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        match self.as_mplace_or_imm() {
            Either::Left(mplace) => mplace.visit_provenance(visit),
            Either::Right(imm) => imm.visit_provenance(visit),
        }
    }
}

impl VisitProvenance for Allocation<Provenance, AllocExtra<'_>, MiriAllocBytes> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        for prov in self.provenance().provenances() {
            prov.visit_provenance(visit);
        }

        self.extra.visit_provenance(visit);
    }
}

impl VisitProvenance for crate::MiriInterpCx<'_> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        // Visit the contents of the allocations and the IDs themselves, to account for all
        // live allocation IDs and all provenance in the allocation bytes, even if they are leaked.
        // We do *not* visit all the `AllocId` of the live allocations; we tried that and adding
        // them all to the live set is too expensive. Instead we later do liveness check by
        // checking both "is this alloc id live" and "is it mentioned anywhere else in
        // the interpreter state".
        self.memory.alloc_map().iter(|it| {
            for (_id, (_kind, alloc)) in it {
                alloc.visit_provenance(visit);
            }
        });
        // And all the other machine values.
        self.machine.visit_provenance(visit);
    }
}

pub struct LiveAllocs<'a, 'tcx> {
    collected: FxHashSet<AllocId>,
    ecx: &'a MiriInterpCx<'tcx>,
}

impl LiveAllocs<'_, '_> {
    pub fn is_live(&self, id: AllocId) -> bool {
        self.collected.contains(&id) || self.ecx.is_alloc_live(id)
    }
}

impl<'tcx> EvalContextExt<'tcx> for crate::MiriInterpCx<'tcx> {}
pub trait EvalContextExt<'tcx>: MiriInterpCxExt<'tcx> {
    fn run_provenance_gc(&mut self) {
        // We collect all tags from various parts of the interpreter, but also
        let this = self.eval_context_mut();

        let mut tags = FxHashSet::default();
        let mut alloc_ids = FxHashSet::default();
        this.visit_provenance(&mut |id, tag| {
            if let Some(id) = id {
                alloc_ids.insert(id);
            }
            if let Some(tag) = tag {
                tags.insert(tag);
            }
        });
        self.remove_unreachable_tags(tags);
        self.remove_unreachable_allocs(alloc_ids);
    }

    fn remove_unreachable_tags(&mut self, tags: FxHashSet<BorTag>) {
        let this = self.eval_context_mut();
        this.memory.alloc_map().iter(|it| {
            for (_id, (_kind, alloc)) in it {
                if let Some(bt) = &alloc.extra.borrow_tracker {
                    bt.remove_unreachable_tags(&tags);
                }
            }
        });
    }

    fn remove_unreachable_allocs(&mut self, allocs: FxHashSet<AllocId>) {
        let this = self.eval_context_mut();
        let allocs = LiveAllocs { ecx: this, collected: allocs };
        this.machine.allocation_spans.borrow_mut().retain(|id, _| allocs.is_live(*id));
        this.machine.symbolic_alignment.borrow_mut().retain(|id, _| allocs.is_live(*id));
        this.machine.alloc_addresses.borrow_mut().remove_unreachable_allocs(&allocs);
        if let Some(borrow_tracker) = &this.machine.borrow_tracker {
            borrow_tracker.borrow_mut().remove_unreachable_allocs(&allocs);
        }
        this.remove_unreachable_allocs(&allocs.collected);
    }
}
