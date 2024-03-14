use rustc_index::bit_set::BitSet;
use rustc_middle::mir::visit::{NonMutatingUseContext, PlaceContext, Visitor};
use rustc_middle::mir::*;

use std::borrow::Cow;

use super::MaybeBorrowedLocals;
use crate::{GenKill, ResultsCursor};

#[derive(Clone)]
pub struct MaybeStorageLive<'a> {
    always_live_locals: Cow<'a, BitSet<Local>>,
}

impl<'a> MaybeStorageLive<'a> {
    pub fn new(always_live_locals: Cow<'a, BitSet<Local>>) -> Self {
        MaybeStorageLive { always_live_locals }
    }
}

impl<'tcx, 'a> crate::AnalysisDomain<'tcx> for MaybeStorageLive<'a> {
    type Domain = BitSet<Local>;

    const NAME: &'static str = "maybe_storage_live";

    fn bottom_value(&self, body: &Body<'tcx>) -> Self::Domain {
        // bottom = dead
        BitSet::new_empty(body.local_decls.len())
    }

    fn initialize_start_block(&self, body: &Body<'tcx>, on_entry: &mut Self::Domain) {
        assert_eq!(body.local_decls.len(), self.always_live_locals.domain_size());
        for local in self.always_live_locals.iter() {
            on_entry.insert(local);
        }

        for arg in body.args_iter() {
            on_entry.insert(arg);
        }
    }
}

impl<'tcx, 'a> crate::GenKillAnalysis<'tcx> for MaybeStorageLive<'a> {
    type Idx = Local;

    fn domain_size(&self, body: &Body<'tcx>) -> usize {
        body.local_decls.len()
    }

    fn statement_effect(
        &mut self,
        trans: &mut impl GenKill<Self::Idx>,
        stmt: &Statement<'tcx>,
        _: Location,
    ) {
        match stmt.kind {
            StatementKind::StorageLive(l) => trans.gen(l),
            StatementKind::StorageDead(l) => trans.kill(l),
            _ => (),
        }
    }

    fn terminator_effect<'mir>(
        &mut self,
        _trans: &mut Self::Domain,
        terminator: &'mir Terminator<'tcx>,
        _: Location,
    ) -> TerminatorEdges<'mir, 'tcx> {
        // Terminators have no effect
        terminator.edges()
    }

    fn call_return_effect(
        &mut self,
        _trans: &mut Self::Domain,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
        // Nothing to do when a call returns successfully
    }
}

#[derive(Clone)]
pub struct MaybeStorageDead<'a> {
    always_live_locals: Cow<'a, BitSet<Local>>,
}

impl<'a> MaybeStorageDead<'a> {
    pub fn new(always_live_locals: Cow<'a, BitSet<Local>>) -> Self {
        MaybeStorageDead { always_live_locals }
    }
}

impl<'tcx, 'a> crate::AnalysisDomain<'tcx> for MaybeStorageDead<'a> {
    type Domain = BitSet<Local>;

    const NAME: &'static str = "maybe_storage_dead";

    fn bottom_value(&self, body: &Body<'tcx>) -> Self::Domain {
        // bottom = live
        BitSet::new_empty(body.local_decls.len())
    }

    fn initialize_start_block(&self, body: &Body<'tcx>, on_entry: &mut Self::Domain) {
        assert_eq!(body.local_decls.len(), self.always_live_locals.domain_size());
        // Do not iterate on return place and args, as they are trivially always live.
        for local in body.vars_and_temps_iter() {
            if !self.always_live_locals.contains(local) {
                on_entry.insert(local);
            }
        }
    }
}

impl<'tcx, 'a> crate::GenKillAnalysis<'tcx> for MaybeStorageDead<'a> {
    type Idx = Local;

    fn domain_size(&self, body: &Body<'tcx>) -> usize {
        body.local_decls.len()
    }

    fn statement_effect(
        &mut self,
        trans: &mut impl GenKill<Self::Idx>,
        stmt: &Statement<'tcx>,
        _: Location,
    ) {
        match stmt.kind {
            StatementKind::StorageLive(l) => trans.kill(l),
            StatementKind::StorageDead(l) => trans.gen(l),
            _ => (),
        }
    }

    fn terminator_effect<'mir>(
        &mut self,
        _: &mut Self::Domain,
        terminator: &'mir Terminator<'tcx>,
        _: Location,
    ) -> TerminatorEdges<'mir, 'tcx> {
        // Terminators have no effect
        terminator.edges()
    }

    fn call_return_effect(
        &mut self,
        _trans: &mut Self::Domain,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
        // Nothing to do when a call returns successfully
    }
}

type BorrowedLocalsResults<'mir, 'tcx> = ResultsCursor<'mir, 'tcx, MaybeBorrowedLocals>;

/// Dataflow analysis that determines whether each local requires storage at a
/// given location; i.e. whether its storage can go away without being observed.
pub struct MaybeRequiresStorage<'mir, 'tcx> {
    borrowed_locals: BorrowedLocalsResults<'mir, 'tcx>,
}

impl<'mir, 'tcx> MaybeRequiresStorage<'mir, 'tcx> {
    pub fn new(borrowed_locals: BorrowedLocalsResults<'mir, 'tcx>) -> Self {
        MaybeRequiresStorage { borrowed_locals }
    }
}

impl<'tcx> crate::AnalysisDomain<'tcx> for MaybeRequiresStorage<'_, 'tcx> {
    type Domain = BitSet<Local>;

    const NAME: &'static str = "requires_storage";

    fn bottom_value(&self, body: &Body<'tcx>) -> Self::Domain {
        // bottom = dead
        BitSet::new_empty(body.local_decls.len())
    }

    fn initialize_start_block(&self, body: &Body<'tcx>, on_entry: &mut Self::Domain) {
        // The resume argument is live on function entry (we don't care about
        // the `self` argument)
        for arg in body.args_iter().skip(1) {
            on_entry.insert(arg);
        }
    }
}

impl<'tcx> crate::GenKillAnalysis<'tcx> for MaybeRequiresStorage<'_, 'tcx> {
    type Idx = Local;

    fn domain_size(&self, body: &Body<'tcx>) -> usize {
        body.local_decls.len()
    }

    fn before_statement_effect(
        &mut self,
        trans: &mut impl GenKill<Self::Idx>,
        stmt: &Statement<'tcx>,
        loc: Location,
    ) {
        // If a place is borrowed in a statement, it needs storage for that statement.
        self.borrowed_locals.mut_analysis().statement_effect(trans, stmt, loc);

        match &stmt.kind {
            StatementKind::StorageDead(l) => trans.kill(*l),

            // If a place is assigned to in a statement, it needs storage for that statement.
            StatementKind::Assign(box (place, _))
            | StatementKind::SetDiscriminant { box place, .. }
            | StatementKind::Deinit(box place) => {
                trans.gen(place.local);
            }

            // Nothing to do for these. Match exhaustively so this fails to compile when new
            // variants are added.
            StatementKind::AscribeUserType(..)
            | StatementKind::PlaceMention(..)
            | StatementKind::Coverage(..)
            | StatementKind::FakeRead(..)
            | StatementKind::ConstEvalCounter
            | StatementKind::Nop
            | StatementKind::Retag(..)
            | StatementKind::Intrinsic(..)
            | StatementKind::StorageLive(..) => {}
        }
    }

    fn statement_effect(
        &mut self,
        trans: &mut impl GenKill<Self::Idx>,
        _: &Statement<'tcx>,
        loc: Location,
    ) {
        // If we move from a place then it only stops needing storage *after*
        // that statement.
        self.check_for_move(trans, loc);
    }

    fn before_terminator_effect(
        &mut self,
        trans: &mut Self::Domain,
        terminator: &Terminator<'tcx>,
        loc: Location,
    ) {
        // If a place is borrowed in a terminator, it needs storage for that terminator.
        self.borrowed_locals
            .mut_analysis()
            .transfer_function(trans)
            .visit_terminator(terminator, loc);

        match &terminator.kind {
            TerminatorKind::Call { destination, .. } => {
                trans.gen(destination.local);
            }

            // Note that we do *not* gen the `resume_arg` of `Yield` terminators. The reason for
            // that is that a `yield` will return from the function, and `resume_arg` is written
            // only when the coroutine is later resumed. Unlike `Call`, this doesn't require the
            // place to have storage *before* the yield, only after.
            TerminatorKind::Yield { .. } => {}

            TerminatorKind::InlineAsm { operands, .. } => {
                for op in operands {
                    match op {
                        InlineAsmOperand::Out { place, .. }
                        | InlineAsmOperand::InOut { out_place: place, .. } => {
                            if let Some(place) = place {
                                trans.gen(place.local);
                            }
                        }
                        InlineAsmOperand::In { .. }
                        | InlineAsmOperand::Const { .. }
                        | InlineAsmOperand::SymFn { .. }
                        | InlineAsmOperand::SymStatic { .. }
                        | InlineAsmOperand::Label { .. } => {}
                    }
                }
            }

            // Nothing to do for these. Match exhaustively so this fails to compile when new
            // variants are added.
            TerminatorKind::UnwindTerminate(_)
            | TerminatorKind::Assert { .. }
            | TerminatorKind::Drop { .. }
            | TerminatorKind::FalseEdge { .. }
            | TerminatorKind::FalseUnwind { .. }
            | TerminatorKind::CoroutineDrop
            | TerminatorKind::Goto { .. }
            | TerminatorKind::UnwindResume
            | TerminatorKind::Return
            | TerminatorKind::SwitchInt { .. }
            | TerminatorKind::Unreachable => {}
        }
    }

    fn terminator_effect<'t>(
        &mut self,
        trans: &mut Self::Domain,
        terminator: &'t Terminator<'tcx>,
        loc: Location,
    ) -> TerminatorEdges<'t, 'tcx> {
        match terminator.kind {
            // For call terminators the destination requires storage for the call
            // and after the call returns successfully, but not after a panic.
            // Since `propagate_call_unwind` doesn't exist, we have to kill the
            // destination here, and then gen it again in `call_return_effect`.
            TerminatorKind::Call { destination, .. } => {
                trans.kill(destination.local);
            }

            // The same applies to InlineAsm outputs.
            TerminatorKind::InlineAsm { ref operands, .. } => {
                CallReturnPlaces::InlineAsm(operands).for_each(|place| trans.kill(place.local));
            }

            // Nothing to do for these. Match exhaustively so this fails to compile when new
            // variants are added.
            TerminatorKind::Yield { .. }
            | TerminatorKind::UnwindTerminate(_)
            | TerminatorKind::Assert { .. }
            | TerminatorKind::Drop { .. }
            | TerminatorKind::FalseEdge { .. }
            | TerminatorKind::FalseUnwind { .. }
            | TerminatorKind::CoroutineDrop
            | TerminatorKind::Goto { .. }
            | TerminatorKind::UnwindResume
            | TerminatorKind::Return
            | TerminatorKind::SwitchInt { .. }
            | TerminatorKind::Unreachable => {}
        }

        self.check_for_move(trans, loc);
        terminator.edges()
    }

    fn call_return_effect(
        &mut self,
        trans: &mut Self::Domain,
        _block: BasicBlock,
        return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
        return_places.for_each(|place| trans.gen(place.local));
    }
}

impl<'tcx> MaybeRequiresStorage<'_, 'tcx> {
    /// Kill locals that are fully moved and have not been borrowed.
    fn check_for_move(&mut self, trans: &mut impl GenKill<Local>, loc: Location) {
        let body = self.borrowed_locals.body();
        let mut visitor = MoveVisitor { trans, borrowed_locals: &mut self.borrowed_locals };
        visitor.visit_location(body, loc);
    }
}

struct MoveVisitor<'a, 'mir, 'tcx, T> {
    borrowed_locals: &'a mut BorrowedLocalsResults<'mir, 'tcx>,
    trans: &'a mut T,
}

impl<'tcx, T> Visitor<'tcx> for MoveVisitor<'_, '_, 'tcx, T>
where
    T: GenKill<Local>,
{
    fn visit_local(&mut self, local: Local, context: PlaceContext, loc: Location) {
        if PlaceContext::NonMutatingUse(NonMutatingUseContext::Move) == context {
            self.borrowed_locals.seek_before_primary_effect(loc);
            if !self.borrowed_locals.contains(local) {
                self.trans.kill(local);
            }
        }
    }
}
