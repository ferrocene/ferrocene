use crate::errors::{CheckNameUnknownTool, RequestedLevel, UnsupportedGroup};
use crate::lints::{
    DeprecatedLintNameFromCommandLine, RemovedLintFromCommandLine, RenamedLintFromCommandLine,
    UnknownLintFromCommandLine,
};
use crate::{
    builtin::MISSING_DOCS,
    context::{CheckLintNameResult, LintStore},
    fluent_generated as fluent,
    late::unerased_lint_store,
    lints::{
        DeprecatedLintName, IgnoredUnlessCrateSpecified, OverruledAttributeLint, RemovedLint,
        RenamedLint, RenamedLintSuggestion, UnknownLint, UnknownLintSuggestion,
    },
};
use rustc_ast as ast;
use rustc_ast_pretty::pprust;
use rustc_data_structures::fx::FxIndexMap;
use rustc_errors::{DecorateLint, Diag, DiagMessage, MultiSpan};
use rustc_feature::{Features, GateIssue};
use rustc_hir as hir;
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::HirId;
use rustc_index::IndexVec;
use rustc_middle::hir::nested_filter;
use rustc_middle::lint::{
    lint_level, reveal_actual_level, LevelAndSource, LintExpectation, LintLevelSource,
    ShallowLintLevelMap,
};
use rustc_middle::query::Providers;
use rustc_middle::ty::{RegisteredTools, TyCtxt};
use rustc_session::lint::{
    builtin::{
        self, FORBIDDEN_LINT_GROUPS, RENAMED_AND_REMOVED_LINTS, SINGLE_USE_LIFETIMES,
        UNFULFILLED_LINT_EXPECTATIONS, UNKNOWN_LINTS, UNUSED_ATTRIBUTES,
    },
    Level, Lint, LintExpectationId, LintId,
};
use rustc_session::parse::feature_err;
use rustc_session::Session;
use rustc_span::symbol::{sym, Symbol};
use rustc_span::{Span, DUMMY_SP};

use crate::errors::{
    MalformedAttribute, MalformedAttributeSub, OverruledAttribute, OverruledAttributeSub,
    UnknownToolInScopedLint,
};

/// Collection of lint levels for the whole crate.
/// This is used by AST-based lints, which do not
/// wait until we have built HIR to be emitted.
#[derive(Debug)]
struct LintLevelSets {
    /// Linked list of specifications.
    list: IndexVec<LintStackIndex, LintSet>,
}

rustc_index::newtype_index! {
    struct LintStackIndex {
        const COMMAND_LINE = 0;
    }
}

/// Specifications found at this position in the stack. This map only represents the lints
/// found for one set of attributes (like `shallow_lint_levels_on` does).
///
/// We store the level specifications as a linked list.
/// Each `LintSet` represents a set of attributes on the same AST node.
/// The `parent` forms a linked list that matches the AST tree.
/// This way, walking the linked list is equivalent to walking the AST bottom-up
/// to find the specifications for a given lint.
#[derive(Debug)]
struct LintSet {
    // -A,-W,-D flags, a `Symbol` for the flag itself and `Level` for which
    // flag.
    specs: FxIndexMap<LintId, LevelAndSource>,
    parent: LintStackIndex,
}

impl LintLevelSets {
    fn new() -> Self {
        LintLevelSets { list: IndexVec::new() }
    }

    fn get_lint_level(
        &self,
        lint: &'static Lint,
        idx: LintStackIndex,
        aux: Option<&FxIndexMap<LintId, LevelAndSource>>,
        sess: &Session,
    ) -> LevelAndSource {
        let lint = LintId::of(lint);
        let (level, mut src) = self.raw_lint_id_level(lint, idx, aux);
        let level = reveal_actual_level(level, &mut src, sess, lint, |id| {
            self.raw_lint_id_level(id, idx, aux)
        });
        (level, src)
    }

    fn raw_lint_id_level(
        &self,
        id: LintId,
        mut idx: LintStackIndex,
        aux: Option<&FxIndexMap<LintId, LevelAndSource>>,
    ) -> (Option<Level>, LintLevelSource) {
        if let Some(specs) = aux {
            if let Some(&(level, src)) = specs.get(&id) {
                return (Some(level), src);
            }
        }
        loop {
            let LintSet { ref specs, parent } = self.list[idx];
            if let Some(&(level, src)) = specs.get(&id) {
                return (Some(level), src);
            }
            if idx == COMMAND_LINE {
                return (None, LintLevelSource::Default);
            }
            idx = parent;
        }
    }
}

fn lint_expectations(tcx: TyCtxt<'_>, (): ()) -> Vec<(LintExpectationId, LintExpectation)> {
    let store = unerased_lint_store(tcx.sess);

    let mut builder = LintLevelsBuilder {
        sess: tcx.sess,
        features: tcx.features(),
        provider: QueryMapExpectationsWrapper {
            tcx,
            cur: hir::CRATE_HIR_ID,
            specs: ShallowLintLevelMap::default(),
            expectations: Vec::new(),
            unstable_to_stable_ids: FxIndexMap::default(),
            empty: FxIndexMap::default(),
        },
        lint_added_lints: false,
        store,
        registered_tools: tcx.registered_tools(()),
    };

    builder.add_command_line();
    builder.add_id(hir::CRATE_HIR_ID);
    tcx.hir().walk_toplevel_module(&mut builder);

    tcx.dcx().update_unstable_expectation_id(&builder.provider.unstable_to_stable_ids);

    builder.provider.expectations
}

#[instrument(level = "trace", skip(tcx), ret)]
fn shallow_lint_levels_on(tcx: TyCtxt<'_>, owner: hir::OwnerId) -> ShallowLintLevelMap {
    let store = unerased_lint_store(tcx.sess);
    let attrs = tcx.hir_attrs(owner);

    let mut levels = LintLevelsBuilder {
        sess: tcx.sess,
        features: tcx.features(),
        provider: LintLevelQueryMap {
            tcx,
            cur: owner.into(),
            specs: ShallowLintLevelMap::default(),
            empty: FxIndexMap::default(),
            attrs,
        },
        lint_added_lints: false,
        store,
        registered_tools: tcx.registered_tools(()),
    };

    if owner == hir::CRATE_OWNER_ID {
        levels.add_command_line();
    }

    match attrs.map.range(..) {
        // There is only something to do if there are attributes at all.
        [] => {}
        // Most of the time, there is only one attribute. Avoid fetching HIR in that case.
        [(local_id, _)] => levels.add_id(HirId { owner, local_id: *local_id }),
        // Otherwise, we need to visit the attributes in source code order, so we fetch HIR and do
        // a standard visit.
        // FIXME(#102522) Just iterate on attrs once that iteration order matches HIR's.
        _ => match tcx.hir_owner_node(owner) {
            hir::OwnerNode::Item(item) => levels.visit_item(item),
            hir::OwnerNode::ForeignItem(item) => levels.visit_foreign_item(item),
            hir::OwnerNode::TraitItem(item) => levels.visit_trait_item(item),
            hir::OwnerNode::ImplItem(item) => levels.visit_impl_item(item),
            hir::OwnerNode::Crate(mod_) => {
                levels.add_id(hir::CRATE_HIR_ID);
                levels.visit_mod(mod_, mod_.spans.inner_span, hir::CRATE_HIR_ID)
            }
        },
    }

    let specs = levels.provider.specs;

    #[cfg(debug_assertions)]
    for (_, v) in specs.specs.iter() {
        debug_assert!(!v.is_empty());
    }

    specs
}

pub struct TopDown {
    sets: LintLevelSets,
    cur: LintStackIndex,
}

pub trait LintLevelsProvider {
    fn current_specs(&self) -> &FxIndexMap<LintId, LevelAndSource>;
    fn insert(&mut self, id: LintId, lvl: LevelAndSource);
    fn get_lint_level(&self, lint: &'static Lint, sess: &Session) -> LevelAndSource;
    fn push_expectation(&mut self, _id: LintExpectationId, _expectation: LintExpectation) {}
}

impl LintLevelsProvider for TopDown {
    fn current_specs(&self) -> &FxIndexMap<LintId, LevelAndSource> {
        &self.sets.list[self.cur].specs
    }

    fn insert(&mut self, id: LintId, lvl: LevelAndSource) {
        self.sets.list[self.cur].specs.insert(id, lvl);
    }

    fn get_lint_level(&self, lint: &'static Lint, sess: &Session) -> LevelAndSource {
        self.sets.get_lint_level(lint, self.cur, Some(self.current_specs()), sess)
    }
}

struct LintLevelQueryMap<'tcx> {
    tcx: TyCtxt<'tcx>,
    cur: HirId,
    specs: ShallowLintLevelMap,
    /// Empty hash map to simplify code.
    empty: FxIndexMap<LintId, LevelAndSource>,
    attrs: &'tcx hir::AttributeMap<'tcx>,
}

impl LintLevelsProvider for LintLevelQueryMap<'_> {
    fn current_specs(&self) -> &FxIndexMap<LintId, LevelAndSource> {
        self.specs.specs.get(&self.cur.local_id).unwrap_or(&self.empty)
    }
    fn insert(&mut self, id: LintId, lvl: LevelAndSource) {
        self.specs.specs.get_mut_or_insert_default(self.cur.local_id).insert(id, lvl);
    }
    fn get_lint_level(&self, lint: &'static Lint, _: &Session) -> LevelAndSource {
        self.specs.lint_level_id_at_node(self.tcx, LintId::of(lint), self.cur)
    }
}

struct QueryMapExpectationsWrapper<'tcx> {
    tcx: TyCtxt<'tcx>,
    /// HirId of the currently investigated element.
    cur: HirId,
    /// Level map for `cur`.
    specs: ShallowLintLevelMap,
    expectations: Vec<(LintExpectationId, LintExpectation)>,
    unstable_to_stable_ids: FxIndexMap<LintExpectationId, LintExpectationId>,
    /// Empty hash map to simplify code.
    empty: FxIndexMap<LintId, LevelAndSource>,
}

impl LintLevelsProvider for QueryMapExpectationsWrapper<'_> {
    fn current_specs(&self) -> &FxIndexMap<LintId, LevelAndSource> {
        self.specs.specs.get(&self.cur.local_id).unwrap_or(&self.empty)
    }
    fn insert(&mut self, id: LintId, lvl: LevelAndSource) {
        self.specs.specs.get_mut_or_insert_default(self.cur.local_id).insert(id, lvl);
    }
    fn get_lint_level(&self, lint: &'static Lint, _: &Session) -> LevelAndSource {
        // We cannot use `tcx.lint_level_at_node` because we want to know in which order the
        // attributes have been inserted, in particular whether an `expect` follows a `forbid`.
        self.specs.lint_level_id_at_node(self.tcx, LintId::of(lint), self.cur)
    }
    fn push_expectation(&mut self, id: LintExpectationId, expectation: LintExpectation) {
        let LintExpectationId::Stable { attr_id: Some(attr_id), hir_id, attr_index, .. } = id
        else {
            bug!("unstable expectation id should already be mapped")
        };
        let key = LintExpectationId::Unstable { attr_id, lint_index: None };

        self.unstable_to_stable_ids.entry(key).or_insert(LintExpectationId::Stable {
            hir_id,
            attr_index,
            lint_index: None,
            attr_id: None,
        });

        self.expectations.push((id.normalize(), expectation));
    }
}

impl<'tcx> LintLevelsBuilder<'_, LintLevelQueryMap<'tcx>> {
    fn add_id(&mut self, hir_id: HirId) {
        self.provider.cur = hir_id;
        self.add(
            self.provider.attrs.get(hir_id.local_id),
            hir_id == hir::CRATE_HIR_ID,
            Some(hir_id),
        );
    }
}

impl<'tcx> Visitor<'tcx> for LintLevelsBuilder<'_, LintLevelQueryMap<'tcx>> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.provider.tcx.hir()
    }

    fn visit_param(&mut self, param: &'tcx hir::Param<'tcx>) {
        self.add_id(param.hir_id);
        intravisit::walk_param(self, param);
    }

    fn visit_item(&mut self, it: &'tcx hir::Item<'tcx>) {
        self.add_id(it.hir_id());
        intravisit::walk_item(self, it);
    }

    fn visit_foreign_item(&mut self, it: &'tcx hir::ForeignItem<'tcx>) {
        self.add_id(it.hir_id());
        intravisit::walk_foreign_item(self, it);
    }

    fn visit_stmt(&mut self, e: &'tcx hir::Stmt<'tcx>) {
        // We will call `add_id` when we walk
        // the `StmtKind`. The outer statement itself doesn't
        // define the lint levels.
        intravisit::walk_stmt(self, e);
    }

    fn visit_expr(&mut self, e: &'tcx hir::Expr<'tcx>) {
        self.add_id(e.hir_id);
        intravisit::walk_expr(self, e);
    }

    fn visit_expr_field(&mut self, f: &'tcx hir::ExprField<'tcx>) {
        self.add_id(f.hir_id);
        intravisit::walk_expr_field(self, f);
    }

    fn visit_field_def(&mut self, s: &'tcx hir::FieldDef<'tcx>) {
        self.add_id(s.hir_id);
        intravisit::walk_field_def(self, s);
    }

    fn visit_variant(&mut self, v: &'tcx hir::Variant<'tcx>) {
        self.add_id(v.hir_id);
        intravisit::walk_variant(self, v);
    }

    fn visit_local(&mut self, l: &'tcx hir::Local<'tcx>) {
        self.add_id(l.hir_id);
        intravisit::walk_local(self, l);
    }

    fn visit_arm(&mut self, a: &'tcx hir::Arm<'tcx>) {
        self.add_id(a.hir_id);
        intravisit::walk_arm(self, a);
    }

    fn visit_trait_item(&mut self, trait_item: &'tcx hir::TraitItem<'tcx>) {
        self.add_id(trait_item.hir_id());
        intravisit::walk_trait_item(self, trait_item);
    }

    fn visit_impl_item(&mut self, impl_item: &'tcx hir::ImplItem<'tcx>) {
        self.add_id(impl_item.hir_id());
        intravisit::walk_impl_item(self, impl_item);
    }
}

impl<'tcx> LintLevelsBuilder<'_, QueryMapExpectationsWrapper<'tcx>> {
    fn add_id(&mut self, hir_id: HirId) {
        // Change both the `HirId` and the associated specs.
        self.provider.cur = hir_id;
        self.provider.specs.specs.clear();
        self.add(self.provider.tcx.hir().attrs(hir_id), hir_id == hir::CRATE_HIR_ID, Some(hir_id));
    }
}

impl<'tcx> Visitor<'tcx> for LintLevelsBuilder<'_, QueryMapExpectationsWrapper<'tcx>> {
    type NestedFilter = nested_filter::All;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.provider.tcx.hir()
    }

    fn visit_param(&mut self, param: &'tcx hir::Param<'tcx>) {
        self.add_id(param.hir_id);
        intravisit::walk_param(self, param);
    }

    fn visit_item(&mut self, it: &'tcx hir::Item<'tcx>) {
        self.add_id(it.hir_id());
        intravisit::walk_item(self, it);
    }

    fn visit_foreign_item(&mut self, it: &'tcx hir::ForeignItem<'tcx>) {
        self.add_id(it.hir_id());
        intravisit::walk_foreign_item(self, it);
    }

    fn visit_stmt(&mut self, e: &'tcx hir::Stmt<'tcx>) {
        // We will call `add_id` when we walk
        // the `StmtKind`. The outer statement itself doesn't
        // define the lint levels.
        intravisit::walk_stmt(self, e);
    }

    fn visit_expr(&mut self, e: &'tcx hir::Expr<'tcx>) {
        self.add_id(e.hir_id);
        intravisit::walk_expr(self, e);
    }

    fn visit_field_def(&mut self, s: &'tcx hir::FieldDef<'tcx>) {
        self.add_id(s.hir_id);
        intravisit::walk_field_def(self, s);
    }

    fn visit_variant(&mut self, v: &'tcx hir::Variant<'tcx>) {
        self.add_id(v.hir_id);
        intravisit::walk_variant(self, v);
    }

    fn visit_local(&mut self, l: &'tcx hir::Local<'tcx>) {
        self.add_id(l.hir_id);
        intravisit::walk_local(self, l);
    }

    fn visit_arm(&mut self, a: &'tcx hir::Arm<'tcx>) {
        self.add_id(a.hir_id);
        intravisit::walk_arm(self, a);
    }

    fn visit_trait_item(&mut self, trait_item: &'tcx hir::TraitItem<'tcx>) {
        self.add_id(trait_item.hir_id());
        intravisit::walk_trait_item(self, trait_item);
    }

    fn visit_impl_item(&mut self, impl_item: &'tcx hir::ImplItem<'tcx>) {
        self.add_id(impl_item.hir_id());
        intravisit::walk_impl_item(self, impl_item);
    }
}

pub struct LintLevelsBuilder<'s, P> {
    sess: &'s Session,
    features: &'s Features,
    provider: P,
    lint_added_lints: bool,
    store: &'s LintStore,
    registered_tools: &'s RegisteredTools,
}

pub(crate) struct BuilderPush {
    prev: LintStackIndex,
}

impl<'s> LintLevelsBuilder<'s, TopDown> {
    pub(crate) fn new(
        sess: &'s Session,
        features: &'s Features,
        lint_added_lints: bool,
        store: &'s LintStore,
        registered_tools: &'s RegisteredTools,
    ) -> Self {
        let mut builder = LintLevelsBuilder {
            sess,
            features,
            provider: TopDown { sets: LintLevelSets::new(), cur: COMMAND_LINE },
            lint_added_lints,
            store,
            registered_tools,
        };
        builder.process_command_line();
        assert_eq!(builder.provider.sets.list.len(), 1);
        builder
    }

    fn process_command_line(&mut self) {
        self.provider.cur = self
            .provider
            .sets
            .list
            .push(LintSet { specs: FxIndexMap::default(), parent: COMMAND_LINE });
        self.add_command_line();
    }

    /// Pushes a list of AST lint attributes onto this context.
    ///
    /// This function will return a `BuilderPush` object which should be passed
    /// to `pop` when this scope for the attributes provided is exited.
    ///
    /// This function will perform a number of tasks:
    ///
    /// * It'll validate all lint-related attributes in `attrs`
    /// * It'll mark all lint-related attributes as used
    /// * Lint levels will be updated based on the attributes provided
    /// * Lint attributes are validated, e.g., a `#[forbid]` can't be switched to
    ///   `#[allow]`
    ///
    /// Don't forget to call `pop`!
    pub(crate) fn push(
        &mut self,
        attrs: &[ast::Attribute],
        is_crate_node: bool,
        source_hir_id: Option<HirId>,
    ) -> BuilderPush {
        let prev = self.provider.cur;
        self.provider.cur =
            self.provider.sets.list.push(LintSet { specs: FxIndexMap::default(), parent: prev });

        self.add(attrs, is_crate_node, source_hir_id);

        if self.provider.current_specs().is_empty() {
            self.provider.sets.list.pop();
            self.provider.cur = prev;
        }

        BuilderPush { prev }
    }

    /// Called after `push` when the scope of a set of attributes are exited.
    pub(crate) fn pop(&mut self, push: BuilderPush) {
        self.provider.cur = push.prev;
        std::mem::forget(push);
    }
}

#[cfg(debug_assertions)]
impl Drop for BuilderPush {
    fn drop(&mut self) {
        panic!("Found a `push` without a `pop`.");
    }
}

impl<'s, P: LintLevelsProvider> LintLevelsBuilder<'s, P> {
    pub(crate) fn sess(&self) -> &Session {
        self.sess
    }

    pub(crate) fn features(&self) -> &Features {
        self.features
    }

    fn current_specs(&self) -> &FxIndexMap<LintId, LevelAndSource> {
        self.provider.current_specs()
    }

    fn insert(&mut self, id: LintId, lvl: LevelAndSource) {
        self.provider.insert(id, lvl)
    }

    fn add_command_line(&mut self) {
        for &(ref lint_name, level) in &self.sess.opts.lint_opts {
            // Checks the validity of lint names derived from the command line.
            let (tool_name, lint_name_only) = parse_lint_and_tool_name(lint_name);
            if lint_name_only == crate::WARNINGS.name_lower()
                && matches!(level, Level::ForceWarn(_))
            {
                self.sess
                    .dcx()
                    .emit_err(UnsupportedGroup { lint_group: crate::WARNINGS.name_lower() });
            }
            match self.store.check_lint_name(lint_name_only, tool_name, self.registered_tools) {
                CheckLintNameResult::Renamed(ref replace) => {
                    let name = lint_name.as_str();
                    let suggestion = RenamedLintSuggestion::WithoutSpan { replace };
                    let requested_level = RequestedLevel { level, lint_name };
                    let lint = RenamedLintFromCommandLine { name, suggestion, requested_level };
                    self.emit_lint(RENAMED_AND_REMOVED_LINTS, lint);
                }
                CheckLintNameResult::Removed(ref reason) => {
                    let name = lint_name.as_str();
                    let requested_level = RequestedLevel { level, lint_name };
                    let lint = RemovedLintFromCommandLine { name, reason, requested_level };
                    self.emit_lint(RENAMED_AND_REMOVED_LINTS, lint);
                }
                CheckLintNameResult::NoLint(suggestion) => {
                    let name = lint_name.clone();
                    let suggestion = suggestion.map(|(replace, from_rustc)| {
                        UnknownLintSuggestion::WithoutSpan { replace, from_rustc }
                    });
                    let requested_level = RequestedLevel { level, lint_name };
                    let lint = UnknownLintFromCommandLine { name, suggestion, requested_level };
                    self.emit_lint(UNKNOWN_LINTS, lint);
                }
                CheckLintNameResult::Tool(Err((Some(_), ref replace))) => {
                    let name = lint_name.clone();
                    let requested_level = RequestedLevel { level, lint_name };
                    let lint = DeprecatedLintNameFromCommandLine { name, replace, requested_level };
                    self.emit_lint(RENAMED_AND_REMOVED_LINTS, lint);
                }
                CheckLintNameResult::NoTool => {
                    self.sess.dcx().emit_err(CheckNameUnknownTool {
                        tool_name: tool_name.unwrap(),
                        sub: RequestedLevel { level, lint_name },
                    });
                }
                _ => {}
            };

            let orig_level = level;
            let lint_flag_val = Symbol::intern(lint_name);

            let Ok(ids) = self.store.find_lints(lint_name) else {
                // errors already handled above
                continue;
            };
            for id in ids {
                // ForceWarn and Forbid cannot be overridden
                if let Some((Level::ForceWarn(_) | Level::Forbid, _)) =
                    self.current_specs().get(&id)
                {
                    continue;
                }

                if self.check_gated_lint(id, DUMMY_SP, true) {
                    let src = LintLevelSource::CommandLine(lint_flag_val, orig_level);
                    self.insert(id, (level, src));
                }
            }
        }
    }

    /// Attempts to insert the `id` to `level_src` map entry. If unsuccessful
    /// (e.g. if a forbid was already inserted on the same scope), then emits a
    /// diagnostic with no change to `specs`.
    fn insert_spec(&mut self, id: LintId, (mut level, src): LevelAndSource) {
        let (old_level, old_src) = self.provider.get_lint_level(id.lint, self.sess);
        if let Level::Expect(id) = &mut level
            && let LintExpectationId::Stable { .. } = id
        {
            *id = id.normalize();
        }
        // Setting to a non-forbid level is an error if the lint previously had
        // a forbid level. Note that this is not necessarily true even with a
        // `#[forbid(..)]` attribute present, as that is overridden by `--cap-lints`.
        //
        // This means that this only errors if we're truly lowering the lint
        // level from forbid.
        if self.lint_added_lints && level != Level::Forbid {
            if let Level::Forbid = old_level {
                // Backwards compatibility check:
                //
                // We used to not consider `forbid(lint_group)`
                // as preventing `allow(lint)` for some lint `lint` in
                // `lint_group`. For now, issue a future-compatibility
                // warning for this case.
                let id_name = id.lint.name_lower();
                let fcw_warning = match old_src {
                    LintLevelSource::Default => false,
                    LintLevelSource::Node { name, .. } => self.store.is_lint_group(name),
                    LintLevelSource::CommandLine(symbol, _) => self.store.is_lint_group(symbol),
                };
                debug!(
                    "fcw_warning={:?}, specs.get(&id) = {:?}, old_src={:?}, id_name={:?}",
                    fcw_warning,
                    self.current_specs(),
                    old_src,
                    id_name
                );
                let sub = match old_src {
                    LintLevelSource::Default => {
                        OverruledAttributeSub::DefaultSource { id: id.to_string() }
                    }
                    LintLevelSource::Node { span, reason, .. } => {
                        OverruledAttributeSub::NodeSource { span, reason }
                    }
                    LintLevelSource::CommandLine(_, _) => OverruledAttributeSub::CommandLineSource,
                };
                if !fcw_warning {
                    self.sess.dcx().emit_err(OverruledAttribute {
                        span: src.span(),
                        overruled: src.span(),
                        lint_level: level.as_str(),
                        lint_source: src.name(),
                        sub,
                    });
                } else {
                    self.emit_span_lint(
                        FORBIDDEN_LINT_GROUPS,
                        src.span().into(),
                        OverruledAttributeLint {
                            overruled: src.span(),
                            lint_level: level.as_str(),
                            lint_source: src.name(),
                            sub,
                        },
                    );
                }

                // Retain the forbid lint level, unless we are
                // issuing a FCW. In the FCW case, we want to
                // respect the new setting.
                if !fcw_warning {
                    return;
                }
            }
        }

        // The lint `unfulfilled_lint_expectations` can't be expected, as it would suppress itself.
        // Handling expectations of this lint would add additional complexity with little to no
        // benefit. The expect level for this lint will therefore be ignored.
        if let Level::Expect(_) = level
            && id == LintId::of(UNFULFILLED_LINT_EXPECTATIONS)
        {
            return;
        }

        match (old_level, level) {
            // If the new level is an expectation store it in `ForceWarn`
            (Level::ForceWarn(_), Level::Expect(expectation_id)) => {
                self.insert(id, (Level::ForceWarn(Some(expectation_id)), old_src))
            }
            // Keep `ForceWarn` level but drop the expectation
            (Level::ForceWarn(_), _) => self.insert(id, (Level::ForceWarn(None), old_src)),
            // Set the lint level as normal
            _ => self.insert(id, (level, src)),
        };
    }

    #[allow(rustc::untranslatable_diagnostic)] // FIXME: make this translatable
    fn add(&mut self, attrs: &[ast::Attribute], is_crate_node: bool, source_hir_id: Option<HirId>) {
        let sess = self.sess;
        for (attr_index, attr) in attrs.iter().enumerate() {
            if attr.has_name(sym::automatically_derived) {
                self.insert(
                    LintId::of(SINGLE_USE_LIFETIMES),
                    (Level::Allow, LintLevelSource::Default),
                );
                continue;
            }

            // `#[doc(hidden)]` disables missing_docs check.
            if attr.has_name(sym::doc)
                && attr
                    .meta_item_list()
                    .is_some_and(|l| ast::attr::list_contains_name(&l, sym::hidden))
            {
                self.insert(LintId::of(MISSING_DOCS), (Level::Allow, LintLevelSource::Default));
                continue;
            }

            let level = match Level::from_attr(attr) {
                None => continue,
                // This is the only lint level with a `LintExpectationId` that can be created from an attribute
                Some(Level::Expect(unstable_id)) if let Some(hir_id) = source_hir_id => {
                    let LintExpectationId::Unstable { attr_id, lint_index } = unstable_id else {
                        bug!("stable id Level::from_attr")
                    };

                    let stable_id = LintExpectationId::Stable {
                        hir_id,
                        attr_index: attr_index.try_into().unwrap(),
                        lint_index,
                        // we pass the previous unstable attr_id such that we can trace the ast id when building a map
                        // to go from unstable to stable id.
                        attr_id: Some(attr_id),
                    };

                    Level::Expect(stable_id)
                }
                Some(lvl) => lvl,
            };

            let Some(mut metas) = attr.meta_item_list() else { continue };

            if metas.is_empty() {
                // This emits the unused_attributes lint for `#[level()]`
                continue;
            }

            // Before processing the lint names, look for a reason (RFC 2383)
            // at the end.
            let mut reason = None;
            let tail_li = &metas[metas.len() - 1];
            if let Some(item) = tail_li.meta_item() {
                match item.kind {
                    ast::MetaItemKind::Word => {} // actual lint names handled later
                    ast::MetaItemKind::NameValue(ref name_value) => {
                        if item.path == sym::reason {
                            if let ast::LitKind::Str(rationale, _) = name_value.kind {
                                if !self.features.lint_reasons {
                                    feature_err(
                                        &self.sess,
                                        sym::lint_reasons,
                                        item.span,
                                        "lint reasons are experimental",
                                    )
                                    .emit();
                                }
                                reason = Some(rationale);
                            } else {
                                sess.dcx().emit_err(MalformedAttribute {
                                    span: name_value.span,
                                    sub: MalformedAttributeSub::ReasonMustBeStringLiteral(
                                        name_value.span,
                                    ),
                                });
                            }
                            // found reason, reslice meta list to exclude it
                            metas.pop().unwrap();
                        } else {
                            sess.dcx().emit_err(MalformedAttribute {
                                span: item.span,
                                sub: MalformedAttributeSub::BadAttributeArgument(item.span),
                            });
                        }
                    }
                    ast::MetaItemKind::List(_) => {
                        sess.dcx().emit_err(MalformedAttribute {
                            span: item.span,
                            sub: MalformedAttributeSub::BadAttributeArgument(item.span),
                        });
                    }
                }
            }

            for (lint_index, li) in metas.iter_mut().enumerate() {
                let level = match level {
                    Level::Expect(mut id) => {
                        id.set_lint_index(Some(lint_index as u16));
                        Level::Expect(id)
                    }
                    level => level,
                };

                let sp = li.span();
                let meta_item = match li {
                    ast::NestedMetaItem::MetaItem(meta_item) if meta_item.is_word() => meta_item,
                    _ => {
                        if let Some(item) = li.meta_item() {
                            if let ast::MetaItemKind::NameValue(_) = item.kind {
                                if item.path == sym::reason {
                                    sess.dcx().emit_err(MalformedAttribute {
                                        span: sp,
                                        sub: MalformedAttributeSub::ReasonMustComeLast(sp),
                                    });
                                    continue;
                                }
                            }
                        }
                        sess.dcx().emit_err(MalformedAttribute {
                            span: sp,
                            sub: MalformedAttributeSub::BadAttributeArgument(sp),
                        });
                        continue;
                    }
                };
                let tool_ident = if meta_item.path.segments.len() > 1 {
                    Some(meta_item.path.segments.remove(0).ident)
                } else {
                    None
                };
                let tool_name = tool_ident.map(|ident| ident.name);
                let name = pprust::path_to_string(&meta_item.path);
                let lint_result =
                    self.store.check_lint_name(&name, tool_name, self.registered_tools);
                match &lint_result {
                    CheckLintNameResult::Ok(ids) => {
                        // This checks for instances where the user writes `#[expect(unfulfilled_lint_expectations)]`
                        // in that case we want to avoid overriding the lint level but instead add an expectation that
                        // can't be fulfilled. The lint message will include an explanation, that the
                        // `unfulfilled_lint_expectations` lint can't be expected.
                        if let Level::Expect(expect_id) = level {
                            // The `unfulfilled_lint_expectations` lint is not part of any lint groups. Therefore. we
                            // only need to check the slice if it contains a single lint.
                            let is_unfulfilled_lint_expectations = match ids {
                                [lint] => *lint == LintId::of(UNFULFILLED_LINT_EXPECTATIONS),
                                _ => false,
                            };
                            self.provider.push_expectation(
                                expect_id,
                                LintExpectation::new(
                                    reason,
                                    sp,
                                    is_unfulfilled_lint_expectations,
                                    tool_name,
                                ),
                            );
                        }
                        let src = LintLevelSource::Node {
                            name: meta_item
                                .path
                                .segments
                                .last()
                                .expect("empty lint name")
                                .ident
                                .name,
                            span: sp,
                            reason,
                        };
                        for &id in *ids {
                            if self.check_gated_lint(id, attr.span, false) {
                                self.insert_spec(id, (level, src));
                            }
                        }
                    }

                    CheckLintNameResult::Tool(result) => {
                        match *result {
                            Ok(ids) => {
                                let complete_name =
                                    &format!("{}::{}", tool_ident.unwrap().name, name);
                                let src = LintLevelSource::Node {
                                    name: Symbol::intern(complete_name),
                                    span: sp,
                                    reason,
                                };
                                for &id in ids {
                                    if self.check_gated_lint(id, attr.span, false) {
                                        self.insert_spec(id, (level, src));
                                    }
                                }
                                if let Level::Expect(expect_id) = level {
                                    self.provider.push_expectation(
                                        expect_id,
                                        LintExpectation::new(reason, sp, false, tool_name),
                                    );
                                }
                            }
                            Err((Some(ids), ref new_lint_name)) => {
                                let lint = builtin::RENAMED_AND_REMOVED_LINTS;
                                self.emit_span_lint(
                                    lint,
                                    sp.into(),
                                    DeprecatedLintName {
                                        name,
                                        suggestion: sp,
                                        replace: new_lint_name,
                                    },
                                );

                                let src = LintLevelSource::Node {
                                    name: Symbol::intern(new_lint_name),
                                    span: sp,
                                    reason,
                                };
                                for id in ids {
                                    self.insert_spec(*id, (level, src));
                                }
                                if let Level::Expect(expect_id) = level {
                                    self.provider.push_expectation(
                                        expect_id,
                                        LintExpectation::new(reason, sp, false, tool_name),
                                    );
                                }
                            }
                            Err((None, _)) => {
                                // If Tool(Err(None, _)) is returned, then either the lint does not
                                // exist in the tool or the code was not compiled with the tool and
                                // therefore the lint was never added to the `LintStore`. To detect
                                // this is the responsibility of the lint tool.
                            }
                        }
                    }

                    &CheckLintNameResult::NoTool => {
                        sess.dcx().emit_err(UnknownToolInScopedLint {
                            span: tool_ident.map(|ident| ident.span),
                            tool_name: tool_name.unwrap(),
                            lint_name: pprust::path_to_string(&meta_item.path),
                            is_nightly_build: sess.is_nightly_build().then_some(()),
                        });
                        continue;
                    }

                    _ if !self.lint_added_lints => {}

                    CheckLintNameResult::Renamed(ref replace) => {
                        let suggestion =
                            RenamedLintSuggestion::WithSpan { suggestion: sp, replace };
                        let name = tool_ident.map(|tool| format!("{tool}::{name}")).unwrap_or(name);
                        let lint = RenamedLint { name: name.as_str(), suggestion };
                        self.emit_span_lint(RENAMED_AND_REMOVED_LINTS, sp.into(), lint);
                    }

                    CheckLintNameResult::Removed(ref reason) => {
                        let name = tool_ident.map(|tool| format!("{tool}::{name}")).unwrap_or(name);
                        let lint = RemovedLint { name: name.as_str(), reason };
                        self.emit_span_lint(RENAMED_AND_REMOVED_LINTS, sp.into(), lint);
                    }

                    CheckLintNameResult::NoLint(suggestion) => {
                        let name = if let Some(tool_ident) = tool_ident {
                            format!("{}::{}", tool_ident.name, name)
                        } else {
                            name.to_string()
                        };
                        let suggestion = suggestion.map(|(replace, from_rustc)| {
                            UnknownLintSuggestion::WithSpan { suggestion: sp, replace, from_rustc }
                        });
                        let lint = UnknownLint { name, suggestion };
                        self.emit_span_lint(UNKNOWN_LINTS, sp.into(), lint);
                    }
                }
                // If this lint was renamed, apply the new lint instead of ignoring the attribute.
                // This happens outside of the match because the new lint should be applied even if
                // we don't warn about the name change.
                if let CheckLintNameResult::Renamed(new_name) = lint_result {
                    // Ignore any errors or warnings that happen because the new name is inaccurate
                    // NOTE: `new_name` already includes the tool name, so we don't have to add it again.
                    if let CheckLintNameResult::Ok(ids) =
                        self.store.check_lint_name(&new_name, None, self.registered_tools)
                    {
                        let src = LintLevelSource::Node {
                            name: Symbol::intern(&new_name),
                            span: sp,
                            reason,
                        };
                        for &id in ids {
                            if self.check_gated_lint(id, attr.span, false) {
                                self.insert_spec(id, (level, src));
                            }
                        }
                        if let Level::Expect(expect_id) = level {
                            self.provider.push_expectation(
                                expect_id,
                                LintExpectation::new(reason, sp, false, tool_name),
                            );
                        }
                    } else {
                        panic!("renamed lint does not exist: {new_name}");
                    }
                }
            }
        }

        if self.lint_added_lints && !is_crate_node {
            for (id, &(level, ref src)) in self.current_specs().iter() {
                if !id.lint.crate_level_only {
                    continue;
                }

                let LintLevelSource::Node { name: lint_attr_name, span: lint_attr_span, .. } = *src
                else {
                    continue;
                };

                self.emit_span_lint(
                    UNUSED_ATTRIBUTES,
                    lint_attr_span.into(),
                    IgnoredUnlessCrateSpecified { level: level.as_str(), name: lint_attr_name },
                );
                // don't set a separate error for every lint in the group
                break;
            }
        }
    }

    /// Checks if the lint is gated on a feature that is not enabled.
    ///
    /// Returns `true` if the lint's feature is enabled.
    #[track_caller]
    fn check_gated_lint(&self, lint_id: LintId, span: Span, lint_from_cli: bool) -> bool {
        if let Some(feature) = lint_id.lint.feature_gate {
            if !self.features.active(feature) {
                if self.lint_added_lints {
                    let lint = builtin::UNKNOWN_LINTS;
                    let (level, src) = self.lint_level(builtin::UNKNOWN_LINTS);
                    // FIXME: make this translatable
                    #[allow(rustc::diagnostic_outside_of_impl)]
                    #[allow(rustc::untranslatable_diagnostic)]
                    lint_level(
                        self.sess,
                        lint,
                        level,
                        src,
                        Some(span.into()),
                        fluent::lint_unknown_gated_lint,
                        |lint| {
                            lint.arg("name", lint_id.lint.name_lower());
                            lint.note(fluent::lint_note);
                            rustc_session::parse::add_feature_diagnostics_for_issue(
                                lint,
                                &self.sess,
                                feature,
                                GateIssue::Language,
                                lint_from_cli,
                            );
                        },
                    );
                }
                return false;
            }
        }
        true
    }

    /// Find the lint level for a lint.
    pub fn lint_level(&self, lint: &'static Lint) -> LevelAndSource {
        self.provider.get_lint_level(lint, self.sess)
    }

    /// Used to emit a lint-related diagnostic based on the current state of
    /// this lint context.
    ///
    /// [`lint_level`]: rustc_middle::lint::lint_level#decorate-signature
    #[rustc_lint_diagnostics]
    #[track_caller]
    pub(crate) fn opt_span_lint(
        &self,
        lint: &'static Lint,
        span: Option<MultiSpan>,
        msg: impl Into<DiagMessage>,
        decorate: impl for<'a, 'b> FnOnce(&'b mut Diag<'a, ()>),
    ) {
        let (level, src) = self.lint_level(lint);
        lint_level(self.sess, lint, level, src, span, msg, decorate)
    }

    #[track_caller]
    pub fn emit_span_lint(
        &self,
        lint: &'static Lint,
        span: MultiSpan,
        decorate: impl for<'a> DecorateLint<'a, ()>,
    ) {
        let (level, src) = self.lint_level(lint);
        lint_level(self.sess, lint, level, src, Some(span), decorate.msg(), |lint| {
            decorate.decorate_lint(lint);
        });
    }

    #[track_caller]
    pub fn emit_lint(&self, lint: &'static Lint, decorate: impl for<'a> DecorateLint<'a, ()>) {
        let (level, src) = self.lint_level(lint);
        lint_level(self.sess, lint, level, src, None, decorate.msg(), |lint| {
            decorate.decorate_lint(lint);
        });
    }
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { shallow_lint_levels_on, lint_expectations, ..*providers };
}

pub fn parse_lint_and_tool_name(lint_name: &str) -> (Option<Symbol>, &str) {
    match lint_name.split_once("::") {
        Some((tool_name, lint_name)) => {
            let tool_name = Symbol::intern(tool_name);

            (Some(tool_name), lint_name)
        }
        None => (None, lint_name),
    }
}
