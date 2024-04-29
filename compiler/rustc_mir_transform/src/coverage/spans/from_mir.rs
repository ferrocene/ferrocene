use rustc_data_structures::captures::Captures;
use rustc_data_structures::fx::FxHashSet;
use rustc_index::IndexVec;
use rustc_middle::mir::coverage::{
    BlockMarkerId, BranchSpan, CoverageKind, MCDCBranchSpan, MCDCDecisionSpan,
};
use rustc_middle::mir::{
    self, AggregateKind, BasicBlock, FakeReadCause, Rvalue, Statement, StatementKind, Terminator,
    TerminatorKind,
};
use rustc_span::{ExpnKind, MacroKind, Span, Symbol};

use crate::coverage::graph::{
    BasicCoverageBlock, BasicCoverageBlockData, CoverageGraph, START_BCB,
};
use crate::coverage::spans::{BcbBranchPair, BcbMapping, BcbMappingKind};
use crate::coverage::ExtractedHirInfo;

/// Traverses the MIR body to produce an initial collection of coverage-relevant
/// spans, each associated with a node in the coverage graph (BCB) and possibly
/// other metadata.
///
/// The returned spans are sorted in a specific order that is expected by the
/// subsequent span-refinement step.
pub(super) fn mir_to_initial_sorted_coverage_spans(
    mir_body: &mir::Body<'_>,
    hir_info: &ExtractedHirInfo,
    basic_coverage_blocks: &CoverageGraph,
) -> Vec<SpanFromMir> {
    let &ExtractedHirInfo { body_span, .. } = hir_info;

    let mut initial_spans = vec![];

    for (bcb, bcb_data) in basic_coverage_blocks.iter_enumerated() {
        initial_spans.extend(bcb_to_initial_coverage_spans(mir_body, body_span, bcb, bcb_data));
    }

    // Only add the signature span if we found at least one span in the body.
    if !initial_spans.is_empty() {
        // If there is no usable signature span, add a fake one (before refinement)
        // to avoid an ugly gap between the body start and the first real span.
        // FIXME: Find a more principled way to solve this problem.
        let fn_sig_span = hir_info.fn_sig_span_extended.unwrap_or_else(|| body_span.shrink_to_lo());
        initial_spans.push(SpanFromMir::for_fn_sig(fn_sig_span));
    }

    initial_spans.sort_by(|a, b| basic_coverage_blocks.cmp_in_dominator_order(a.bcb, b.bcb));
    remove_unwanted_macro_spans(&mut initial_spans);
    split_visible_macro_spans(&mut initial_spans);

    initial_spans.sort_by(|a, b| {
        // First sort by span start.
        Ord::cmp(&a.span.lo(), &b.span.lo())
            // If span starts are the same, sort by span end in reverse order.
            // This ensures that if spans A and B are adjacent in the list,
            // and they overlap but are not equal, then either:
            // - Span A extends further left, or
            // - Both have the same start and span A extends further right
            .then_with(|| Ord::cmp(&a.span.hi(), &b.span.hi()).reverse())
            // If two spans have the same lo & hi, put hole spans first,
            // as they take precedence over non-hole spans.
            .then_with(|| Ord::cmp(&a.is_hole, &b.is_hole).reverse())
            // After deduplication, we want to keep only the most-dominated BCB.
            .then_with(|| basic_coverage_blocks.cmp_in_dominator_order(a.bcb, b.bcb).reverse())
    });

    // Among covspans with the same span, keep only one. Hole spans take
    // precedence, otherwise keep the one with the most-dominated BCB.
    // (Ideally we should try to preserve _all_ non-dominating BCBs, but that
    // requires a lot more complexity in the span refiner, for little benefit.)
    initial_spans.dedup_by(|b, a| a.span.source_equal(b.span));

    initial_spans
}

/// Macros that expand into branches (e.g. `assert!`, `trace!`) tend to generate
/// multiple condition/consequent blocks that have the span of the whole macro
/// invocation, which is unhelpful. Keeping only the first such span seems to
/// give better mappings, so remove the others.
///
/// (The input spans should be sorted in BCB dominator order, so that the
/// retained "first" span is likely to dominate the others.)
fn remove_unwanted_macro_spans(initial_spans: &mut Vec<SpanFromMir>) {
    let mut seen_macro_spans = FxHashSet::default();
    initial_spans.retain(|covspan| {
        // Ignore (retain) hole spans and non-macro-expansion spans.
        if covspan.is_hole || covspan.visible_macro.is_none() {
            return true;
        }

        // Retain only the first macro-expanded covspan with this span.
        seen_macro_spans.insert(covspan.span)
    });
}

/// When a span corresponds to a macro invocation that is visible from the
/// function body, split it into two parts. The first part covers just the
/// macro name plus `!`, and the second part covers the rest of the macro
/// invocation. This seems to give better results for code that uses macros.
fn split_visible_macro_spans(initial_spans: &mut Vec<SpanFromMir>) {
    let mut extra_spans = vec![];

    initial_spans.retain(|covspan| {
        if covspan.is_hole {
            return true;
        }

        let Some(visible_macro) = covspan.visible_macro else { return true };

        let split_len = visible_macro.as_str().len() as u32 + 1;
        let (before, after) = covspan.span.split_at(split_len);
        if !covspan.span.contains(before) || !covspan.span.contains(after) {
            // Something is unexpectedly wrong with the split point.
            // The debug assertion in `split_at` will have already caught this,
            // but in release builds it's safer to do nothing and maybe get a
            // bug report for unexpected coverage, rather than risk an ICE.
            return true;
        }

        assert!(!covspan.is_hole);
        extra_spans.push(SpanFromMir::new(before, covspan.visible_macro, covspan.bcb, false));
        extra_spans.push(SpanFromMir::new(after, covspan.visible_macro, covspan.bcb, false));
        false // Discard the original covspan that we just split.
    });

    // The newly-split spans are added at the end, so any previous sorting
    // is not preserved.
    initial_spans.extend(extra_spans);
}

// Generate a set of coverage spans from the filtered set of `Statement`s and `Terminator`s of
// the `BasicBlock`(s) in the given `BasicCoverageBlockData`. One coverage span is generated
// for each `Statement` and `Terminator`. (Note that subsequent stages of coverage analysis will
// merge some coverage spans, at which point a coverage span may represent multiple
// `Statement`s and/or `Terminator`s.)
fn bcb_to_initial_coverage_spans<'a, 'tcx>(
    mir_body: &'a mir::Body<'tcx>,
    body_span: Span,
    bcb: BasicCoverageBlock,
    bcb_data: &'a BasicCoverageBlockData,
) -> impl Iterator<Item = SpanFromMir> + Captures<'a> + Captures<'tcx> {
    bcb_data.basic_blocks.iter().flat_map(move |&bb| {
        let data = &mir_body[bb];

        let unexpand = move |expn_span| {
            unexpand_into_body_span_with_visible_macro(expn_span, body_span)
                // Discard any spans that fill the entire body, because they tend
                // to represent compiler-inserted code, e.g. implicitly returning `()`.
                .filter(|(span, _)| !span.source_equal(body_span))
        };

        let statement_spans = data.statements.iter().filter_map(move |statement| {
            let expn_span = filtered_statement_span(statement)?;
            let (span, visible_macro) = unexpand(expn_span)?;

            // A statement that looks like the assignment of a closure expression
            // is treated as a "hole" span, to be carved out of other spans.
            Some(SpanFromMir::new(span, visible_macro, bcb, is_closure_like(statement)))
        });

        let terminator_span = Some(data.terminator()).into_iter().filter_map(move |terminator| {
            let expn_span = filtered_terminator_span(terminator)?;
            let (span, visible_macro) = unexpand(expn_span)?;

            Some(SpanFromMir::new(span, visible_macro, bcb, false))
        });

        statement_spans.chain(terminator_span)
    })
}

fn is_closure_like(statement: &Statement<'_>) -> bool {
    match statement.kind {
        StatementKind::Assign(box (_, Rvalue::Aggregate(box ref agg_kind, _))) => match agg_kind {
            AggregateKind::Closure(_, _)
            | AggregateKind::Coroutine(_, _)
            | AggregateKind::CoroutineClosure(..) => true,
            _ => false,
        },
        _ => false,
    }
}

/// If the MIR `Statement` has a span contributive to computing coverage spans,
/// return it; otherwise return `None`.
fn filtered_statement_span(statement: &Statement<'_>) -> Option<Span> {
    match statement.kind {
        // These statements have spans that are often outside the scope of the executed source code
        // for their parent `BasicBlock`.
        StatementKind::StorageLive(_)
        | StatementKind::StorageDead(_)
        | StatementKind::ConstEvalCounter
        | StatementKind::Nop => None,

        // FIXME(#78546): MIR InstrumentCoverage - Can the source_info.span for `FakeRead`
        // statements be more consistent?
        //
        // FakeReadCause::ForGuardBinding, in this example:
        //     match somenum {
        //         x if x < 1 => { ... }
        //     }...
        // The BasicBlock within the match arm code included one of these statements, but the span
        // for it covered the `1` in this source. The actual statements have nothing to do with that
        // source span:
        //     FakeRead(ForGuardBinding, _4);
        // where `_4` is:
        //     _4 = &_1; (at the span for the first `x`)
        // and `_1` is the `Place` for `somenum`.
        //
        // If and when the Issue is resolved, remove this special case match pattern:
        StatementKind::FakeRead(box (FakeReadCause::ForGuardBinding, _)) => None,

        // Retain spans from most other statements.
        StatementKind::FakeRead(_)
        | StatementKind::Intrinsic(..)
        | StatementKind::Coverage(
            // The purpose of `SpanMarker` is to be matched and accepted here.
            CoverageKind::SpanMarker,
        )
        | StatementKind::Assign(_)
        | StatementKind::SetDiscriminant { .. }
        | StatementKind::Deinit(..)
        | StatementKind::Retag(_, _)
        | StatementKind::PlaceMention(..)
        | StatementKind::AscribeUserType(_, _) => Some(statement.source_info.span),

        // Block markers are used for branch coverage, so ignore them here.
        StatementKind::Coverage(CoverageKind::BlockMarker { .. }) => None,

        // These coverage statements should not exist prior to coverage instrumentation.
        StatementKind::Coverage(
            CoverageKind::CounterIncrement { .. }
            | CoverageKind::ExpressionUsed { .. }
            | CoverageKind::CondBitmapUpdate { .. }
            | CoverageKind::TestVectorBitmapUpdate { .. },
        ) => bug!(
            "Unexpected coverage statement found during coverage instrumentation: {statement:?}"
        ),
    }
}

/// If the MIR `Terminator` has a span contributive to computing coverage spans,
/// return it; otherwise return `None`.
fn filtered_terminator_span(terminator: &Terminator<'_>) -> Option<Span> {
    match terminator.kind {
        // These terminators have spans that don't positively contribute to computing a reasonable
        // span of actually executed source code. (For example, SwitchInt terminators extracted from
        // an `if condition { block }` has a span that includes the executed block, if true,
        // but for coverage, the code region executed, up to *and* through the SwitchInt,
        // actually stops before the if's block.)
        TerminatorKind::Unreachable // Unreachable blocks are not connected to the MIR CFG
        | TerminatorKind::Assert { .. }
        | TerminatorKind::Drop { .. }
        | TerminatorKind::SwitchInt { .. }
        // For `FalseEdge`, only the `real` branch is taken, so it is similar to a `Goto`.
        | TerminatorKind::FalseEdge { .. }
        | TerminatorKind::Goto { .. } => None,

        // Call `func` operand can have a more specific span when part of a chain of calls
        | TerminatorKind::Call { ref func, .. } => {
            let mut span = terminator.source_info.span;
            if let mir::Operand::Constant(box constant) = func {
                if constant.span.lo() > span.lo() {
                    span = span.with_lo(constant.span.lo());
                }
            }
            Some(span)
        }

        // Retain spans from all other terminators
        TerminatorKind::UnwindResume
        | TerminatorKind::UnwindTerminate(_)
        | TerminatorKind::Return
        | TerminatorKind::Yield { .. }
        | TerminatorKind::CoroutineDrop
        | TerminatorKind::FalseUnwind { .. }
        | TerminatorKind::InlineAsm { .. } => {
            Some(terminator.source_info.span)
        }
    }
}

/// Returns an extrapolated span (pre-expansion[^1]) corresponding to a range
/// within the function's body source. This span is guaranteed to be contained
/// within, or equal to, the `body_span`. If the extrapolated span is not
/// contained within the `body_span`, `None` is returned.
///
/// [^1]Expansions result from Rust syntax including macros, syntactic sugar,
/// etc.).
fn unexpand_into_body_span_with_visible_macro(
    original_span: Span,
    body_span: Span,
) -> Option<(Span, Option<Symbol>)> {
    let (span, prev) = unexpand_into_body_span_with_prev(original_span, body_span)?;

    let visible_macro = prev
        .map(|prev| match prev.ctxt().outer_expn_data().kind {
            ExpnKind::Macro(MacroKind::Bang, name) => Some(name),
            _ => None,
        })
        .flatten();

    Some((span, visible_macro))
}

/// Walks through the expansion ancestors of `original_span` to find a span that
/// is contained in `body_span` and has the same [`SyntaxContext`] as `body_span`.
/// The ancestor that was traversed just before the matching span (if any) is
/// also returned.
///
/// For example, a return value of `Some((ancestor, Some(prev))` means that:
/// - `ancestor == original_span.find_ancestor_inside_same_ctxt(body_span)`
/// - `ancestor == prev.parent_callsite()`
///
/// [`SyntaxContext`]: rustc_span::SyntaxContext
fn unexpand_into_body_span_with_prev(
    original_span: Span,
    body_span: Span,
) -> Option<(Span, Option<Span>)> {
    let mut prev = None;
    let mut curr = original_span;

    while !body_span.contains(curr) || !curr.eq_ctxt(body_span) {
        prev = Some(curr);
        curr = curr.parent_callsite()?;
    }

    debug_assert_eq!(Some(curr), original_span.find_ancestor_in_same_ctxt(body_span));
    if let Some(prev) = prev {
        debug_assert_eq!(Some(curr), prev.parent_callsite());
    }

    Some((curr, prev))
}

#[derive(Debug)]
pub(super) struct SpanFromMir {
    /// A span that has been extracted from MIR and then "un-expanded" back to
    /// within the current function's `body_span`. After various intermediate
    /// processing steps, this span is emitted as part of the final coverage
    /// mappings.
    ///
    /// With the exception of `fn_sig_span`, this should always be contained
    /// within `body_span`.
    pub(super) span: Span,
    visible_macro: Option<Symbol>,
    pub(super) bcb: BasicCoverageBlock,
    /// If true, this covspan represents a "hole" that should be carved out
    /// from other spans, e.g. because it represents a closure expression that
    /// will be instrumented separately as its own function.
    pub(super) is_hole: bool,
}

impl SpanFromMir {
    fn for_fn_sig(fn_sig_span: Span) -> Self {
        Self::new(fn_sig_span, None, START_BCB, false)
    }

    fn new(
        span: Span,
        visible_macro: Option<Symbol>,
        bcb: BasicCoverageBlock,
        is_hole: bool,
    ) -> Self {
        Self { span, visible_macro, bcb, is_hole }
    }
}

fn resolve_block_markers(
    branch_info: &mir::coverage::BranchInfo,
    mir_body: &mir::Body<'_>,
) -> IndexVec<BlockMarkerId, Option<BasicBlock>> {
    let mut block_markers = IndexVec::<BlockMarkerId, Option<BasicBlock>>::from_elem_n(
        None,
        branch_info.num_block_markers,
    );

    // Fill out the mapping from block marker IDs to their enclosing blocks.
    for (bb, data) in mir_body.basic_blocks.iter_enumerated() {
        for statement in &data.statements {
            if let StatementKind::Coverage(CoverageKind::BlockMarker { id }) = statement.kind {
                block_markers[id] = Some(bb);
            }
        }
    }

    block_markers
}

// FIXME: There is currently a lot of redundancy between
// `extract_branch_pairs` and `extract_mcdc_mappings`. This is needed so
// that they can each be modified without interfering with the other, but in
// the long term we should try to bring them together again when branch coverage
// and MC/DC coverage support are more mature.

pub(super) fn extract_branch_pairs(
    mir_body: &mir::Body<'_>,
    hir_info: &ExtractedHirInfo,
    basic_coverage_blocks: &CoverageGraph,
) -> Vec<BcbBranchPair> {
    let Some(branch_info) = mir_body.coverage_branch_info.as_deref() else { return vec![] };

    let block_markers = resolve_block_markers(branch_info, mir_body);

    branch_info
        .branch_spans
        .iter()
        .filter_map(|&BranchSpan { span: raw_span, true_marker, false_marker }| {
            // For now, ignore any branch span that was introduced by
            // expansion. This makes things like assert macros less noisy.
            if !raw_span.ctxt().outer_expn_data().is_root() {
                return None;
            }
            let (span, _) =
                unexpand_into_body_span_with_visible_macro(raw_span, hir_info.body_span)?;

            let bcb_from_marker =
                |marker: BlockMarkerId| basic_coverage_blocks.bcb_from_bb(block_markers[marker]?);

            let true_bcb = bcb_from_marker(true_marker)?;
            let false_bcb = bcb_from_marker(false_marker)?;

            Some(BcbBranchPair { span, true_bcb, false_bcb })
        })
        .collect::<Vec<_>>()
}

pub(super) fn extract_mcdc_mappings(
    mir_body: &mir::Body<'_>,
    body_span: Span,
    basic_coverage_blocks: &CoverageGraph,
) -> Vec<BcbMapping> {
    let Some(branch_info) = mir_body.coverage_branch_info.as_deref() else {
        return vec![];
    };

    let block_markers = resolve_block_markers(branch_info, mir_body);

    let bcb_from_marker =
        |marker: BlockMarkerId| basic_coverage_blocks.bcb_from_bb(block_markers[marker]?);

    let check_branch_bcb =
        |raw_span: Span, true_marker: BlockMarkerId, false_marker: BlockMarkerId| {
            // For now, ignore any branch span that was introduced by
            // expansion. This makes things like assert macros less noisy.
            if !raw_span.ctxt().outer_expn_data().is_root() {
                return None;
            }
            let (span, _) = unexpand_into_body_span_with_visible_macro(raw_span, body_span)?;

            let true_bcb = bcb_from_marker(true_marker)?;
            let false_bcb = bcb_from_marker(false_marker)?;
            Some((span, true_bcb, false_bcb))
        };

    let mcdc_branch_filter_map = |&MCDCBranchSpan {
                                      span: raw_span,
                                      true_marker,
                                      false_marker,
                                      condition_info,
                                      decision_depth,
                                  }| {
        check_branch_bcb(raw_span, true_marker, false_marker).map(|(span, true_bcb, false_bcb)| {
            BcbMapping {
                kind: BcbMappingKind::MCDCBranch {
                    true_bcb,
                    false_bcb,
                    condition_info,
                    decision_depth,
                },
                span,
            }
        })
    };

    let mut next_bitmap_idx = 0;

    let decision_filter_map = |decision: &MCDCDecisionSpan| {
        let (span, _) = unexpand_into_body_span_with_visible_macro(decision.span, body_span)?;

        let end_bcbs = decision
            .end_markers
            .iter()
            .map(|&marker| bcb_from_marker(marker))
            .collect::<Option<_>>()?;

        let bitmap_idx = next_bitmap_idx;
        next_bitmap_idx += (1_u32 << decision.conditions_num).div_ceil(8);

        Some(BcbMapping {
            kind: BcbMappingKind::MCDCDecision {
                end_bcbs,
                bitmap_idx,
                conditions_num: decision.conditions_num as u16,
                decision_depth: decision.decision_depth,
            },
            span,
        })
    };

    std::iter::empty()
        .chain(branch_info.mcdc_branch_spans.iter().filter_map(mcdc_branch_filter_map))
        .chain(branch_info.mcdc_decision_spans.iter().filter_map(decision_filter_map))
        .collect::<Vec<_>>()
}
