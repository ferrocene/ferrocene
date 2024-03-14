//! Code related to match expressions. These are sufficiently complex to
//! warrant their own module and submodules. :) This main module includes the
//! high-level algorithm, the submodules contain the details.
//!
//! This also includes code for pattern bindings in `let` statements and
//! function parameters.

use crate::build::expr::as_place::PlaceBuilder;
use crate::build::scope::DropKind;
use crate::build::ForGuard::{self, OutsideGuard, RefWithinGuard};
use crate::build::{BlockAnd, BlockAndExtension, Builder};
use crate::build::{GuardFrame, GuardFrameLocal, LocalsForNode};
use rustc_data_structures::{
    fx::{FxHashSet, FxIndexMap, FxIndexSet},
    stack::ensure_sufficient_stack,
};
use rustc_index::bit_set::BitSet;
use rustc_middle::middle::region;
use rustc_middle::mir::{self, *};
use rustc_middle::thir::{self, *};
use rustc_middle::ty::{self, CanonicalUserTypeAnnotation, Ty};
use rustc_span::symbol::Symbol;
use rustc_span::{BytePos, Pos, Span};
use rustc_target::abi::VariantIdx;
// helper functions, broken out by category:
mod simplify;
mod test;
mod util;

use std::borrow::Borrow;
use std::mem;

/// Arguments to [`Builder::then_else_break_inner`] that are usually forwarded
/// to recursive invocations.
#[derive(Clone, Copy)]
struct ThenElseArgs {
    /// Used as the temp scope for lowering `expr`. If absent (for match guards),
    /// `self.local_scope()` is used.
    temp_scope_override: Option<region::Scope>,
    variable_source_info: SourceInfo,
    /// Forwarded to [`Builder::lower_let_expr`] when lowering [`ExprKind::Let`].
    /// When false (for match guards), `let` bindings won't be declared.
    declare_let_bindings: bool,
}

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// Lowers a condition in a way that ensures that variables bound in any let
    /// expressions are definitely initialized in the if body.
    ///
    /// If `declare_let_bindings` is false then variables created in `let`
    /// expressions will not be declared. This is for if let guards on arms with
    /// an or pattern, where the guard is lowered multiple times.
    pub(crate) fn then_else_break(
        &mut self,
        block: BasicBlock,
        expr_id: ExprId,
        temp_scope_override: Option<region::Scope>,
        variable_source_info: SourceInfo,
        declare_let_bindings: bool,
    ) -> BlockAnd<()> {
        self.then_else_break_inner(
            block,
            expr_id,
            ThenElseArgs { temp_scope_override, variable_source_info, declare_let_bindings },
        )
    }

    fn then_else_break_inner(
        &mut self,
        block: BasicBlock, // Block that the condition and branch will be lowered into
        expr_id: ExprId,   // Condition expression to lower
        args: ThenElseArgs,
    ) -> BlockAnd<()> {
        let this = self;
        let expr = &this.thir[expr_id];
        let expr_span = expr.span;

        match expr.kind {
            ExprKind::LogicalOp { op: LogicalOp::And, lhs, rhs } => {
                let lhs_then_block = unpack!(this.then_else_break_inner(block, lhs, args));
                let rhs_then_block = unpack!(this.then_else_break_inner(lhs_then_block, rhs, args));
                rhs_then_block.unit()
            }
            ExprKind::LogicalOp { op: LogicalOp::Or, lhs, rhs } => {
                let local_scope = this.local_scope();
                let (lhs_success_block, failure_block) =
                    this.in_if_then_scope(local_scope, expr_span, |this| {
                        this.then_else_break_inner(
                            block,
                            lhs,
                            ThenElseArgs { declare_let_bindings: true, ..args },
                        )
                    });
                let rhs_success_block = unpack!(this.then_else_break_inner(
                    failure_block,
                    rhs,
                    ThenElseArgs { declare_let_bindings: true, ..args },
                ));

                // Make the LHS and RHS success arms converge to a common block.
                // (We can't just make LHS goto RHS, because `rhs_success_block`
                // might contain statements that we don't want on the LHS path.)
                let success_block = this.cfg.start_new_block();
                this.cfg.goto(lhs_success_block, args.variable_source_info, success_block);
                this.cfg.goto(rhs_success_block, args.variable_source_info, success_block);
                success_block.unit()
            }
            ExprKind::Unary { op: UnOp::Not, arg } => {
                let local_scope = this.local_scope();
                let (success_block, failure_block) =
                    this.in_if_then_scope(local_scope, expr_span, |this| {
                        // Help out coverage instrumentation by injecting a dummy statement with
                        // the original condition's span (including `!`). This fixes #115468.
                        if this.tcx.sess.instrument_coverage() {
                            this.cfg.push_coverage_span_marker(block, this.source_info(expr_span));
                        }
                        this.then_else_break_inner(
                            block,
                            arg,
                            ThenElseArgs { declare_let_bindings: true, ..args },
                        )
                    });
                this.break_for_else(success_block, args.variable_source_info);
                failure_block.unit()
            }
            ExprKind::Scope { region_scope, lint_level, value } => {
                let region_scope = (region_scope, this.source_info(expr_span));
                this.in_scope(region_scope, lint_level, |this| {
                    this.then_else_break_inner(block, value, args)
                })
            }
            ExprKind::Use { source } => this.then_else_break_inner(block, source, args),
            ExprKind::Let { expr, ref pat } => this.lower_let_expr(
                block,
                expr,
                pat,
                Some(args.variable_source_info.scope),
                args.variable_source_info.span,
                args.declare_let_bindings,
            ),
            _ => {
                let mut block = block;
                let temp_scope = args.temp_scope_override.unwrap_or_else(|| this.local_scope());
                let mutability = Mutability::Mut;
                let place =
                    unpack!(block = this.as_temp(block, Some(temp_scope), expr_id, mutability));
                let operand = Operand::Move(Place::from(place));

                let then_block = this.cfg.start_new_block();
                let else_block = this.cfg.start_new_block();
                let term = TerminatorKind::if_(operand, then_block, else_block);

                let source_info = this.source_info(expr_span);
                this.cfg.terminate(block, source_info, term);
                this.break_for_else(else_block, source_info);

                then_block.unit()
            }
        }
    }

    /// Generates MIR for a `match` expression.
    ///
    /// The MIR that we generate for a match looks like this.
    ///
    /// ```text
    /// [ 0. Pre-match ]
    ///        |
    /// [ 1. Evaluate Scrutinee (expression being matched on) ]
    /// [ (PlaceMention of scrutinee) ]
    ///        |
    /// [ 2. Decision tree -- check discriminants ] <--------+
    ///        |                                             |
    ///        | (once a specific arm is chosen)             |
    ///        |                                             |
    /// [pre_binding_block]                           [otherwise_block]
    ///        |                                             |
    /// [ 3. Create "guard bindings" for arm ]               |
    /// [ (create fake borrows) ]                            |
    ///        |                                             |
    /// [ 4. Execute guard code ]                            |
    /// [ (read fake borrows) ] --(guard is false)-----------+
    ///        |
    ///        | (guard results in true)
    ///        |
    /// [ 5. Create real bindings and execute arm ]
    ///        |
    /// [ Exit match ]
    /// ```
    ///
    /// All of the different arms have been stacked on top of each other to
    /// simplify the diagram. For an arm with no guard the blocks marked 3 and
    /// 4 and the fake borrows are omitted.
    ///
    /// We generate MIR in the following steps:
    ///
    /// 1. Evaluate the scrutinee and add the PlaceMention of it ([Builder::lower_scrutinee]).
    /// 2. Create the decision tree ([Builder::lower_match_tree]).
    /// 3. Determine the fake borrows that are needed from the places that were
    ///    matched against and create the required temporaries for them
    ///    ([Builder::calculate_fake_borrows]).
    /// 4. Create everything else: the guards and the arms ([Builder::lower_match_arms]).
    ///
    /// ## False edges
    ///
    /// We don't want to have the exact structure of the decision tree be
    /// visible through borrow checking. False edges ensure that the CFG as
    /// seen by borrow checking doesn't encode this. False edges are added:
    ///
    /// * From each pre-binding block to the next pre-binding block.
    /// * From each otherwise block to the next pre-binding block.
    #[instrument(level = "debug", skip(self, arms))]
    pub(crate) fn match_expr(
        &mut self,
        destination: Place<'tcx>,
        mut block: BasicBlock,
        scrutinee_id: ExprId,
        arms: &[ArmId],
        span: Span,
        scrutinee_span: Span,
    ) -> BlockAnd<()> {
        let scrutinee_span = scrutinee_span;
        let scrutinee_place =
            unpack!(block = self.lower_scrutinee(block, scrutinee_id, scrutinee_span));

        let mut arm_candidates = self.create_match_candidates(&scrutinee_place, arms);

        let match_has_guard = arm_candidates.iter().any(|(_, candidate)| candidate.has_guard);
        let mut candidates =
            arm_candidates.iter_mut().map(|(_, candidate)| candidate).collect::<Vec<_>>();

        let match_start_span = span.shrink_to_lo().to(scrutinee_span);

        let fake_borrow_temps = self.lower_match_tree(
            block,
            scrutinee_span,
            &scrutinee_place,
            match_start_span,
            match_has_guard,
            &mut candidates,
        );

        self.lower_match_arms(
            destination,
            scrutinee_place,
            scrutinee_span,
            arm_candidates,
            self.source_info(span),
            fake_borrow_temps,
        )
    }

    /// Evaluate the scrutinee and add the PlaceMention for it.
    fn lower_scrutinee(
        &mut self,
        mut block: BasicBlock,
        scrutinee_id: ExprId,
        scrutinee_span: Span,
    ) -> BlockAnd<PlaceBuilder<'tcx>> {
        let scrutinee_place_builder = unpack!(block = self.as_place_builder(block, scrutinee_id));
        if let Some(scrutinee_place) = scrutinee_place_builder.try_to_place(self) {
            let source_info = self.source_info(scrutinee_span);
            self.cfg.push_place_mention(block, source_info, scrutinee_place);
        }

        block.and(scrutinee_place_builder)
    }

    /// Create the initial `Candidate`s for a `match` expression.
    fn create_match_candidates<'pat>(
        &mut self,
        scrutinee: &PlaceBuilder<'tcx>,
        arms: &'pat [ArmId],
    ) -> Vec<(&'pat Arm<'tcx>, Candidate<'pat, 'tcx>)>
    where
        'a: 'pat,
    {
        // Assemble a list of candidates: there is one candidate per pattern,
        // which means there may be more than one candidate *per arm*.
        arms.iter()
            .copied()
            .map(|arm| {
                let arm = &self.thir[arm];
                let arm_has_guard = arm.guard.is_some();
                let arm_candidate =
                    Candidate::new(scrutinee.clone(), &arm.pattern, arm_has_guard, self);
                (arm, arm_candidate)
            })
            .collect()
    }

    /// Create the decision tree for the match expression, starting from `block`.
    ///
    /// Modifies `candidates` to store the bindings and type ascriptions for
    /// that candidate.
    ///
    /// Returns the places that need fake borrows because we bind or test them.
    fn lower_match_tree<'pat>(
        &mut self,
        block: BasicBlock,
        scrutinee_span: Span,
        scrutinee_place_builder: &PlaceBuilder<'tcx>,
        match_start_span: Span,
        match_has_guard: bool,
        candidates: &mut [&mut Candidate<'pat, 'tcx>],
    ) -> Vec<(Place<'tcx>, Local)> {
        // The set of places that we are creating fake borrows of. If there are
        // no match guards then we don't need any fake borrows, so don't track
        // them.
        let fake_borrows = match_has_guard
            .then(|| util::FakeBorrowCollector::collect_fake_borrows(self, candidates));

        let otherwise_block = self.cfg.start_new_block();

        // This will generate code to test scrutinee_place and
        // branch to the appropriate arm block
        self.match_candidates(match_start_span, scrutinee_span, block, otherwise_block, candidates);

        // See the doc comment on `match_candidates` for why we may have an
        // otherwise block. Match checking will ensure this is actually
        // unreachable.
        let source_info = self.source_info(scrutinee_span);

        // Matching on a `scrutinee_place` with an uninhabited type doesn't
        // generate any memory reads by itself, and so if the place "expression"
        // contains unsafe operations like raw pointer dereferences or union
        // field projections, we wouldn't know to require an `unsafe` block
        // around a `match` equivalent to `std::intrinsics::unreachable()`.
        // See issue #47412 for this hole being discovered in the wild.
        //
        // HACK(eddyb) Work around the above issue by adding a dummy inspection
        // of `scrutinee_place`, specifically by applying `ReadForMatch`.
        //
        // NOTE: ReadForMatch also checks that the scrutinee is initialized.
        // This is currently needed to not allow matching on an uninitialized,
        // uninhabited value. If we get never patterns, those will check that
        // the place is initialized, and so this read would only be used to
        // check safety.
        let cause_matched_place = FakeReadCause::ForMatchedPlace(None);

        if let Some(scrutinee_place) = scrutinee_place_builder.try_to_place(self) {
            self.cfg.push_fake_read(
                otherwise_block,
                source_info,
                cause_matched_place,
                scrutinee_place,
            );
        }

        self.cfg.terminate(otherwise_block, source_info, TerminatorKind::Unreachable);

        // Link each leaf candidate to the `pre_binding_block` of the next one.
        let mut previous_candidate: Option<&mut Candidate<'_, '_>> = None;

        for candidate in candidates {
            candidate.visit_leaves(|leaf_candidate| {
                if let Some(ref mut prev) = previous_candidate {
                    prev.next_candidate_pre_binding_block = leaf_candidate.pre_binding_block;
                }
                previous_candidate = Some(leaf_candidate);
            });
        }

        if let Some(ref borrows) = fake_borrows {
            self.calculate_fake_borrows(borrows, scrutinee_span)
        } else {
            Vec::new()
        }
    }

    /// Lower the bindings, guards and arm bodies of a `match` expression.
    ///
    /// The decision tree should have already been created
    /// (by [Builder::lower_match_tree]).
    ///
    /// `outer_source_info` is the SourceInfo for the whole match.
    fn lower_match_arms(
        &mut self,
        destination: Place<'tcx>,
        scrutinee_place_builder: PlaceBuilder<'tcx>,
        scrutinee_span: Span,
        arm_candidates: Vec<(&'_ Arm<'tcx>, Candidate<'_, 'tcx>)>,
        outer_source_info: SourceInfo,
        fake_borrow_temps: Vec<(Place<'tcx>, Local)>,
    ) -> BlockAnd<()> {
        let arm_end_blocks: Vec<_> = arm_candidates
            .into_iter()
            .map(|(arm, candidate)| {
                debug!("lowering arm {:?}\ncandidate = {:?}", arm, candidate);

                let arm_source_info = self.source_info(arm.span);
                let arm_scope = (arm.scope, arm_source_info);
                let match_scope = self.local_scope();
                self.in_scope(arm_scope, arm.lint_level, |this| {
                    let old_dedup_scope =
                        mem::replace(&mut this.fixed_temps_scope, Some(arm.scope));

                    // `try_to_place` may fail if it is unable to resolve the given
                    // `PlaceBuilder` inside a closure. In this case, we don't want to include
                    // a scrutinee place. `scrutinee_place_builder` will fail to be resolved
                    // if the only match arm is a wildcard (`_`).
                    // Example:
                    // ```
                    // let foo = (0, 1);
                    // let c = || {
                    //    match foo { _ => () };
                    // };
                    // ```
                    let scrutinee_place = scrutinee_place_builder.try_to_place(this);
                    let opt_scrutinee_place =
                        scrutinee_place.as_ref().map(|place| (Some(place), scrutinee_span));
                    let scope = this.declare_bindings(
                        None,
                        arm.span,
                        &arm.pattern,
                        arm.guard,
                        opt_scrutinee_place,
                    );

                    let arm_block = this.bind_pattern(
                        outer_source_info,
                        candidate,
                        &fake_borrow_temps,
                        scrutinee_span,
                        Some((arm, match_scope)),
                        false,
                    );

                    this.fixed_temps_scope = old_dedup_scope;

                    if let Some(source_scope) = scope {
                        this.source_scope = source_scope;
                    }

                    this.expr_into_dest(destination, arm_block, arm.body)
                })
            })
            .collect();

        // all the arm blocks will rejoin here
        let end_block = self.cfg.start_new_block();

        let end_brace = self.source_info(
            outer_source_info.span.with_lo(outer_source_info.span.hi() - BytePos::from_usize(1)),
        );
        for arm_block in arm_end_blocks {
            let block = &self.cfg.basic_blocks[arm_block.0];
            let last_location = block.statements.last().map(|s| s.source_info);

            self.cfg.goto(unpack!(arm_block), last_location.unwrap_or(end_brace), end_block);
        }

        self.source_scope = outer_source_info.scope;

        end_block.unit()
    }

    /// Binds the variables and ascribes types for a given `match` arm or
    /// `let` binding.
    ///
    /// Also check if the guard matches, if it's provided.
    /// `arm_scope` should be `Some` if and only if this is called for a
    /// `match` arm.
    fn bind_pattern(
        &mut self,
        outer_source_info: SourceInfo,
        candidate: Candidate<'_, 'tcx>,
        fake_borrow_temps: &[(Place<'tcx>, Local)],
        scrutinee_span: Span,
        arm_match_scope: Option<(&Arm<'tcx>, region::Scope)>,
        storages_alive: bool,
    ) -> BasicBlock {
        if candidate.subcandidates.is_empty() {
            // Avoid generating another `BasicBlock` when we only have one
            // candidate.
            self.bind_and_guard_matched_candidate(
                candidate,
                &[],
                fake_borrow_temps,
                scrutinee_span,
                arm_match_scope,
                true,
                storages_alive,
            )
        } else {
            // It's helpful to avoid scheduling drops multiple times to save
            // drop elaboration from having to clean up the extra drops.
            //
            // If we are in a `let` then we only schedule drops for the first
            // candidate.
            //
            // If we're in a `match` arm then we could have a case like so:
            //
            // Ok(x) | Err(x) if return => { /* ... */ }
            //
            // In this case we don't want a drop of `x` scheduled when we
            // return: it isn't bound by move until right before enter the arm.
            // To handle this we instead unschedule it's drop after each time
            // we lower the guard.
            let target_block = self.cfg.start_new_block();
            let mut schedule_drops = true;
            let arm = arm_match_scope.unzip().0;
            // We keep a stack of all of the bindings and type ascriptions
            // from the parent candidates that we visit, that also need to
            // be bound for each candidate.
            traverse_candidate(
                candidate,
                &mut Vec::new(),
                &mut |leaf_candidate, parent_bindings| {
                    if let Some(arm) = arm {
                        self.clear_top_scope(arm.scope);
                    }
                    let binding_end = self.bind_and_guard_matched_candidate(
                        leaf_candidate,
                        parent_bindings,
                        fake_borrow_temps,
                        scrutinee_span,
                        arm_match_scope,
                        schedule_drops,
                        storages_alive,
                    );
                    if arm.is_none() {
                        schedule_drops = false;
                    }
                    self.cfg.goto(binding_end, outer_source_info, target_block);
                },
                |inner_candidate, parent_bindings| {
                    parent_bindings.push((inner_candidate.bindings, inner_candidate.ascriptions));
                    inner_candidate.subcandidates.into_iter()
                },
                |parent_bindings| {
                    parent_bindings.pop();
                },
            );

            target_block
        }
    }

    pub(super) fn expr_into_pattern(
        &mut self,
        mut block: BasicBlock,
        irrefutable_pat: &Pat<'tcx>,
        initializer_id: ExprId,
    ) -> BlockAnd<()> {
        match irrefutable_pat.kind {
            // Optimize the case of `let x = ...` to write directly into `x`
            PatKind::Binding { mode: BindingMode::ByValue, var, subpattern: None, .. } => {
                let place =
                    self.storage_live_binding(block, var, irrefutable_pat.span, OutsideGuard, true);
                unpack!(block = self.expr_into_dest(place, block, initializer_id));

                // Inject a fake read, see comments on `FakeReadCause::ForLet`.
                let source_info = self.source_info(irrefutable_pat.span);
                self.cfg.push_fake_read(block, source_info, FakeReadCause::ForLet(None), place);

                self.schedule_drop_for_binding(var, irrefutable_pat.span, OutsideGuard);
                block.unit()
            }

            // Optimize the case of `let x: T = ...` to write directly
            // into `x` and then require that `T == typeof(x)`.
            //
            // Weirdly, this is needed to prevent the
            // `intrinsic-move-val.rs` test case from crashing. That
            // test works with uninitialized values in a rather
            // dubious way, so it may be that the test is kind of
            // broken.
            PatKind::AscribeUserType {
                subpattern:
                    box Pat {
                        kind:
                            PatKind::Binding {
                                mode: BindingMode::ByValue, var, subpattern: None, ..
                            },
                        ..
                    },
                ascription: thir::Ascription { ref annotation, variance: _ },
            } => {
                let place =
                    self.storage_live_binding(block, var, irrefutable_pat.span, OutsideGuard, true);
                unpack!(block = self.expr_into_dest(place, block, initializer_id));

                // Inject a fake read, see comments on `FakeReadCause::ForLet`.
                let pattern_source_info = self.source_info(irrefutable_pat.span);
                let cause_let = FakeReadCause::ForLet(None);
                self.cfg.push_fake_read(block, pattern_source_info, cause_let, place);

                let ty_source_info = self.source_info(annotation.span);

                let base = self.canonical_user_type_annotations.push(annotation.clone());
                self.cfg.push(
                    block,
                    Statement {
                        source_info: ty_source_info,
                        kind: StatementKind::AscribeUserType(
                            Box::new((place, UserTypeProjection { base, projs: Vec::new() })),
                            // We always use invariant as the variance here. This is because the
                            // variance field from the ascription refers to the variance to use
                            // when applying the type to the value being matched, but this
                            // ascription applies rather to the type of the binding. e.g., in this
                            // example:
                            //
                            // ```
                            // let x: T = <expr>
                            // ```
                            //
                            // We are creating an ascription that defines the type of `x` to be
                            // exactly `T` (i.e., with invariance). The variance field, in
                            // contrast, is intended to be used to relate `T` to the type of
                            // `<expr>`.
                            ty::Variance::Invariant,
                        ),
                    },
                );

                self.schedule_drop_for_binding(var, irrefutable_pat.span, OutsideGuard);
                block.unit()
            }

            _ => {
                let initializer = &self.thir[initializer_id];
                let place_builder =
                    unpack!(block = self.lower_scrutinee(block, initializer_id, initializer.span));
                self.place_into_pattern(block, irrefutable_pat, place_builder, true)
            }
        }
    }

    pub(crate) fn place_into_pattern(
        &mut self,
        block: BasicBlock,
        irrefutable_pat: &Pat<'tcx>,
        initializer: PlaceBuilder<'tcx>,
        set_match_place: bool,
    ) -> BlockAnd<()> {
        let mut candidate = Candidate::new(initializer.clone(), irrefutable_pat, false, self);
        let fake_borrow_temps = self.lower_match_tree(
            block,
            irrefutable_pat.span,
            &initializer,
            irrefutable_pat.span,
            false,
            &mut [&mut candidate],
        );

        // For matches and function arguments, the place that is being matched
        // can be set when creating the variables. But the place for
        // let PATTERN = ... might not even exist until we do the assignment.
        // so we set it here instead.
        if set_match_place {
            let mut next = Some(&candidate);
            while let Some(candidate_ref) = next.take() {
                for binding in &candidate_ref.bindings {
                    let local = self.var_local_id(binding.var_id, OutsideGuard);
                    // `try_to_place` may fail if it is unable to resolve the given
                    // `PlaceBuilder` inside a closure. In this case, we don't want to include
                    // a scrutinee place. `scrutinee_place_builder` will fail for destructured
                    // assignments. This is because a closure only captures the precise places
                    // that it will read and as a result a closure may not capture the entire
                    // tuple/struct and rather have individual places that will be read in the
                    // final MIR.
                    // Example:
                    // ```
                    // let foo = (0, 1);
                    // let c = || {
                    //    let (v1, v2) = foo;
                    // };
                    // ```
                    if let Some(place) = initializer.try_to_place(self) {
                        let LocalInfo::User(BindingForm::Var(VarBindingForm {
                            opt_match_place: Some((ref mut match_place, _)),
                            ..
                        })) = **self.local_decls[local].local_info.as_mut().assert_crate_local()
                        else {
                            bug!("Let binding to non-user variable.")
                        };
                        *match_place = Some(place);
                    }
                }
                // All of the subcandidates should bind the same locals, so we
                // only visit the first one.
                next = candidate_ref.subcandidates.get(0)
            }
        }

        self.bind_pattern(
            self.source_info(irrefutable_pat.span),
            candidate,
            fake_borrow_temps.as_slice(),
            irrefutable_pat.span,
            None,
            false,
        )
        .unit()
    }

    /// Declares the bindings of the given patterns and returns the visibility
    /// scope for the bindings in these patterns, if such a scope had to be
    /// created. NOTE: Declaring the bindings should always be done in their
    /// drop scope.
    #[instrument(skip(self), level = "debug")]
    pub(crate) fn declare_bindings(
        &mut self,
        mut visibility_scope: Option<SourceScope>,
        scope_span: Span,
        pattern: &Pat<'tcx>,
        guard: Option<ExprId>,
        opt_match_place: Option<(Option<&Place<'tcx>>, Span)>,
    ) -> Option<SourceScope> {
        self.visit_primary_bindings(
            pattern,
            UserTypeProjections::none(),
            &mut |this, mutability, name, mode, var, span, ty, user_ty| {
                if visibility_scope.is_none() {
                    visibility_scope =
                        Some(this.new_source_scope(scope_span, LintLevel::Inherited, None));
                }
                let source_info = SourceInfo { span, scope: this.source_scope };
                let visibility_scope = visibility_scope.unwrap();
                this.declare_binding(
                    source_info,
                    visibility_scope,
                    mutability,
                    name,
                    mode,
                    var,
                    ty,
                    user_ty,
                    ArmHasGuard(guard.is_some()),
                    opt_match_place.map(|(x, y)| (x.cloned(), y)),
                    pattern.span,
                );
            },
        );
        if let Some(guard_expr) = guard {
            self.declare_guard_bindings(guard_expr, scope_span, visibility_scope);
        }
        visibility_scope
    }

    /// Declare bindings in a guard. This has to be done when declaring bindings
    /// for an arm to ensure that or patterns only have one version of each
    /// variable.
    pub(crate) fn declare_guard_bindings(
        &mut self,
        guard_expr: ExprId,
        scope_span: Span,
        visibility_scope: Option<SourceScope>,
    ) {
        match self.thir.exprs[guard_expr].kind {
            ExprKind::Let { expr: _, pat: ref guard_pat } => {
                // FIXME: pass a proper `opt_match_place`
                self.declare_bindings(visibility_scope, scope_span, guard_pat, None, None);
            }
            ExprKind::Scope { value, .. } => {
                self.declare_guard_bindings(value, scope_span, visibility_scope);
            }
            ExprKind::Use { source } => {
                self.declare_guard_bindings(source, scope_span, visibility_scope);
            }
            ExprKind::LogicalOp { op: LogicalOp::And, lhs, rhs } => {
                self.declare_guard_bindings(lhs, scope_span, visibility_scope);
                self.declare_guard_bindings(rhs, scope_span, visibility_scope);
            }
            _ => {}
        }
    }

    pub(crate) fn storage_live_binding(
        &mut self,
        block: BasicBlock,
        var: LocalVarId,
        span: Span,
        for_guard: ForGuard,
        schedule_drop: bool,
    ) -> Place<'tcx> {
        let local_id = self.var_local_id(var, for_guard);
        let source_info = self.source_info(span);
        self.cfg.push(block, Statement { source_info, kind: StatementKind::StorageLive(local_id) });
        // Although there is almost always scope for given variable in corner cases
        // like #92893 we might get variable with no scope.
        if let Some(region_scope) = self.region_scope_tree.var_scope(var.0.local_id)
            && schedule_drop
        {
            self.schedule_drop(span, region_scope, local_id, DropKind::Storage);
        }
        Place::from(local_id)
    }

    pub(crate) fn schedule_drop_for_binding(
        &mut self,
        var: LocalVarId,
        span: Span,
        for_guard: ForGuard,
    ) {
        let local_id = self.var_local_id(var, for_guard);
        if let Some(region_scope) = self.region_scope_tree.var_scope(var.0.local_id) {
            self.schedule_drop(span, region_scope, local_id, DropKind::Value);
        }
    }

    /// Visit all of the primary bindings in a patterns, that is, visit the
    /// leftmost occurrence of each variable bound in a pattern. A variable
    /// will occur more than once in an or-pattern.
    pub(super) fn visit_primary_bindings(
        &mut self,
        pattern: &Pat<'tcx>,
        pattern_user_ty: UserTypeProjections,
        f: &mut impl FnMut(
            &mut Self,
            Mutability,
            Symbol,
            BindingMode,
            LocalVarId,
            Span,
            Ty<'tcx>,
            UserTypeProjections,
        ),
    ) {
        debug!(
            "visit_primary_bindings: pattern={:?} pattern_user_ty={:?}",
            pattern, pattern_user_ty
        );
        match pattern.kind {
            PatKind::Binding {
                mutability,
                name,
                mode,
                var,
                ty,
                ref subpattern,
                is_primary,
                ..
            } => {
                if is_primary {
                    f(self, mutability, name, mode, var, pattern.span, ty, pattern_user_ty.clone());
                }
                if let Some(subpattern) = subpattern.as_ref() {
                    self.visit_primary_bindings(subpattern, pattern_user_ty, f);
                }
            }

            PatKind::Array { ref prefix, ref slice, ref suffix }
            | PatKind::Slice { ref prefix, ref slice, ref suffix } => {
                let from = u64::try_from(prefix.len()).unwrap();
                let to = u64::try_from(suffix.len()).unwrap();
                for subpattern in prefix.iter() {
                    self.visit_primary_bindings(subpattern, pattern_user_ty.clone().index(), f);
                }
                for subpattern in slice {
                    self.visit_primary_bindings(
                        subpattern,
                        pattern_user_ty.clone().subslice(from, to),
                        f,
                    );
                }
                for subpattern in suffix.iter() {
                    self.visit_primary_bindings(subpattern, pattern_user_ty.clone().index(), f);
                }
            }

            PatKind::Constant { .. }
            | PatKind::Range { .. }
            | PatKind::Wild
            | PatKind::Never
            | PatKind::Error(_) => {}

            PatKind::Deref { ref subpattern } => {
                self.visit_primary_bindings(subpattern, pattern_user_ty.deref(), f);
            }

            PatKind::AscribeUserType {
                ref subpattern,
                ascription: thir::Ascription { ref annotation, variance: _ },
            } => {
                // This corresponds to something like
                //
                // ```
                // let A::<'a>(_): A<'static> = ...;
                // ```
                //
                // Note that the variance doesn't apply here, as we are tracking the effect
                // of `user_ty` on any bindings contained with subpattern.

                let projection = UserTypeProjection {
                    base: self.canonical_user_type_annotations.push(annotation.clone()),
                    projs: Vec::new(),
                };
                let subpattern_user_ty =
                    pattern_user_ty.push_projection(&projection, annotation.span);
                self.visit_primary_bindings(subpattern, subpattern_user_ty, f)
            }

            PatKind::InlineConstant { ref subpattern, .. } => {
                self.visit_primary_bindings(subpattern, pattern_user_ty, f)
            }

            PatKind::Leaf { ref subpatterns } => {
                for subpattern in subpatterns {
                    let subpattern_user_ty = pattern_user_ty.clone().leaf(subpattern.field);
                    debug!("visit_primary_bindings: subpattern_user_ty={:?}", subpattern_user_ty);
                    self.visit_primary_bindings(&subpattern.pattern, subpattern_user_ty, f);
                }
            }

            PatKind::Variant { adt_def, args: _, variant_index, ref subpatterns } => {
                for subpattern in subpatterns {
                    let subpattern_user_ty =
                        pattern_user_ty.clone().variant(adt_def, variant_index, subpattern.field);
                    self.visit_primary_bindings(&subpattern.pattern, subpattern_user_ty, f);
                }
            }
            PatKind::Or { ref pats } => {
                // In cases where we recover from errors the primary bindings
                // may not all be in the leftmost subpattern. For example in
                // `let (x | y) = ...`, the primary binding of `y` occurs in
                // the right subpattern
                for subpattern in pats.iter() {
                    self.visit_primary_bindings(subpattern, pattern_user_ty.clone(), f);
                }
            }
        }
    }
}

/// A pattern in a form suitable for generating code.
#[derive(Debug, Clone)]
struct FlatPat<'pat, 'tcx> {
    /// [`Span`] of the original pattern.
    span: Span,

    /// To match the pattern, all of these must be satisfied...
    // Invariant: all the `MatchPair`s are recursively simplified.
    // Invariant: or-patterns must be sorted to the end.
    match_pairs: Vec<MatchPair<'pat, 'tcx>>,

    /// ...these bindings established...
    bindings: Vec<Binding<'tcx>>,

    /// ...and these types asserted.
    ascriptions: Vec<Ascription<'tcx>>,
}

impl<'tcx, 'pat> FlatPat<'pat, 'tcx> {
    fn new(
        place: PlaceBuilder<'tcx>,
        pattern: &'pat Pat<'tcx>,
        cx: &mut Builder<'_, 'tcx>,
    ) -> Self {
        let mut match_pairs = vec![MatchPair::new(place, pattern, cx)];
        let mut bindings = Vec::new();
        let mut ascriptions = Vec::new();

        cx.simplify_match_pairs(&mut match_pairs, &mut bindings, &mut ascriptions);

        FlatPat { span: pattern.span, match_pairs, bindings, ascriptions }
    }
}

#[derive(Debug)]
struct Candidate<'pat, 'tcx> {
    /// [`Span`] of the original pattern that gave rise to this candidate.
    span: Span,

    /// Whether this `Candidate` has a guard.
    has_guard: bool,

    /// All of these must be satisfied...
    // Invariant: all the `MatchPair`s are recursively simplified.
    // Invariant: or-patterns must be sorted at the end.
    match_pairs: Vec<MatchPair<'pat, 'tcx>>,

    /// ...these bindings established...
    // Invariant: not mutated after candidate creation.
    bindings: Vec<Binding<'tcx>>,

    /// ...and these types asserted...
    // Invariant: not mutated after candidate creation.
    ascriptions: Vec<Ascription<'tcx>>,

    /// ...and if this is non-empty, one of these subcandidates also has to match...
    subcandidates: Vec<Candidate<'pat, 'tcx>>,

    /// ...and the guard must be evaluated; if it's `false` then branch to `otherwise_block`.
    otherwise_block: Option<BasicBlock>,

    /// The block before the `bindings` have been established.
    pre_binding_block: Option<BasicBlock>,
    /// The pre-binding block of the next candidate.
    next_candidate_pre_binding_block: Option<BasicBlock>,
}

impl<'tcx, 'pat> Candidate<'pat, 'tcx> {
    fn new(
        place: PlaceBuilder<'tcx>,
        pattern: &'pat Pat<'tcx>,
        has_guard: bool,
        cx: &mut Builder<'_, 'tcx>,
    ) -> Self {
        Self::from_flat_pat(FlatPat::new(place, pattern, cx), has_guard)
    }

    fn from_flat_pat(flat_pat: FlatPat<'pat, 'tcx>, has_guard: bool) -> Self {
        Candidate {
            span: flat_pat.span,
            match_pairs: flat_pat.match_pairs,
            bindings: flat_pat.bindings,
            ascriptions: flat_pat.ascriptions,
            has_guard,
            subcandidates: Vec::new(),
            otherwise_block: None,
            pre_binding_block: None,
            next_candidate_pre_binding_block: None,
        }
    }

    /// Visit the leaf candidates (those with no subcandidates) contained in
    /// this candidate.
    fn visit_leaves<'a>(&'a mut self, mut visit_leaf: impl FnMut(&'a mut Self)) {
        traverse_candidate(
            self,
            &mut (),
            &mut move |c, _| visit_leaf(c),
            move |c, _| c.subcandidates.iter_mut(),
            |_| {},
        );
    }
}

/// A depth-first traversal of the `Candidate` and all of its recursive
/// subcandidates.
fn traverse_candidate<'pat, 'tcx: 'pat, C, T, I>(
    candidate: C,
    context: &mut T,
    visit_leaf: &mut impl FnMut(C, &mut T),
    get_children: impl Copy + Fn(C, &mut T) -> I,
    complete_children: impl Copy + Fn(&mut T),
) where
    C: Borrow<Candidate<'pat, 'tcx>>,
    I: Iterator<Item = C>,
{
    if candidate.borrow().subcandidates.is_empty() {
        visit_leaf(candidate, context)
    } else {
        for child in get_children(candidate, context) {
            traverse_candidate(child, context, visit_leaf, get_children, complete_children);
        }
        complete_children(context)
    }
}

#[derive(Clone, Debug)]
struct Binding<'tcx> {
    span: Span,
    source: Place<'tcx>,
    var_id: LocalVarId,
    binding_mode: BindingMode,
}

/// Indicates that the type of `source` must be a subtype of the
/// user-given type `user_ty`; this is basically a no-op but can
/// influence region inference.
#[derive(Clone, Debug)]
struct Ascription<'tcx> {
    source: Place<'tcx>,
    annotation: CanonicalUserTypeAnnotation<'tcx>,
    variance: ty::Variance,
}

#[derive(Debug, Clone)]
enum TestCase<'pat, 'tcx> {
    Irrefutable { binding: Option<Binding<'tcx>>, ascription: Option<Ascription<'tcx>> },
    Variant { adt_def: ty::AdtDef<'tcx>, variant_index: VariantIdx },
    Constant { value: mir::Const<'tcx> },
    Range(&'pat PatRange<'tcx>),
    Slice { len: usize, variable_length: bool },
    Or { pats: Box<[FlatPat<'pat, 'tcx>]> },
}

#[derive(Debug, Clone)]
pub(crate) struct MatchPair<'pat, 'tcx> {
    /// This place...
    place: PlaceBuilder<'tcx>,

    /// ... must pass this test...
    // Invariant: after creation and simplification in `Candidate::new()`, this must not be
    // `Irrefutable`.
    test_case: TestCase<'pat, 'tcx>,

    /// ... and these subpairs must match.
    subpairs: Vec<Self>,

    /// The pattern this was created from.
    pattern: &'pat Pat<'tcx>,
}

/// See [`Test`] for more.
#[derive(Clone, Debug, PartialEq)]
enum TestKind<'tcx> {
    /// Test what enum variant a value is.
    Switch {
        /// The enum type being tested.
        adt_def: ty::AdtDef<'tcx>,
        /// The set of variants that we should create a branch for. We also
        /// create an additional "otherwise" case.
        variants: BitSet<VariantIdx>,
    },

    /// Test what value an integer or `char` has.
    SwitchInt {
        /// The (ordered) set of values that we test for.
        ///
        /// We create a branch to each of the values in `options`, as well as an "otherwise" branch
        /// for all other values, even in the (rare) case that `options` is exhaustive.
        options: FxIndexMap<Const<'tcx>, u128>,
    },

    /// Test what value a `bool` has.
    If,

    /// Test for equality with value, possibly after an unsizing coercion to
    /// `ty`,
    Eq {
        value: Const<'tcx>,
        // Integer types are handled by `SwitchInt`, and constants with ADT
        // types are converted back into patterns, so this can only be `&str`,
        // `&[T]`, `f32` or `f64`.
        ty: Ty<'tcx>,
    },

    /// Test whether the value falls within an inclusive or exclusive range.
    Range(Box<PatRange<'tcx>>),

    /// Test that the length of the slice is equal to `len`.
    Len { len: u64, op: BinOp },
}

/// A test to perform to determine which [`Candidate`] matches a value.
///
/// [`Test`] is just the test to perform; it does not include the value
/// to be tested.
#[derive(Debug)]
pub(crate) struct Test<'tcx> {
    span: Span,
    kind: TestKind<'tcx>,
}

/// `ArmHasGuard` is a wrapper around a boolean flag. It indicates whether
/// a match arm has a guard expression attached to it.
#[derive(Copy, Clone, Debug)]
pub(crate) struct ArmHasGuard(pub(crate) bool);

///////////////////////////////////////////////////////////////////////////
// Main matching algorithm

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// The main match algorithm. It begins with a set of candidates
    /// `candidates` and has the job of generating code to determine
    /// which of these candidates, if any, is the correct one. The
    /// candidates are sorted such that the first item in the list
    /// has the highest priority. When a candidate is found to match
    /// the value, we will set and generate a branch to the appropriate
    /// pre-binding block.
    ///
    /// If we find that *NONE* of the candidates apply, we branch to `otherwise_block`.
    ///
    /// It might be surprising that the input can be non-exhaustive.
    /// Indeed, initially, it is not, because all matches are
    /// exhaustive in Rust. But during processing we sometimes divide
    /// up the list of candidates and recurse with a non-exhaustive
    /// list. This is how our lowering approach (called "backtracking
    /// automaton" in the literature) works.
    /// See [`Builder::test_candidates`] for more details.
    ///
    /// If `fake_borrows` is `Some`, then places which need fake borrows
    /// will be added to it.
    ///
    /// For an example of how we use `otherwise_block`, consider:
    /// ```
    /// # fn foo((x, y): (bool, bool)) -> u32 {
    /// match (x, y) {
    ///     (true, true) => 1,
    ///     (_, false) => 2,
    ///     (false, true) => 3,
    /// }
    /// # }
    /// ```
    /// For this match, we generate something like:
    /// ```
    /// # fn foo((x, y): (bool, bool)) -> u32 {
    /// if x {
    ///     if y {
    ///         return 1
    ///     } else {
    ///         // continue
    ///     }
    /// } else {
    ///     // continue
    /// }
    /// if y {
    ///     if x {
    ///         // This is actually unreachable because the `(true, true)` case was handled above.
    ///         // continue
    ///     } else {
    ///         return 3
    ///     }
    /// } else {
    ///     return 2
    /// }
    /// // this is the final `otherwise_block`, which is unreachable because the match was exhaustive.
    /// unreachable!()
    /// # }
    /// ```
    ///
    /// Every `continue` is an instance of branching to some `otherwise_block` somewhere deep within
    /// the algorithm. For more details on why we lower like this, see [`Builder::test_candidates`].
    ///
    /// Note how we test `x` twice. This is the tradeoff of backtracking automata: we prefer smaller
    /// code size at the expense of non-optimal code paths.
    #[instrument(skip(self), level = "debug")]
    fn match_candidates<'pat>(
        &mut self,
        span: Span,
        scrutinee_span: Span,
        start_block: BasicBlock,
        otherwise_block: BasicBlock,
        candidates: &mut [&mut Candidate<'pat, 'tcx>],
    ) {
        let mut split_or_candidate = false;
        for candidate in &mut *candidates {
            if let [MatchPair { test_case: TestCase::Or { pats, .. }, .. }] =
                &*candidate.match_pairs
            {
                // Split a candidate in which the only match-pair is an or-pattern into multiple
                // candidates. This is so that
                //
                // match x {
                //     0 | 1 => { ... },
                //     2 | 3 => { ... },
                // }
                //
                // only generates a single switch.
                candidate.subcandidates = self.create_or_subcandidates(pats, candidate.has_guard);
                candidate.match_pairs.pop();
                split_or_candidate = true;
            }
        }

        ensure_sufficient_stack(|| {
            if split_or_candidate {
                // At least one of the candidates has been split into subcandidates.
                // We need to change the candidate list to include those.
                let mut new_candidates = Vec::new();

                for candidate in candidates {
                    candidate.visit_leaves(|leaf_candidate| new_candidates.push(leaf_candidate));
                }
                self.match_simplified_candidates(
                    span,
                    scrutinee_span,
                    start_block,
                    otherwise_block,
                    &mut *new_candidates,
                );
            } else {
                self.match_simplified_candidates(
                    span,
                    scrutinee_span,
                    start_block,
                    otherwise_block,
                    candidates,
                );
            }
        });
    }

    fn match_simplified_candidates(
        &mut self,
        span: Span,
        scrutinee_span: Span,
        mut start_block: BasicBlock,
        otherwise_block: BasicBlock,
        candidates: &mut [&mut Candidate<'_, 'tcx>],
    ) {
        match candidates {
            [] => {
                // If there are no candidates that still need testing, we're done. Since all matches are
                // exhaustive, execution should never reach this point.
                let source_info = self.source_info(span);
                self.cfg.goto(start_block, source_info, otherwise_block);
            }
            [first, remaining @ ..] if first.match_pairs.is_empty() => {
                // The first candidate has satisfied all its match pairs; we link it up and continue
                // with the remaining candidates.
                start_block = self.select_matched_candidate(first, start_block);
                self.match_simplified_candidates(
                    span,
                    scrutinee_span,
                    start_block,
                    otherwise_block,
                    remaining,
                )
            }
            candidates => {
                // The first candidate has some unsatisfied match pairs; we proceed to do more tests.
                self.test_candidates_with_or(
                    span,
                    scrutinee_span,
                    candidates,
                    start_block,
                    otherwise_block,
                );
            }
        }
    }

    /// Link up matched candidates.
    ///
    /// For example, if we have something like this:
    ///
    /// ```ignore (illustrative)
    /// ...
    /// Some(x) if cond1 => ...
    /// Some(x) => ...
    /// Some(x) if cond2 => ...
    /// ...
    /// ```
    ///
    /// We generate real edges from:
    ///
    /// * `start_block` to the [pre-binding block] of the first pattern,
    /// * the [otherwise block] of the first pattern to the second pattern,
    /// * the [otherwise block] of the third pattern to a block with an
    ///   [`Unreachable` terminator](TerminatorKind::Unreachable).
    ///
    /// In addition, we later add fake edges from the otherwise blocks to the
    /// pre-binding block of the next candidate in the original set of
    /// candidates.
    ///
    /// [pre-binding block]: Candidate::pre_binding_block
    /// [otherwise block]: Candidate::otherwise_block
    fn select_matched_candidate(
        &mut self,
        candidate: &mut Candidate<'_, 'tcx>,
        start_block: BasicBlock,
    ) -> BasicBlock {
        assert!(candidate.otherwise_block.is_none());
        assert!(candidate.pre_binding_block.is_none());
        assert!(candidate.subcandidates.is_empty());

        candidate.pre_binding_block = Some(start_block);
        let otherwise_block = self.cfg.start_new_block();
        if candidate.has_guard {
            // Create the otherwise block for this candidate, which is the
            // pre-binding block for the next candidate.
            candidate.otherwise_block = Some(otherwise_block);
        }
        otherwise_block
    }

    /// Tests a candidate where there are only or-patterns left to test, or
    /// forwards to [Builder::test_candidates].
    ///
    /// Given a pattern `(P | Q, R | S)` we (in principle) generate a CFG like
    /// so:
    ///
    /// ```text
    /// [ start ]
    ///      |
    /// [ match P, Q ]
    ///      |
    ///      +----------------------------------------+------------------------------------+
    ///      |                                        |                                    |
    ///      V                                        V                                    V
    /// [ P matches ]                           [ Q matches ]                        [ otherwise ]
    ///      |                                        |                                    |
    ///      V                                        V                                    |
    /// [ match R, S ]                          [ match R, S ]                             |
    ///      |                                        |                                    |
    ///      +--------------+------------+            +--------------+------------+        |
    ///      |              |            |            |              |            |        |
    ///      V              V            V            V              V            V        |
    /// [ R matches ] [ S matches ] [otherwise ] [ R matches ] [ S matches ] [otherwise ]  |
    ///      |              |            |            |              |            |        |
    ///      +--------------+------------|------------+--------------+            |        |
    ///      |                           |                                        |        |
    ///      |                           +----------------------------------------+--------+
    ///      |                           |
    ///      V                           V
    /// [ Success ]                 [ Failure ]
    /// ```
    ///
    /// In practice there are some complications:
    ///
    /// * If there's a guard, then the otherwise branch of the first match on
    ///   `R | S` goes to a test for whether `Q` matches, and the control flow
    ///   doesn't merge into a single success block until after the guard is
    ///   tested.
    /// * If neither `P` or `Q` has any bindings or type ascriptions and there
    ///   isn't a match guard, then we create a smaller CFG like:
    ///
    /// ```text
    ///     ...
    ///      +---------------+------------+
    ///      |               |            |
    /// [ P matches ] [ Q matches ] [ otherwise ]
    ///      |               |            |
    ///      +---------------+            |
    ///      |                           ...
    /// [ match R, S ]
    ///      |
    ///     ...
    /// ```
    fn test_candidates_with_or(
        &mut self,
        span: Span,
        scrutinee_span: Span,
        candidates: &mut [&mut Candidate<'_, 'tcx>],
        start_block: BasicBlock,
        otherwise_block: BasicBlock,
    ) {
        let (first_candidate, remaining_candidates) = candidates.split_first_mut().unwrap();
        assert!(first_candidate.subcandidates.is_empty());
        if !matches!(first_candidate.match_pairs[0].test_case, TestCase::Or { .. }) {
            self.test_candidates(span, scrutinee_span, candidates, start_block, otherwise_block);
            return;
        }

        let match_pairs = mem::take(&mut first_candidate.match_pairs);
        let (first_match_pair, remaining_match_pairs) = match_pairs.split_first().unwrap();
        let TestCase::Or { ref pats } = &first_match_pair.test_case else { unreachable!() };

        let remainder_start = self.cfg.start_new_block();
        let or_span = first_match_pair.pattern.span;
        // Test the alternatives of this or-pattern.
        self.test_or_pattern(first_candidate, start_block, remainder_start, pats, or_span);

        if !remaining_match_pairs.is_empty() {
            // If more match pairs remain, test them after each subcandidate.
            // We could add them to the or-candidates before the call to `test_or_pattern` but this
            // would make it impossible to detect simplifiable or-patterns. That would guarantee
            // exponentially large CFGs for cases like `(1 | 2, 3 | 4, ...)`.
            first_candidate.visit_leaves(|leaf_candidate| {
                assert!(leaf_candidate.match_pairs.is_empty());
                leaf_candidate.match_pairs.extend(remaining_match_pairs.iter().cloned());
                let or_start = leaf_candidate.pre_binding_block.unwrap();
                // In a case like `(a | b, c | d)`, if `a` succeeds and `c | d` fails, we know `(b,
                // c | d)` will fail too. If there is no guard, we skip testing of `b` by branching
                // directly to `remainder_start`. If there is a guard, we have to try `(b, c | d)`.
                let or_otherwise = leaf_candidate.otherwise_block.unwrap_or(remainder_start);
                self.test_candidates_with_or(
                    span,
                    scrutinee_span,
                    &mut [leaf_candidate],
                    or_start,
                    or_otherwise,
                );
            });
        }

        // Test the remaining candidates.
        self.match_candidates(
            span,
            scrutinee_span,
            remainder_start,
            otherwise_block,
            remaining_candidates,
        );
    }

    #[instrument(
        skip(self, start_block, otherwise_block, or_span, candidate, pats),
        level = "debug"
    )]
    fn test_or_pattern<'pat>(
        &mut self,
        candidate: &mut Candidate<'pat, 'tcx>,
        start_block: BasicBlock,
        otherwise_block: BasicBlock,
        pats: &[FlatPat<'pat, 'tcx>],
        or_span: Span,
    ) {
        debug!("candidate={:#?}\npats={:#?}", candidate, pats);
        let mut or_candidates: Vec<_> = pats
            .iter()
            .cloned()
            .map(|flat_pat| Candidate::from_flat_pat(flat_pat, candidate.has_guard))
            .collect();
        let mut or_candidate_refs: Vec<_> = or_candidates.iter_mut().collect();
        self.match_candidates(
            or_span,
            or_span,
            start_block,
            otherwise_block,
            &mut or_candidate_refs,
        );
        candidate.subcandidates = or_candidates;
        self.merge_trivial_subcandidates(candidate, self.source_info(or_span));
    }

    /// Try to merge all of the subcandidates of the given candidate into one.
    /// This avoids exponentially large CFGs in cases like `(1 | 2, 3 | 4, ...)`.
    fn merge_trivial_subcandidates(
        &mut self,
        candidate: &mut Candidate<'_, 'tcx>,
        source_info: SourceInfo,
    ) {
        if candidate.subcandidates.is_empty() || candidate.has_guard {
            // FIXME(or_patterns; matthewjasper) Don't give up if we have a guard.
            return;
        }

        let mut can_merge = true;

        // Not `Iterator::all` because we don't want to short-circuit.
        for subcandidate in &mut candidate.subcandidates {
            self.merge_trivial_subcandidates(subcandidate, source_info);

            // FIXME(or_patterns; matthewjasper) Try to be more aggressive here.
            can_merge &= subcandidate.subcandidates.is_empty()
                && subcandidate.bindings.is_empty()
                && subcandidate.ascriptions.is_empty();
        }

        if can_merge {
            let any_matches = self.cfg.start_new_block();
            for subcandidate in mem::take(&mut candidate.subcandidates) {
                let or_block = subcandidate.pre_binding_block.unwrap();
                self.cfg.goto(or_block, source_info, any_matches);
            }
            candidate.pre_binding_block = Some(any_matches);
        }
    }

    /// Pick a test to run. Which test doesn't matter as long as it is guaranteed to fully match at
    /// least one match pair. We currently simply pick the test corresponding to the first match
    /// pair of the first candidate in the list.
    ///
    /// *Note:* taking the first match pair is somewhat arbitrary, and we might do better here by
    /// choosing more carefully what to test.
    ///
    /// For example, consider the following possible match-pairs:
    ///
    /// 1. `x @ Some(P)` -- we will do a [`Switch`] to decide what variant `x` has
    /// 2. `x @ 22` -- we will do a [`SwitchInt`] to decide what value `x` has
    /// 3. `x @ 3..5` -- we will do a [`Range`] test to decide what range `x` falls in
    /// 4. etc.
    ///
    /// [`Switch`]: TestKind::Switch
    /// [`SwitchInt`]: TestKind::SwitchInt
    /// [`Range`]: TestKind::Range
    fn pick_test(
        &mut self,
        candidates: &mut [&mut Candidate<'_, 'tcx>],
    ) -> (PlaceBuilder<'tcx>, Test<'tcx>) {
        // Extract the match-pair from the highest priority candidate
        let match_pair = &candidates.first().unwrap().match_pairs[0];
        let mut test = self.test(match_pair);
        let match_place = match_pair.place.clone();

        debug!(?test, ?match_pair);
        // Most of the time, the test to perform is simply a function of the main candidate; but for
        // a test like SwitchInt, we may want to add cases based on the candidates that are
        // available
        match test.kind {
            TestKind::SwitchInt { ref mut options } => {
                for candidate in candidates.iter() {
                    if !self.add_cases_to_switch(&match_place, candidate, options) {
                        break;
                    }
                }
            }
            TestKind::Switch { adt_def: _, ref mut variants } => {
                for candidate in candidates.iter() {
                    if !self.add_variants_to_switch(&match_place, candidate, variants) {
                        break;
                    }
                }
            }
            _ => {}
        }

        (match_place, test)
    }

    /// Given a test, we sort the input candidates into several buckets. If a candidate only matches
    /// in one of the branches of `test`, we move it there. If it could match in more than one of
    /// the branches of `test`, we stop sorting candidates.
    ///
    /// This returns a pair of
    /// - the candidates that weren't sorted;
    /// - for each possible outcome of the test, the candidates that match in that outcome.
    ///
    /// Moreover, we transform the branched candidates to reflect the fact that we know which
    /// outcome of `test` occurred.
    ///
    /// For example:
    /// ```
    /// # let (x, y, z) = (true, true, true);
    /// match (x, y, z) {
    ///     (true , _    , true ) => true,  // (0)
    ///     (false, false, _    ) => false, // (1)
    ///     (_    , true , _    ) => true,  // (2)
    ///     (true , _    , false) => false, // (3)
    /// }
    /// # ;
    /// ```
    ///
    /// Assume we are testing on `x`. There are 2 overlapping candidate sets:
    /// - If the outcome is that `x` is true, candidates 0, 2, and 3
    /// - If the outcome is that `x` is false, candidates 1 and 2
    ///
    /// Following our algorithm, candidate 0 is sorted into outcome `x == true`, candidate 1 goes
    /// into outcome `x == false`, and candidate 2 and 3 remain unsorted.
    ///
    /// The sorted candidates are transformed:
    /// - candidate 0 becomes `[z @ true]` since we know that `x` was `true`;
    /// - candidate 1 becomes `[y @ false]` since we know that `x` was `false`.
    fn sort_candidates<'b, 'c, 'pat>(
        &mut self,
        match_place: &PlaceBuilder<'tcx>,
        test: &Test<'tcx>,
        mut candidates: &'b mut [&'c mut Candidate<'pat, 'tcx>],
    ) -> (&'b mut [&'c mut Candidate<'pat, 'tcx>], Vec<Vec<&'b mut Candidate<'pat, 'tcx>>>) {
        // For each of the N possible outcomes, create a (initially empty) vector of candidates.
        // Those are the candidates that apply if the test has that particular outcome.
        let mut target_candidates: Vec<Vec<&mut Candidate<'pat, 'tcx>>> = vec![];
        target_candidates.resize_with(test.targets(), Default::default);

        let total_candidate_count = candidates.len();

        // Sort the candidates into the appropriate vector in `target_candidates`. Note that at some
        // point we may encounter a candidate where the test is not relevant; at that point, we stop
        // sorting.
        while let Some(candidate) = candidates.first_mut() {
            let Some(idx) = self.sort_candidate(&match_place, &test, candidate) else {
                break;
            };
            let (candidate, rest) = candidates.split_first_mut().unwrap();
            target_candidates[idx].push(candidate);
            candidates = rest;
        }

        // At least the first candidate ought to be tested
        assert!(
            total_candidate_count > candidates.len(),
            "{total_candidate_count}, {candidates:#?}"
        );
        debug!("tested_candidates: {}", total_candidate_count - candidates.len());
        debug!("untested_candidates: {}", candidates.len());

        (candidates, target_candidates)
    }

    /// This is the most subtle part of the match lowering algorithm. At this point, the input
    /// candidates have been fully simplified, so all remaining match-pairs require some sort of
    /// test.
    ///
    /// Once we pick what sort of test we are going to perform, this test will help us winnow down
    /// our candidates. So we walk over the candidates (from high to low priority) and check. We
    /// compute, for each outcome of the test, a transformed list of candidates. If a candidate
    /// matches in a single branch of our test, we add it to the corresponding outcome. We also
    /// transform it to record the fact that we know which outcome occurred.
    ///
    /// For example, if we are testing `x.0`'s variant, and we have a candidate `(x.0 @ Some(v), x.1
    /// @ 22)`, then we would have a resulting candidate of `((x.0 as Some).0 @ v, x.1 @ 22)` in the
    /// branch corresponding to `Some`. To ensure we make progress, we always pick a test that
    /// results in simplifying the first candidate.
    ///
    /// But there may also be candidates that the test doesn't
    /// apply to. The classical example is wildcards:
    ///
    /// ```
    /// # let (x, y, z) = (true, true, true);
    /// match (x, y, z) {
    ///     (true , _    , true ) => true,  // (0)
    ///     (false, false, _    ) => false, // (1)
    ///     (_    , true , _    ) => true,  // (2)
    ///     (true , _    , false) => false, // (3)
    /// }
    /// # ;
    /// ```
    ///
    /// Here, the traditional "decision tree" method would generate 2 separate code-paths for the 2
    /// possible values of `x`. This would however duplicate some candidates, which would need to be
    /// lowered several times.
    ///
    /// In some cases, this duplication can create an exponential amount of
    /// code. This is most easily seen by noticing that this method terminates
    /// with precisely the reachable arms being reachable - but that problem
    /// is trivially NP-complete:
    ///
    /// ```ignore (illustrative)
    /// match (var0, var1, var2, var3, ...) {
    ///     (true , _   , _    , false, true, ...) => false,
    ///     (_    , true, true , false, _   , ...) => false,
    ///     (false, _   , false, false, _   , ...) => false,
    ///     ...
    ///     _ => true
    /// }
    /// ```
    ///
    /// Here the last arm is reachable only if there is an assignment to
    /// the variables that does not match any of the literals. Therefore,
    /// compilation would take an exponential amount of time in some cases.
    ///
    /// In rustc, we opt instead for the "backtracking automaton" approach. This guarantees we never
    /// duplicate a candidate (except in the presence of or-patterns). In fact this guarantee is
    /// ensured by the fact that we carry around `&mut Candidate`s which can't be duplicated.
    ///
    /// To make this work, whenever we decide to perform a test, if we encounter a candidate that
    /// could match in more than one branch of the test, we stop. We generate code for the test and
    /// for the candidates in its branches; the remaining candidates will be tested if the
    /// candidates in the branches fail to match.
    ///
    /// For example, if we test on `x` in the following:
    /// ```
    /// # fn foo((x, y, z): (bool, bool, bool)) -> u32 {
    /// match (x, y, z) {
    ///     (true , _    , true ) => 0,
    ///     (false, false, _    ) => 1,
    ///     (_    , true , _    ) => 2,
    ///     (true , _    , false) => 3,
    /// }
    /// # }
    /// ```
    /// this function generates code that looks more of less like:
    /// ```
    /// # fn foo((x, y, z): (bool, bool, bool)) -> u32 {
    /// if x {
    ///     match (y, z) {
    ///         (_, true) => return 0,
    ///         _ => {} // continue matching
    ///     }
    /// } else {
    ///     match (y, z) {
    ///         (false, _) => return 1,
    ///         _ => {} // continue matching
    ///     }
    /// }
    /// // the block here is `remainder_start`
    /// match (x, y, z) {
    ///     (_    , true , _    ) => 2,
    ///     (true , _    , false) => 3,
    ///     _ => unreachable!(),
    /// }
    /// # }
    /// ```
    fn test_candidates<'pat, 'b, 'c>(
        &mut self,
        span: Span,
        scrutinee_span: Span,
        candidates: &'b mut [&'c mut Candidate<'pat, 'tcx>],
        start_block: BasicBlock,
        otherwise_block: BasicBlock,
    ) {
        // Extract the match-pair from the highest priority candidate and build a test from it.
        let (match_place, test) = self.pick_test(candidates);

        // For each of the N possible test outcomes, build the vector of candidates that applies if
        // the test has that particular outcome.
        let (remaining_candidates, target_candidates) =
            self.sort_candidates(&match_place, &test, candidates);

        // The block that we should branch to if none of the
        // `target_candidates` match.
        let remainder_start = if !remaining_candidates.is_empty() {
            let remainder_start = self.cfg.start_new_block();
            self.match_candidates(
                span,
                scrutinee_span,
                remainder_start,
                otherwise_block,
                remaining_candidates,
            );
            remainder_start
        } else {
            otherwise_block
        };

        // For each outcome of test, process the candidates that still
        // apply. Collect a list of blocks where control flow will
        // branch if one of the `target_candidate` sets is not
        // exhaustive.
        let target_blocks: Vec<_> = target_candidates
            .into_iter()
            .map(|mut candidates| {
                if !candidates.is_empty() {
                    let candidate_start = self.cfg.start_new_block();
                    self.match_candidates(
                        span,
                        scrutinee_span,
                        candidate_start,
                        remainder_start,
                        &mut *candidates,
                    );
                    candidate_start
                } else {
                    remainder_start
                }
            })
            .collect();

        // Perform the test, branching to one of N blocks.
        self.perform_test(span, scrutinee_span, start_block, &match_place, &test, target_blocks);
    }

    /// Determine the fake borrows that are needed from a set of places that
    /// have to be stable across match guards.
    ///
    /// Returns a list of places that need a fake borrow and the temporary
    /// that's used to store the fake borrow.
    ///
    /// Match exhaustiveness checking is not able to handle the case where the
    /// place being matched on is mutated in the guards. We add "fake borrows"
    /// to the guards that prevent any mutation of the place being matched.
    /// There are a some subtleties:
    ///
    /// 1. Borrowing `*x` doesn't prevent assigning to `x`. If `x` is a shared
    ///    reference, the borrow isn't even tracked. As such we have to add fake
    ///    borrows of any prefixes of a place
    /// 2. We don't want `match x { _ => (), }` to conflict with mutable
    ///    borrows of `x`, so we only add fake borrows for places which are
    ///    bound or tested by the match.
    /// 3. We don't want the fake borrows to conflict with `ref mut` bindings,
    ///    so we use a special BorrowKind for them.
    /// 4. The fake borrows may be of places in inactive variants, so it would
    ///    be UB to generate code for them. They therefore have to be removed
    ///    by a MIR pass run after borrow checking.
    fn calculate_fake_borrows<'b>(
        &mut self,
        fake_borrows: &'b FxIndexSet<Place<'tcx>>,
        temp_span: Span,
    ) -> Vec<(Place<'tcx>, Local)> {
        let tcx = self.tcx;

        debug!("add_fake_borrows fake_borrows = {:?}", fake_borrows);

        let mut all_fake_borrows = Vec::with_capacity(fake_borrows.len());

        // Insert a Shallow borrow of the prefixes of any fake borrows.
        for place in fake_borrows {
            let mut cursor = place.projection.as_ref();
            while let [proj_base @ .., elem] = cursor {
                cursor = proj_base;

                if let ProjectionElem::Deref = elem {
                    // Insert a shallow borrow after a deref. For other
                    // projections the borrow of prefix_cursor will
                    // conflict with any mutation of base.
                    all_fake_borrows.push(PlaceRef { local: place.local, projection: proj_base });
                }
            }

            all_fake_borrows.push(place.as_ref());
        }

        // Deduplicate
        let mut dedup = FxHashSet::default();
        all_fake_borrows.retain(|b| dedup.insert(*b));

        debug!("add_fake_borrows all_fake_borrows = {:?}", all_fake_borrows);

        all_fake_borrows
            .into_iter()
            .map(|matched_place_ref| {
                let matched_place = Place {
                    local: matched_place_ref.local,
                    projection: tcx.mk_place_elems(matched_place_ref.projection),
                };
                let fake_borrow_deref_ty = matched_place.ty(&self.local_decls, tcx).ty;
                let fake_borrow_ty =
                    Ty::new_imm_ref(tcx, tcx.lifetimes.re_erased, fake_borrow_deref_ty);
                let mut fake_borrow_temp = LocalDecl::new(fake_borrow_ty, temp_span);
                fake_borrow_temp.local_info = ClearCrossCrate::Set(Box::new(LocalInfo::FakeBorrow));
                let fake_borrow_temp = self.local_decls.push(fake_borrow_temp);

                (matched_place, fake_borrow_temp)
            })
            .collect()
    }
}

///////////////////////////////////////////////////////////////////////////
// Pat binding - used for `let` and function parameters as well.

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// If the bindings have already been declared, set `declare_bindings` to
    /// `false` to avoid duplicated bindings declaration. Used for if-let guards.
    pub(crate) fn lower_let_expr(
        &mut self,
        mut block: BasicBlock,
        expr_id: ExprId,
        pat: &Pat<'tcx>,
        source_scope: Option<SourceScope>,
        span: Span,
        declare_bindings: bool,
    ) -> BlockAnd<()> {
        let expr_span = self.thir[expr_id].span;
        let expr_place_builder = unpack!(block = self.lower_scrutinee(block, expr_id, expr_span));
        let wildcard = Pat::wildcard_from_ty(pat.ty);
        let mut guard_candidate = Candidate::new(expr_place_builder.clone(), pat, false, self);
        let mut otherwise_candidate =
            Candidate::new(expr_place_builder.clone(), &wildcard, false, self);
        let fake_borrow_temps = self.lower_match_tree(
            block,
            pat.span,
            &expr_place_builder,
            pat.span,
            false,
            &mut [&mut guard_candidate, &mut otherwise_candidate],
        );
        let expr_place = expr_place_builder.try_to_place(self);
        let opt_expr_place = expr_place.as_ref().map(|place| (Some(place), expr_span));
        let otherwise_post_guard_block = otherwise_candidate.pre_binding_block.unwrap();
        self.break_for_else(otherwise_post_guard_block, self.source_info(expr_span));

        if declare_bindings {
            self.declare_bindings(source_scope, pat.span.to(span), pat, None, opt_expr_place);
        }

        let post_guard_block = self.bind_pattern(
            self.source_info(pat.span),
            guard_candidate,
            fake_borrow_temps.as_slice(),
            expr_span,
            None,
            false,
        );

        post_guard_block.unit()
    }

    /// Initializes each of the bindings from the candidate by
    /// moving/copying/ref'ing the source as appropriate. Tests the guard, if
    /// any, and then branches to the arm. Returns the block for the case where
    /// the guard succeeds.
    ///
    /// Note: we do not check earlier that if there is a guard,
    /// there cannot be move bindings. We avoid a use-after-move by only
    /// moving the binding once the guard has evaluated to true (see below).
    fn bind_and_guard_matched_candidate<'pat>(
        &mut self,
        candidate: Candidate<'pat, 'tcx>,
        parent_bindings: &[(Vec<Binding<'tcx>>, Vec<Ascription<'tcx>>)],
        fake_borrows: &[(Place<'tcx>, Local)],
        scrutinee_span: Span,
        arm_match_scope: Option<(&Arm<'tcx>, region::Scope)>,
        schedule_drops: bool,
        storages_alive: bool,
    ) -> BasicBlock {
        debug!("bind_and_guard_matched_candidate(candidate={:?})", candidate);

        debug_assert!(candidate.match_pairs.is_empty());

        let candidate_source_info = self.source_info(candidate.span);

        let mut block = candidate.pre_binding_block.unwrap();

        if candidate.next_candidate_pre_binding_block.is_some() {
            let fresh_block = self.cfg.start_new_block();
            self.false_edges(
                block,
                fresh_block,
                candidate.next_candidate_pre_binding_block,
                candidate_source_info,
            );
            block = fresh_block;
        }

        self.ascribe_types(
            block,
            parent_bindings
                .iter()
                .flat_map(|(_, ascriptions)| ascriptions)
                .cloned()
                .chain(candidate.ascriptions),
        );

        // rust-lang/rust#27282: The `autoref` business deserves some
        // explanation here.
        //
        // The intent of the `autoref` flag is that when it is true,
        // then any pattern bindings of type T will map to a `&T`
        // within the context of the guard expression, but will
        // continue to map to a `T` in the context of the arm body. To
        // avoid surfacing this distinction in the user source code
        // (which would be a severe change to the language and require
        // far more revision to the compiler), when `autoref` is true,
        // then any occurrence of the identifier in the guard
        // expression will automatically get a deref op applied to it.
        //
        // So an input like:
        //
        // ```
        // let place = Foo::new();
        // match place { foo if inspect(foo)
        //     => feed(foo), ... }
        // ```
        //
        // will be treated as if it were really something like:
        //
        // ```
        // let place = Foo::new();
        // match place { Foo { .. } if { let tmp1 = &place; inspect(*tmp1) }
        //     => { let tmp2 = place; feed(tmp2) }, ... }
        // ```
        //
        // And an input like:
        //
        // ```
        // let place = Foo::new();
        // match place { ref mut foo if inspect(foo)
        //     => feed(foo), ... }
        // ```
        //
        // will be treated as if it were really something like:
        //
        // ```
        // let place = Foo::new();
        // match place { Foo { .. } if { let tmp1 = & &mut place; inspect(*tmp1) }
        //     => { let tmp2 = &mut place; feed(tmp2) }, ... }
        // ```
        //
        // In short, any pattern binding will always look like *some*
        // kind of `&T` within the guard at least in terms of how the
        // MIR-borrowck views it, and this will ensure that guard
        // expressions cannot mutate their the match inputs via such
        // bindings. (It also ensures that guard expressions can at
        // most *copy* values from such bindings; non-Copy things
        // cannot be moved via pattern bindings in guard expressions.)
        //
        // ----
        //
        // Implementation notes (under assumption `autoref` is true).
        //
        // To encode the distinction above, we must inject the
        // temporaries `tmp1` and `tmp2`.
        //
        // There are two cases of interest: binding by-value, and binding by-ref.
        //
        // 1. Binding by-value: Things are simple.
        //
        //    * Establishing `tmp1` creates a reference into the
        //      matched place. This code is emitted by
        //      bind_matched_candidate_for_guard.
        //
        //    * `tmp2` is only initialized "lazily", after we have
        //      checked the guard. Thus, the code that can trigger
        //      moves out of the candidate can only fire after the
        //      guard evaluated to true. This initialization code is
        //      emitted by bind_matched_candidate_for_arm.
        //
        // 2. Binding by-reference: Things are tricky.
        //
        //    * Here, the guard expression wants a `&&` or `&&mut`
        //      into the original input. This means we need to borrow
        //      the reference that we create for the arm.
        //    * So we eagerly create the reference for the arm and then take a
        //      reference to that.
        if let Some((arm, match_scope)) = arm_match_scope
            && let Some(guard) = arm.guard
        {
            let tcx = self.tcx;
            let bindings = parent_bindings
                .iter()
                .flat_map(|(bindings, _)| bindings)
                .chain(&candidate.bindings);

            self.bind_matched_candidate_for_guard(block, schedule_drops, bindings.clone());
            let guard_frame = GuardFrame {
                locals: bindings.map(|b| GuardFrameLocal::new(b.var_id, b.binding_mode)).collect(),
            };
            debug!("entering guard building context: {:?}", guard_frame);
            self.guard_context.push(guard_frame);

            let re_erased = tcx.lifetimes.re_erased;
            let scrutinee_source_info = self.source_info(scrutinee_span);
            for &(place, temp) in fake_borrows {
                let borrow = Rvalue::Ref(re_erased, BorrowKind::Fake, place);
                self.cfg.push_assign(block, scrutinee_source_info, Place::from(temp), borrow);
            }

            let mut guard_span = rustc_span::DUMMY_SP;

            let (post_guard_block, otherwise_post_guard_block) =
                self.in_if_then_scope(match_scope, guard_span, |this| {
                    guard_span = this.thir[guard].span;
                    this.then_else_break(
                        block,
                        guard,
                        None, // Use `self.local_scope()` as the temp scope
                        this.source_info(arm.span),
                        false, // For guards, `let` bindings are declared separately
                    )
                });

            let source_info = self.source_info(guard_span);
            let guard_end = self.source_info(tcx.sess.source_map().end_point(guard_span));
            let guard_frame = self.guard_context.pop().unwrap();
            debug!("Exiting guard building context with locals: {:?}", guard_frame);

            for &(_, temp) in fake_borrows {
                let cause = FakeReadCause::ForMatchGuard;
                self.cfg.push_fake_read(post_guard_block, guard_end, cause, Place::from(temp));
            }

            let otherwise_block = candidate.otherwise_block.unwrap_or_else(|| {
                let unreachable = self.cfg.start_new_block();
                self.cfg.terminate(unreachable, source_info, TerminatorKind::Unreachable);
                unreachable
            });
            self.false_edges(
                otherwise_post_guard_block,
                otherwise_block,
                candidate.next_candidate_pre_binding_block,
                source_info,
            );

            // We want to ensure that the matched candidates are bound
            // after we have confirmed this candidate *and* any
            // associated guard; Binding them on `block` is too soon,
            // because that would be before we've checked the result
            // from the guard.
            //
            // But binding them on the arm is *too late*, because
            // then all of the candidates for a single arm would be
            // bound in the same place, that would cause a case like:
            //
            // ```rust
            // match (30, 2) {
            //     (mut x, 1) | (2, mut x) if { true } => { ... }
            //     ...                                 // ^^^^^^^ (this is `arm_block`)
            // }
            // ```
            //
            // would yield an `arm_block` something like:
            //
            // ```
            // StorageLive(_4);        // _4 is `x`
            // _4 = &mut (_1.0: i32);  // this is handling `(mut x, 1)` case
            // _4 = &mut (_1.1: i32);  // this is handling `(2, mut x)` case
            // ```
            //
            // and that is clearly not correct.
            let by_value_bindings = parent_bindings
                .iter()
                .flat_map(|(bindings, _)| bindings)
                .chain(&candidate.bindings)
                .filter(|binding| matches!(binding.binding_mode, BindingMode::ByValue));
            // Read all of the by reference bindings to ensure that the
            // place they refer to can't be modified by the guard.
            for binding in by_value_bindings.clone() {
                let local_id = self.var_local_id(binding.var_id, RefWithinGuard);
                let cause = FakeReadCause::ForGuardBinding;
                self.cfg.push_fake_read(post_guard_block, guard_end, cause, Place::from(local_id));
            }
            assert!(schedule_drops, "patterns with guards must schedule drops");
            self.bind_matched_candidate_for_arm_body(
                post_guard_block,
                true,
                by_value_bindings,
                storages_alive,
            );

            post_guard_block
        } else {
            // (Here, it is not too early to bind the matched
            // candidate on `block`, because there is no guard result
            // that we have to inspect before we bind them.)
            self.bind_matched_candidate_for_arm_body(
                block,
                schedule_drops,
                parent_bindings
                    .iter()
                    .flat_map(|(bindings, _)| bindings)
                    .chain(&candidate.bindings),
                storages_alive,
            );
            block
        }
    }

    /// Append `AscribeUserType` statements onto the end of `block`
    /// for each ascription
    fn ascribe_types(
        &mut self,
        block: BasicBlock,
        ascriptions: impl IntoIterator<Item = Ascription<'tcx>>,
    ) {
        for ascription in ascriptions {
            let source_info = self.source_info(ascription.annotation.span);

            let base = self.canonical_user_type_annotations.push(ascription.annotation);
            self.cfg.push(
                block,
                Statement {
                    source_info,
                    kind: StatementKind::AscribeUserType(
                        Box::new((
                            ascription.source,
                            UserTypeProjection { base, projs: Vec::new() },
                        )),
                        ascription.variance,
                    ),
                },
            );
        }
    }

    fn bind_matched_candidate_for_guard<'b>(
        &mut self,
        block: BasicBlock,
        schedule_drops: bool,
        bindings: impl IntoIterator<Item = &'b Binding<'tcx>>,
    ) where
        'tcx: 'b,
    {
        debug!("bind_matched_candidate_for_guard(block={:?})", block);

        // Assign each of the bindings. Since we are binding for a
        // guard expression, this will never trigger moves out of the
        // candidate.
        let re_erased = self.tcx.lifetimes.re_erased;
        for binding in bindings {
            debug!("bind_matched_candidate_for_guard(binding={:?})", binding);
            let source_info = self.source_info(binding.span);

            // For each pattern ident P of type T, `ref_for_guard` is
            // a reference R: &T pointing to the location matched by
            // the pattern, and every occurrence of P within a guard
            // denotes *R.
            let ref_for_guard = self.storage_live_binding(
                block,
                binding.var_id,
                binding.span,
                RefWithinGuard,
                schedule_drops,
            );
            match binding.binding_mode {
                BindingMode::ByValue => {
                    let rvalue = Rvalue::Ref(re_erased, BorrowKind::Shared, binding.source);
                    self.cfg.push_assign(block, source_info, ref_for_guard, rvalue);
                }
                BindingMode::ByRef(borrow_kind) => {
                    let value_for_arm = self.storage_live_binding(
                        block,
                        binding.var_id,
                        binding.span,
                        OutsideGuard,
                        schedule_drops,
                    );

                    let rvalue = Rvalue::Ref(re_erased, borrow_kind, binding.source);
                    self.cfg.push_assign(block, source_info, value_for_arm, rvalue);
                    let rvalue = Rvalue::Ref(re_erased, BorrowKind::Shared, value_for_arm);
                    self.cfg.push_assign(block, source_info, ref_for_guard, rvalue);
                }
            }
        }
    }

    fn bind_matched_candidate_for_arm_body<'b>(
        &mut self,
        block: BasicBlock,
        schedule_drops: bool,
        bindings: impl IntoIterator<Item = &'b Binding<'tcx>>,
        storages_alive: bool,
    ) where
        'tcx: 'b,
    {
        debug!("bind_matched_candidate_for_arm_body(block={:?})", block);

        let re_erased = self.tcx.lifetimes.re_erased;
        // Assign each of the bindings. This may trigger moves out of the candidate.
        for binding in bindings {
            let source_info = self.source_info(binding.span);
            let local = if storages_alive {
                // Here storages are already alive, probably because this is a binding
                // from let-else.
                // We just need to schedule drop for the value.
                self.var_local_id(binding.var_id, OutsideGuard).into()
            } else {
                self.storage_live_binding(
                    block,
                    binding.var_id,
                    binding.span,
                    OutsideGuard,
                    schedule_drops,
                )
            };
            if schedule_drops {
                self.schedule_drop_for_binding(binding.var_id, binding.span, OutsideGuard);
            }
            let rvalue = match binding.binding_mode {
                BindingMode::ByValue => Rvalue::Use(self.consume_by_copy_or_move(binding.source)),
                BindingMode::ByRef(borrow_kind) => {
                    Rvalue::Ref(re_erased, borrow_kind, binding.source)
                }
            };
            self.cfg.push_assign(block, source_info, local, rvalue);
        }
    }

    /// Each binding (`ref mut var`/`ref var`/`mut var`/`var`, where the bound
    /// `var` has type `T` in the arm body) in a pattern maps to 2 locals. The
    /// first local is a binding for occurrences of `var` in the guard, which
    /// will have type `&T`. The second local is a binding for occurrences of
    /// `var` in the arm body, which will have type `T`.
    #[instrument(skip(self), level = "debug")]
    fn declare_binding(
        &mut self,
        source_info: SourceInfo,
        visibility_scope: SourceScope,
        mutability: Mutability,
        name: Symbol,
        mode: BindingMode,
        var_id: LocalVarId,
        var_ty: Ty<'tcx>,
        user_ty: UserTypeProjections,
        has_guard: ArmHasGuard,
        opt_match_place: Option<(Option<Place<'tcx>>, Span)>,
        pat_span: Span,
    ) {
        let tcx = self.tcx;
        let debug_source_info = SourceInfo { span: source_info.span, scope: visibility_scope };
        let binding_mode = match mode {
            BindingMode::ByValue => ty::BindingMode::BindByValue(mutability),
            BindingMode::ByRef(_) => ty::BindingMode::BindByReference(mutability),
        };
        let local = LocalDecl {
            mutability,
            ty: var_ty,
            user_ty: if user_ty.is_empty() { None } else { Some(Box::new(user_ty)) },
            source_info,
            local_info: ClearCrossCrate::Set(Box::new(LocalInfo::User(BindingForm::Var(
                VarBindingForm {
                    binding_mode,
                    // hypothetically, `visit_primary_bindings` could try to unzip
                    // an outermost hir::Ty as we descend, matching up
                    // idents in pat; but complex w/ unclear UI payoff.
                    // Instead, just abandon providing diagnostic info.
                    opt_ty_info: None,
                    opt_match_place,
                    pat_span,
                },
            )))),
        };
        let for_arm_body = self.local_decls.push(local);
        self.var_debug_info.push(VarDebugInfo {
            name,
            source_info: debug_source_info,
            value: VarDebugInfoContents::Place(for_arm_body.into()),
            composite: None,
            argument_index: None,
        });
        let locals = if has_guard.0 {
            let ref_for_guard = self.local_decls.push(LocalDecl::<'tcx> {
                // This variable isn't mutated but has a name, so has to be
                // immutable to avoid the unused mut lint.
                mutability: Mutability::Not,
                ty: Ty::new_imm_ref(tcx, tcx.lifetimes.re_erased, var_ty),
                user_ty: None,
                source_info,
                local_info: ClearCrossCrate::Set(Box::new(LocalInfo::User(
                    BindingForm::RefForGuard,
                ))),
            });
            self.var_debug_info.push(VarDebugInfo {
                name,
                source_info: debug_source_info,
                value: VarDebugInfoContents::Place(ref_for_guard.into()),
                composite: None,
                argument_index: None,
            });
            LocalsForNode::ForGuard { ref_for_guard, for_arm_body }
        } else {
            LocalsForNode::One(for_arm_body)
        };
        debug!(?locals);
        self.var_indices.insert(var_id, locals);
    }

    pub(crate) fn ast_let_else(
        &mut self,
        mut block: BasicBlock,
        init_id: ExprId,
        initializer_span: Span,
        else_block: BlockId,
        let_else_scope: &region::Scope,
        pattern: &Pat<'tcx>,
    ) -> BlockAnd<BasicBlock> {
        let else_block_span = self.thir[else_block].span;
        let (matching, failure) = self.in_if_then_scope(*let_else_scope, else_block_span, |this| {
            let scrutinee = unpack!(block = this.lower_scrutinee(block, init_id, initializer_span));
            let pat = Pat { ty: pattern.ty, span: else_block_span, kind: PatKind::Wild };
            let mut wildcard = Candidate::new(scrutinee.clone(), &pat, false, this);
            let mut candidate = Candidate::new(scrutinee.clone(), pattern, false, this);
            let fake_borrow_temps = this.lower_match_tree(
                block,
                initializer_span,
                &scrutinee,
                pattern.span,
                false,
                &mut [&mut candidate, &mut wildcard],
            );
            // This block is for the matching case
            let matching = this.bind_pattern(
                this.source_info(pattern.span),
                candidate,
                fake_borrow_temps.as_slice(),
                initializer_span,
                None,
                true,
            );
            // This block is for the failure case
            let failure = this.bind_pattern(
                this.source_info(else_block_span),
                wildcard,
                fake_borrow_temps.as_slice(),
                initializer_span,
                None,
                true,
            );
            this.break_for_else(failure, this.source_info(initializer_span));
            matching.unit()
        });
        matching.and(failure)
    }
}
