use rustc_middle::bug;
use rustc_middle::mir::visit::*;
use rustc_middle::mir::*;
use rustc_middle::ty::{self, ParamEnv, Ty, TyCtxt};

const INSTR_COST: usize = 5;
const CALL_PENALTY: usize = 25;
const LANDINGPAD_PENALTY: usize = 50;
const RESUME_PENALTY: usize = 45;
const LARGE_SWITCH_PENALTY: usize = 20;
const CONST_SWITCH_BONUS: usize = 10;

/// Verify that the callee body is compatible with the caller.
#[derive(Clone)]
pub(crate) struct CostChecker<'b, 'tcx> {
    tcx: TyCtxt<'tcx>,
    param_env: ParamEnv<'tcx>,
    penalty: usize,
    bonus: usize,
    callee_body: &'b Body<'tcx>,
    instance: Option<ty::Instance<'tcx>>,
}

impl<'b, 'tcx> CostChecker<'b, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        param_env: ParamEnv<'tcx>,
        instance: Option<ty::Instance<'tcx>>,
        callee_body: &'b Body<'tcx>,
    ) -> CostChecker<'b, 'tcx> {
        CostChecker { tcx, param_env, callee_body, instance, penalty: 0, bonus: 0 }
    }

    pub fn cost(&self) -> usize {
        usize::saturating_sub(self.penalty, self.bonus)
    }

    fn instantiate_ty(&self, v: Ty<'tcx>) -> Ty<'tcx> {
        if let Some(instance) = self.instance {
            instance.instantiate_mir(self.tcx, ty::EarlyBinder::bind(&v))
        } else {
            v
        }
    }
}

impl<'tcx> Visitor<'tcx> for CostChecker<'_, 'tcx> {
    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        // Most costs are in rvalues and terminators, not in statements.
        match statement.kind {
            StatementKind::Intrinsic(ref ndi) => {
                self.penalty += match **ndi {
                    NonDivergingIntrinsic::Assume(..) => INSTR_COST,
                    NonDivergingIntrinsic::CopyNonOverlapping(..) => CALL_PENALTY,
                };
            }
            _ => self.super_statement(statement, location),
        }
    }

    fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, _location: Location) {
        match rvalue {
            Rvalue::NullaryOp(NullOp::UbChecks, ..) if !self.tcx.sess.ub_checks() => {
                // If this is in optimized MIR it's because it's used later,
                // so if we don't need UB checks this session, give a bonus
                // here to offset the cost of the call later.
                self.bonus += CALL_PENALTY;
            }
            // These are essentially constants that didn't end up in an Operand,
            // so treat them as also being free.
            Rvalue::NullaryOp(..) => {}
            _ => self.penalty += INSTR_COST,
        }
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _: Location) {
        match &terminator.kind {
            TerminatorKind::Drop { place, unwind, .. } => {
                // If the place doesn't actually need dropping, treat it like a regular goto.
                let ty = self.instantiate_ty(place.ty(self.callee_body, self.tcx).ty);
                if ty.needs_drop(self.tcx, self.param_env) {
                    self.penalty += CALL_PENALTY;
                    if let UnwindAction::Cleanup(_) = unwind {
                        self.penalty += LANDINGPAD_PENALTY;
                    }
                }
            }
            TerminatorKind::Call { func, unwind, .. } => {
                self.penalty += if let Some((def_id, ..)) = func.const_fn_def()
                    && self.tcx.intrinsic(def_id).is_some()
                {
                    // Don't give intrinsics the extra penalty for calls
                    INSTR_COST
                } else {
                    CALL_PENALTY
                };
                if let UnwindAction::Cleanup(_) = unwind {
                    self.penalty += LANDINGPAD_PENALTY;
                }
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                if discr.constant().is_some() {
                    // Not only will this become a `Goto`, but likely other
                    // things will be removable as unreachable.
                    self.bonus += CONST_SWITCH_BONUS;
                } else if targets.all_targets().len() > 3 {
                    // More than false/true/unreachable gets extra cost.
                    self.penalty += LARGE_SWITCH_PENALTY;
                } else {
                    self.penalty += INSTR_COST;
                }
            }
            TerminatorKind::Assert { unwind, msg, .. } => {
                self.penalty +=
                    if msg.is_optional_overflow_check() && !self.tcx.sess.overflow_checks() {
                        INSTR_COST
                    } else {
                        CALL_PENALTY
                    };
                if let UnwindAction::Cleanup(_) = unwind {
                    self.penalty += LANDINGPAD_PENALTY;
                }
            }
            TerminatorKind::UnwindResume => self.penalty += RESUME_PENALTY,
            TerminatorKind::InlineAsm { unwind, .. } => {
                self.penalty += INSTR_COST;
                if let UnwindAction::Cleanup(_) = unwind {
                    self.penalty += LANDINGPAD_PENALTY;
                }
            }
            TerminatorKind::Unreachable => {
                self.bonus += INSTR_COST;
            }
            TerminatorKind::Goto { .. } | TerminatorKind::Return => {}
            TerminatorKind::UnwindTerminate(..) => {}
            kind @ (TerminatorKind::FalseUnwind { .. }
            | TerminatorKind::FalseEdge { .. }
            | TerminatorKind::Yield { .. }
            | TerminatorKind::CoroutineDrop) => {
                bug!("{kind:?} should not be in runtime MIR");
            }
        }
    }
}
