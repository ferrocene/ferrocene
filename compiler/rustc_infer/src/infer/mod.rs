pub use at::DefineOpaqueTypes;
pub use freshen::TypeFreshener;
pub use lexical_region_resolve::RegionResolutionError;
pub use relate::combine::CombineFields;
pub use relate::combine::ObligationEmittingRelation;
pub use relate::StructurallyRelateAliases;
pub use rustc_middle::ty::IntVarValue;
pub use BoundRegionConversionTime::*;
pub use RegionVariableOrigin::*;
pub use SubregionOrigin::*;
pub use ValuePairs::*;

use crate::traits::{
    self, ObligationCause, ObligationInspector, PredicateObligations, TraitEngine, TraitEngineExt,
};
use error_reporting::TypeErrCtxt;
use free_regions::RegionRelations;
use lexical_region_resolve::LexicalRegionResolutions;
use opaque_types::OpaqueTypeStorage;
use region_constraints::{GenericKind, VarInfos, VerifyBound};
use region_constraints::{RegionConstraintCollector, RegionConstraintStorage};
use rustc_data_structures::captures::Captures;
use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_data_structures::sync::Lrc;
use rustc_data_structures::undo_log::Rollback;
use rustc_data_structures::unify as ut;
use rustc_errors::{Diag, DiagCtxt, ErrorGuaranteed};
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::infer::canonical::{Canonical, CanonicalVarValues};
use rustc_middle::infer::unify_key::ConstVariableValue;
use rustc_middle::infer::unify_key::EffectVarValue;
use rustc_middle::infer::unify_key::{ConstVariableOrigin, ConstVariableOriginKind, ToType};
use rustc_middle::infer::unify_key::{ConstVidKey, EffectVidKey};
use rustc_middle::mir::interpret::{ErrorHandled, EvalToValTreeResult};
use rustc_middle::mir::ConstraintCategory;
use rustc_middle::traits::{select, DefiningAnchor};
use rustc_middle::ty::error::{ExpectedFound, TypeError};
use rustc_middle::ty::fold::BoundVarReplacerDelegate;
use rustc_middle::ty::fold::{TypeFoldable, TypeFolder, TypeSuperFoldable};
use rustc_middle::ty::relate::RelateResult;
use rustc_middle::ty::visit::TypeVisitableExt;
use rustc_middle::ty::{self, GenericParamDefKind, InferConst, InferTy, Ty, TyCtxt};
use rustc_middle::ty::{ConstVid, EffectVid, FloatVid, IntVid, TyVid};
use rustc_middle::ty::{GenericArg, GenericArgKind, GenericArgs, GenericArgsRef};
use rustc_span::symbol::Symbol;
use rustc_span::Span;
use snapshot::undo_log::InferCtxtUndoLogs;
use std::cell::{Cell, RefCell};
use std::fmt;
use type_variable::{TypeVariableOrigin, TypeVariableOriginKind};

pub mod at;
pub mod canonical;
pub mod error_reporting;
pub mod free_regions;
mod freshen;
mod lexical_region_resolve;
pub mod opaque_types;
pub mod outlives;
mod projection;
pub mod region_constraints;
mod relate;
pub mod resolve;
pub(crate) mod snapshot;
pub mod type_variable;

#[must_use]
#[derive(Debug)]
pub struct InferOk<'tcx, T> {
    pub value: T,
    pub obligations: PredicateObligations<'tcx>,
}
pub type InferResult<'tcx, T> = Result<InferOk<'tcx, T>, TypeError<'tcx>>;

pub type UnitResult<'tcx> = RelateResult<'tcx, ()>; // "unify result"
pub type FixupResult<T> = Result<T, FixupError>; // "fixup result"

pub(crate) type UnificationTable<'a, 'tcx, T> = ut::UnificationTable<
    ut::InPlace<T, &'a mut ut::UnificationStorage<T>, &'a mut InferCtxtUndoLogs<'tcx>>,
>;

/// This type contains all the things within `InferCtxt` that sit within a
/// `RefCell` and are involved with taking/rolling back snapshots. Snapshot
/// operations are hot enough that we want only one call to `borrow_mut` per
/// call to `start_snapshot` and `rollback_to`.
#[derive(Clone)]
pub struct InferCtxtInner<'tcx> {
    undo_log: InferCtxtUndoLogs<'tcx>,

    /// Cache for projections.
    ///
    /// This cache is snapshotted along with the infcx.
    projection_cache: traits::ProjectionCacheStorage<'tcx>,

    /// We instantiate `UnificationTable` with `bounds<Ty>` because the types
    /// that might instantiate a general type variable have an order,
    /// represented by its upper and lower bounds.
    type_variable_storage: type_variable::TypeVariableStorage<'tcx>,

    /// Map from const parameter variable to the kind of const it represents.
    const_unification_storage: ut::UnificationTableStorage<ConstVidKey<'tcx>>,

    /// Map from integral variable to the kind of integer it represents.
    int_unification_storage: ut::UnificationTableStorage<ty::IntVid>,

    /// Map from floating variable to the kind of float it represents.
    float_unification_storage: ut::UnificationTableStorage<ty::FloatVid>,

    /// Map from effect variable to the effect param it represents.
    effect_unification_storage: ut::UnificationTableStorage<EffectVidKey<'tcx>>,

    /// Tracks the set of region variables and the constraints between them.
    ///
    /// This is initially `Some(_)` but when
    /// `resolve_regions_and_report_errors` is invoked, this gets set to `None`
    /// -- further attempts to perform unification, etc., may fail if new
    /// region constraints would've been added.
    region_constraint_storage: Option<RegionConstraintStorage<'tcx>>,

    /// A set of constraints that regionck must validate.
    ///
    /// Each constraint has the form `T:'a`, meaning "some type `T` must
    /// outlive the lifetime 'a". These constraints derive from
    /// instantiated type parameters. So if you had a struct defined
    /// like the following:
    /// ```ignore (illustrative)
    /// struct Foo<T: 'static> { ... }
    /// ```
    /// In some expression `let x = Foo { ... }`, it will
    /// instantiate the type parameter `T` with a fresh type `$0`. At
    /// the same time, it will record a region obligation of
    /// `$0: 'static`. This will get checked later by regionck. (We
    /// can't generally check these things right away because we have
    /// to wait until types are resolved.)
    ///
    /// These are stored in a map keyed to the id of the innermost
    /// enclosing fn body / static initializer expression. This is
    /// because the location where the obligation was incurred can be
    /// relevant with respect to which sublifetime assumptions are in
    /// place. The reason that we store under the fn-id, and not
    /// something more fine-grained, is so that it is easier for
    /// regionck to be sure that it has found *all* the region
    /// obligations (otherwise, it's easy to fail to walk to a
    /// particular node-id).
    ///
    /// Before running `resolve_regions_and_report_errors`, the creator
    /// of the inference context is expected to invoke
    /// [`InferCtxt::process_registered_region_obligations`]
    /// for each body-id in this map, which will process the
    /// obligations within. This is expected to be done 'late enough'
    /// that all type inference variables have been bound and so forth.
    region_obligations: Vec<RegionObligation<'tcx>>,

    /// Caches for opaque type inference.
    opaque_type_storage: OpaqueTypeStorage<'tcx>,
}

impl<'tcx> InferCtxtInner<'tcx> {
    fn new() -> InferCtxtInner<'tcx> {
        InferCtxtInner {
            undo_log: InferCtxtUndoLogs::default(),

            projection_cache: Default::default(),
            type_variable_storage: type_variable::TypeVariableStorage::new(),
            const_unification_storage: ut::UnificationTableStorage::new(),
            int_unification_storage: ut::UnificationTableStorage::new(),
            float_unification_storage: ut::UnificationTableStorage::new(),
            effect_unification_storage: ut::UnificationTableStorage::new(),
            region_constraint_storage: Some(RegionConstraintStorage::new()),
            region_obligations: vec![],
            opaque_type_storage: Default::default(),
        }
    }

    #[inline]
    pub fn region_obligations(&self) -> &[RegionObligation<'tcx>] {
        &self.region_obligations
    }

    #[inline]
    pub fn projection_cache(&mut self) -> traits::ProjectionCache<'_, 'tcx> {
        self.projection_cache.with_log(&mut self.undo_log)
    }

    #[inline]
    fn try_type_variables_probe_ref(
        &self,
        vid: ty::TyVid,
    ) -> Option<&type_variable::TypeVariableValue<'tcx>> {
        // Uses a read-only view of the unification table, this way we don't
        // need an undo log.
        self.type_variable_storage.eq_relations_ref().try_probe_value(vid)
    }

    #[inline]
    fn type_variables(&mut self) -> type_variable::TypeVariableTable<'_, 'tcx> {
        self.type_variable_storage.with_log(&mut self.undo_log)
    }

    #[inline]
    pub fn opaque_types(&mut self) -> opaque_types::OpaqueTypeTable<'_, 'tcx> {
        self.opaque_type_storage.with_log(&mut self.undo_log)
    }

    #[inline]
    fn int_unification_table(&mut self) -> UnificationTable<'_, 'tcx, ty::IntVid> {
        self.int_unification_storage.with_log(&mut self.undo_log)
    }

    #[inline]
    fn float_unification_table(&mut self) -> UnificationTable<'_, 'tcx, ty::FloatVid> {
        self.float_unification_storage.with_log(&mut self.undo_log)
    }

    #[inline]
    fn const_unification_table(&mut self) -> UnificationTable<'_, 'tcx, ConstVidKey<'tcx>> {
        self.const_unification_storage.with_log(&mut self.undo_log)
    }

    fn effect_unification_table(&mut self) -> UnificationTable<'_, 'tcx, EffectVidKey<'tcx>> {
        self.effect_unification_storage.with_log(&mut self.undo_log)
    }

    #[inline]
    pub fn unwrap_region_constraints(&mut self) -> RegionConstraintCollector<'_, 'tcx> {
        self.region_constraint_storage
            .as_mut()
            .expect("region constraints already solved")
            .with_log(&mut self.undo_log)
    }
}

pub struct InferCtxt<'tcx> {
    pub tcx: TyCtxt<'tcx>,

    /// The `DefId` of the item in whose context we are performing inference or typeck.
    /// It is used to check whether an opaque type use is a defining use.
    ///
    /// If it is `DefiningAnchor::Bubble`, we can't resolve opaque types here and need to bubble up
    /// the obligation. This frequently happens for
    /// short lived InferCtxt within queries. The opaque type obligations are forwarded
    /// to the outside until the end up in an `InferCtxt` for typeck or borrowck.
    ///
    /// Its default value is `DefiningAnchor::Error`, this way it is easier to catch errors that
    /// might come up during inference or typeck.
    pub defining_use_anchor: DefiningAnchor,

    /// Whether this inference context should care about region obligations in
    /// the root universe. Most notably, this is used during hir typeck as region
    /// solving is left to borrowck instead.
    pub considering_regions: bool,

    /// If set, this flag causes us to skip the 'leak check' during
    /// higher-ranked subtyping operations. This flag is a temporary one used
    /// to manage the removal of the leak-check: for the time being, we still run the
    /// leak-check, but we issue warnings.
    skip_leak_check: bool,

    pub inner: RefCell<InferCtxtInner<'tcx>>,

    /// Once region inference is done, the values for each variable.
    lexical_region_resolutions: RefCell<Option<LexicalRegionResolutions<'tcx>>>,

    /// Caches the results of trait selection. This cache is used
    /// for things that have to do with the parameters in scope.
    pub selection_cache: select::SelectionCache<'tcx>,

    /// Caches the results of trait evaluation.
    pub evaluation_cache: select::EvaluationCache<'tcx>,

    /// The set of predicates on which errors have been reported, to
    /// avoid reporting the same error twice.
    pub reported_trait_errors:
        RefCell<FxIndexMap<Span, (Vec<ty::Predicate<'tcx>>, ErrorGuaranteed)>>,

    pub reported_signature_mismatch: RefCell<FxHashSet<(Span, Option<Span>)>>,

    /// When an error occurs, we want to avoid reporting "derived"
    /// errors that are due to this original failure. Normally, we
    /// handle this with the `err_count_on_creation` count, which
    /// basically just tracks how many errors were reported when we
    /// started type-checking a fn and checks to see if any new errors
    /// have been reported since then. Not great, but it works.
    ///
    /// However, when errors originated in other passes -- notably
    /// resolve -- this heuristic breaks down. Therefore, we have this
    /// auxiliary flag that one can set whenever one creates a
    /// type-error that is due to an error in a prior pass.
    ///
    /// Don't read this flag directly, call `is_tainted_by_errors()`
    /// and `set_tainted_by_errors()`.
    tainted_by_errors: Cell<Option<ErrorGuaranteed>>,

    /// Track how many errors were reported when this infcx is created.
    /// If the number of errors increases, that's also a sign (like
    /// `tainted_by_errors`) to avoid reporting certain kinds of errors.
    // FIXME(matthewjasper) Merge into `tainted_by_errors`
    err_count_on_creation: usize,

    /// What is the innermost universe we have created? Starts out as
    /// `UniverseIndex::root()` but grows from there as we enter
    /// universal quantifiers.
    ///
    /// N.B., at present, we exclude the universal quantifiers on the
    /// item we are type-checking, and just consider those names as
    /// part of the root universe. So this would only get incremented
    /// when we enter into a higher-ranked (`for<..>`) type or trait
    /// bound.
    universe: Cell<ty::UniverseIndex>,

    /// During coherence we have to assume that other crates may add
    /// additional impls which we currently don't know about.
    ///
    /// To deal with this evaluation, we should be conservative
    /// and consider the possibility of impls from outside this crate.
    /// This comes up primarily when resolving ambiguity. Imagine
    /// there is some trait reference `$0: Bar` where `$0` is an
    /// inference variable. If `intercrate` is true, then we can never
    /// say for sure that this reference is not implemented, even if
    /// there are *no impls at all for `Bar`*, because `$0` could be
    /// bound to some type that in a downstream crate that implements
    /// `Bar`.
    ///
    /// Outside of coherence, we set this to false because we are only
    /// interested in types that the user could actually have written.
    /// In other words, we consider `$0: Bar` to be unimplemented if
    /// there is no type that the user could *actually name* that
    /// would satisfy it. This avoids crippling inference, basically.
    pub intercrate: bool,

    next_trait_solver: bool,

    pub obligation_inspector: Cell<Option<ObligationInspector<'tcx>>>,
}

impl<'tcx> ty::InferCtxtLike for InferCtxt<'tcx> {
    type Interner = TyCtxt<'tcx>;

    fn interner(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    fn universe_of_ty(&self, vid: TyVid) -> Option<ty::UniverseIndex> {
        // FIXME(BoxyUwU): this is kind of jank and means that printing unresolved
        // ty infers will give you the universe of the var it resolved to not the universe
        // it actually had. It also means that if you have a `?0.1` and infer it to `u8` then
        // try to print out `?0.1` it will just print `?0`.
        match self.probe_ty_var(vid) {
            Err(universe) => Some(universe),
            Ok(_) => None,
        }
    }

    fn universe_of_ct(&self, ct: ConstVid) -> Option<ty::UniverseIndex> {
        // Same issue as with `universe_of_ty`
        match self.probe_const_var(ct) {
            Err(universe) => Some(universe),
            Ok(_) => None,
        }
    }

    fn universe_of_lt(&self, lt: ty::RegionVid) -> Option<ty::UniverseIndex> {
        match self.inner.borrow_mut().unwrap_region_constraints().probe_value(lt) {
            Err(universe) => Some(universe),
            Ok(_) => None,
        }
    }

    fn root_ty_var(&self, vid: TyVid) -> TyVid {
        self.root_var(vid)
    }

    fn probe_ty_var(&self, vid: TyVid) -> Option<Ty<'tcx>> {
        self.probe_ty_var(vid).ok()
    }

    fn opportunistic_resolve_lt_var(&self, vid: ty::RegionVid) -> Option<ty::Region<'tcx>> {
        let re = self
            .inner
            .borrow_mut()
            .unwrap_region_constraints()
            .opportunistic_resolve_var(self.tcx, vid);
        if *re == ty::ReVar(vid) { None } else { Some(re) }
    }

    fn root_ct_var(&self, vid: ConstVid) -> ConstVid {
        self.root_const_var(vid)
    }

    fn probe_ct_var(&self, vid: ConstVid) -> Option<ty::Const<'tcx>> {
        self.probe_const_var(vid).ok()
    }
}

/// See the `error_reporting` module for more details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, TypeFoldable, TypeVisitable)]
pub enum ValuePairs<'tcx> {
    Regions(ExpectedFound<ty::Region<'tcx>>),
    Terms(ExpectedFound<ty::Term<'tcx>>),
    Aliases(ExpectedFound<ty::AliasTy<'tcx>>),
    PolyTraitRefs(ExpectedFound<ty::PolyTraitRef<'tcx>>),
    PolySigs(ExpectedFound<ty::PolyFnSig<'tcx>>),
    ExistentialTraitRef(ExpectedFound<ty::PolyExistentialTraitRef<'tcx>>),
    ExistentialProjection(ExpectedFound<ty::PolyExistentialProjection<'tcx>>),
}

impl<'tcx> ValuePairs<'tcx> {
    pub fn ty(&self) -> Option<(Ty<'tcx>, Ty<'tcx>)> {
        if let ValuePairs::Terms(ExpectedFound { expected, found }) = self
            && let Some(expected) = expected.ty()
            && let Some(found) = found.ty()
        {
            Some((expected, found))
        } else {
            None
        }
    }
}

/// The trace designates the path through inference that we took to
/// encounter an error or subtyping constraint.
///
/// See the `error_reporting` module for more details.
#[derive(Clone, Debug)]
pub struct TypeTrace<'tcx> {
    pub cause: ObligationCause<'tcx>,
    pub values: ValuePairs<'tcx>,
}

/// The origin of a `r1 <= r2` constraint.
///
/// See `error_reporting` module for more details
#[derive(Clone, Debug)]
pub enum SubregionOrigin<'tcx> {
    /// Arose from a subtyping relation
    Subtype(Box<TypeTrace<'tcx>>),

    /// When casting `&'a T` to an `&'b Trait` object,
    /// relating `'a` to `'b`.
    RelateObjectBound(Span),

    /// Some type parameter was instantiated with the given type,
    /// and that type must outlive some region.
    RelateParamBound(Span, Ty<'tcx>, Option<Span>),

    /// The given region parameter was instantiated with a region
    /// that must outlive some other region.
    RelateRegionParamBound(Span),

    /// Creating a pointer `b` to contents of another reference.
    Reborrow(Span),

    /// (&'a &'b T) where a >= b
    ReferenceOutlivesReferent(Ty<'tcx>, Span),

    /// Comparing the signature and requirements of an impl method against
    /// the containing trait.
    CompareImplItemObligation {
        span: Span,
        impl_item_def_id: LocalDefId,
        trait_item_def_id: DefId,
    },

    /// Checking that the bounds of a trait's associated type hold for a given impl.
    CheckAssociatedTypeBounds {
        parent: Box<SubregionOrigin<'tcx>>,
        impl_item_def_id: LocalDefId,
        trait_item_def_id: DefId,
    },

    AscribeUserTypeProvePredicate(Span),
}

// `SubregionOrigin` is used a lot. Make sure it doesn't unintentionally get bigger.
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
static_assert_size!(SubregionOrigin<'_>, 32);

impl<'tcx> SubregionOrigin<'tcx> {
    pub fn to_constraint_category(&self) -> ConstraintCategory<'tcx> {
        match self {
            Self::Subtype(type_trace) => type_trace.cause.to_constraint_category(),
            Self::AscribeUserTypeProvePredicate(span) => ConstraintCategory::Predicate(*span),
            _ => ConstraintCategory::BoringNoLocation,
        }
    }
}

/// Times when we replace bound regions with existentials:
#[derive(Clone, Copy, Debug)]
pub enum BoundRegionConversionTime {
    /// when a fn is called
    FnCall,

    /// when two higher-ranked types are compared
    HigherRankedType,

    /// when projecting an associated type
    AssocTypeProjection(DefId),
}

/// Reasons to create a region inference variable.
///
/// See `error_reporting` module for more details.
#[derive(Copy, Clone, Debug)]
pub enum RegionVariableOrigin {
    /// Region variables created for ill-categorized reasons.
    ///
    /// They mostly indicate places in need of refactoring.
    MiscVariable(Span),

    /// Regions created by a `&P` or `[...]` pattern.
    PatternRegion(Span),

    /// Regions created by `&` operator.
    ///
    AddrOfRegion(Span),
    /// Regions created as part of an autoref of a method receiver.
    Autoref(Span),

    /// Regions created as part of an automatic coercion.
    Coercion(Span),

    /// Region variables created as the values for early-bound regions.
    ///
    /// FIXME(@lcnr): This can also store a `DefId`, similar to
    /// `TypeVariableOriginKind::TypeParameterDefinition`.
    RegionParameterDefinition(Span, Symbol),

    /// Region variables created when instantiating a binder with
    /// existential variables, e.g. when calling a function or method.
    BoundRegion(Span, ty::BoundRegionKind, BoundRegionConversionTime),

    UpvarRegion(ty::UpvarId, Span),

    /// This origin is used for the inference variables that we create
    /// during NLL region processing.
    Nll(NllRegionVariableOrigin),
}

#[derive(Copy, Clone, Debug)]
pub enum NllRegionVariableOrigin {
    /// During NLL region processing, we create variables for free
    /// regions that we encounter in the function signature and
    /// elsewhere. This origin indices we've got one of those.
    FreeRegion,

    /// "Universal" instantiation of a higher-ranked region (e.g.,
    /// from a `for<'a> T` binder). Meant to represent "any region".
    Placeholder(ty::PlaceholderRegion),

    Existential {
        /// If this is true, then this variable was created to represent a lifetime
        /// bound in a `for` binder. For example, it might have been created to
        /// represent the lifetime `'a` in a type like `for<'a> fn(&'a u32)`.
        /// Such variables are created when we are trying to figure out if there
        /// is any valid instantiation of `'a` that could fit into some scenario.
        ///
        /// This is used to inform error reporting: in the case that we are trying to
        /// determine whether there is any valid instantiation of a `'a` variable that meets
        /// some constraint C, we want to blame the "source" of that `for` type,
        /// rather than blaming the source of the constraint C.
        from_forall: bool,
    },
}

// FIXME(eddyb) investigate overlap between this and `TyOrConstInferVar`.
#[derive(Copy, Clone, Debug)]
pub enum FixupError {
    UnresolvedIntTy(IntVid),
    UnresolvedFloatTy(FloatVid),
    UnresolvedTy(TyVid),
    UnresolvedConst(ConstVid),
}

/// See the `region_obligations` field for more information.
#[derive(Clone, Debug)]
pub struct RegionObligation<'tcx> {
    pub sub_region: ty::Region<'tcx>,
    pub sup_type: Ty<'tcx>,
    pub origin: SubregionOrigin<'tcx>,
}

impl fmt::Display for FixupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::FixupError::*;

        match *self {
            UnresolvedIntTy(_) => write!(
                f,
                "cannot determine the type of this integer; \
                 add a suffix to specify the type explicitly"
            ),
            UnresolvedFloatTy(_) => write!(
                f,
                "cannot determine the type of this number; \
                 add a suffix to specify the type explicitly"
            ),
            UnresolvedTy(_) => write!(f, "unconstrained type"),
            UnresolvedConst(_) => write!(f, "unconstrained const value"),
        }
    }
}

/// Used to configure inference contexts before their creation.
pub struct InferCtxtBuilder<'tcx> {
    tcx: TyCtxt<'tcx>,
    defining_use_anchor: DefiningAnchor,
    considering_regions: bool,
    skip_leak_check: bool,
    /// Whether we are in coherence mode.
    intercrate: bool,
    /// Whether we should use the new trait solver in the local inference context,
    /// which affects things like which solver is used in `predicate_may_hold`.
    next_trait_solver: bool,
}

#[extension(pub trait TyCtxtInferExt<'tcx>)]
impl<'tcx> TyCtxt<'tcx> {
    fn infer_ctxt(self) -> InferCtxtBuilder<'tcx> {
        InferCtxtBuilder {
            tcx: self,
            defining_use_anchor: DefiningAnchor::Error,
            considering_regions: true,
            skip_leak_check: false,
            intercrate: false,
            next_trait_solver: self.next_trait_solver_globally(),
        }
    }
}

impl<'tcx> InferCtxtBuilder<'tcx> {
    /// Whenever the `InferCtxt` should be able to handle defining uses of opaque types,
    /// you need to call this function. Otherwise the opaque type will be treated opaquely.
    ///
    /// It is only meant to be called in two places, for typeck
    /// (via `Inherited::build`) and for the inference context used
    /// in mir borrowck.
    pub fn with_opaque_type_inference(mut self, defining_use_anchor: DefiningAnchor) -> Self {
        self.defining_use_anchor = defining_use_anchor;
        self
    }

    pub fn with_next_trait_solver(mut self, next_trait_solver: bool) -> Self {
        self.next_trait_solver = next_trait_solver;
        self
    }

    pub fn intercrate(mut self, intercrate: bool) -> Self {
        self.intercrate = intercrate;
        self
    }

    pub fn ignoring_regions(mut self) -> Self {
        self.considering_regions = false;
        self
    }

    pub fn skip_leak_check(mut self, skip_leak_check: bool) -> Self {
        self.skip_leak_check = skip_leak_check;
        self
    }

    /// Given a canonical value `C` as a starting point, create an
    /// inference context that contains each of the bound values
    /// within instantiated as a fresh variable. The `f` closure is
    /// invoked with the new infcx, along with the instantiated value
    /// `V` and a instantiation `S`. This instantiation `S` maps from
    /// the bound values in `C` to their instantiated values in `V`
    /// (in other words, `S(C) = V`).
    pub fn build_with_canonical<T>(
        &mut self,
        span: Span,
        canonical: &Canonical<'tcx, T>,
    ) -> (InferCtxt<'tcx>, T, CanonicalVarValues<'tcx>)
    where
        T: TypeFoldable<TyCtxt<'tcx>>,
    {
        let infcx = self.build();
        let (value, args) = infcx.instantiate_canonical_with_fresh_inference_vars(span, canonical);
        (infcx, value, args)
    }

    pub fn build(&mut self) -> InferCtxt<'tcx> {
        let InferCtxtBuilder {
            tcx,
            defining_use_anchor,
            considering_regions,
            skip_leak_check,
            intercrate,
            next_trait_solver,
        } = *self;
        InferCtxt {
            tcx,
            defining_use_anchor,
            considering_regions,
            skip_leak_check,
            inner: RefCell::new(InferCtxtInner::new()),
            lexical_region_resolutions: RefCell::new(None),
            selection_cache: Default::default(),
            evaluation_cache: Default::default(),
            reported_trait_errors: Default::default(),
            reported_signature_mismatch: Default::default(),
            tainted_by_errors: Cell::new(None),
            err_count_on_creation: tcx.dcx().err_count_excluding_lint_errs(),
            universe: Cell::new(ty::UniverseIndex::ROOT),
            intercrate,
            next_trait_solver,
            obligation_inspector: Cell::new(None),
        }
    }
}

impl<'tcx, T> InferOk<'tcx, T> {
    /// Extracts `value`, registering any obligations into `fulfill_cx`.
    pub fn into_value_registering_obligations(
        self,
        infcx: &InferCtxt<'tcx>,
        fulfill_cx: &mut dyn TraitEngine<'tcx>,
    ) -> T {
        let InferOk { value, obligations } = self;
        fulfill_cx.register_predicate_obligations(infcx, obligations);
        value
    }
}

impl<'tcx> InferOk<'tcx, ()> {
    pub fn into_obligations(self) -> PredicateObligations<'tcx> {
        self.obligations
    }
}

impl<'tcx> InferCtxt<'tcx> {
    pub fn dcx(&self) -> &'tcx DiagCtxt {
        self.tcx.dcx()
    }

    pub fn next_trait_solver(&self) -> bool {
        self.next_trait_solver
    }

    /// Creates a `TypeErrCtxt` for emitting various inference errors.
    /// During typeck, use `FnCtxt::err_ctxt` instead.
    pub fn err_ctxt(&self) -> TypeErrCtxt<'_, 'tcx> {
        TypeErrCtxt {
            infcx: self,
            sub_relations: Default::default(),
            typeck_results: None,
            fallback_has_occurred: false,
            normalize_fn_sig: Box::new(|fn_sig| fn_sig),
            autoderef_steps: Box::new(|ty| {
                debug_assert!(false, "shouldn't be using autoderef_steps outside of typeck");
                vec![(ty, vec![])]
            }),
        }
    }

    pub fn freshen<T: TypeFoldable<TyCtxt<'tcx>>>(&self, t: T) -> T {
        t.fold_with(&mut self.freshener())
    }

    /// Returns the origin of the type variable identified by `vid`, or `None`
    /// if this is not a type variable.
    ///
    /// No attempt is made to resolve `ty`.
    pub fn type_var_origin(&self, ty: Ty<'tcx>) -> Option<TypeVariableOrigin> {
        match *ty.kind() {
            ty::Infer(ty::TyVar(vid)) => {
                Some(self.inner.borrow_mut().type_variables().var_origin(vid))
            }
            _ => None,
        }
    }

    pub fn freshener<'b>(&'b self) -> TypeFreshener<'b, 'tcx> {
        freshen::TypeFreshener::new(self)
    }

    pub fn unresolved_variables(&self) -> Vec<Ty<'tcx>> {
        let mut inner = self.inner.borrow_mut();
        let mut vars: Vec<Ty<'_>> = inner
            .type_variables()
            .unresolved_variables()
            .into_iter()
            .map(|t| Ty::new_var(self.tcx, t))
            .collect();
        vars.extend(
            (0..inner.int_unification_table().len())
                .map(|i| ty::IntVid::from_u32(i as u32))
                .filter(|&vid| inner.int_unification_table().probe_value(vid).is_none())
                .map(|v| Ty::new_int_var(self.tcx, v)),
        );
        vars.extend(
            (0..inner.float_unification_table().len())
                .map(|i| ty::FloatVid::from_u32(i as u32))
                .filter(|&vid| inner.float_unification_table().probe_value(vid).is_none())
                .map(|v| Ty::new_float_var(self.tcx, v)),
        );
        vars
    }

    pub fn unsolved_effects(&self) -> Vec<ty::Const<'tcx>> {
        let mut inner = self.inner.borrow_mut();
        let mut table = inner.effect_unification_table();

        (0..table.len())
            .map(|i| ty::EffectVid::from_usize(i))
            .filter(|&vid| table.probe_value(vid).is_unknown())
            .map(|v| {
                ty::Const::new_infer(self.tcx, ty::InferConst::EffectVar(v), self.tcx.types.bool)
            })
            .collect()
    }

    fn combine_fields<'a>(
        &'a self,
        trace: TypeTrace<'tcx>,
        param_env: ty::ParamEnv<'tcx>,
        define_opaque_types: DefineOpaqueTypes,
    ) -> CombineFields<'a, 'tcx> {
        CombineFields {
            infcx: self,
            trace,
            param_env,
            obligations: PredicateObligations::new(),
            define_opaque_types,
        }
    }

    pub fn can_sub<T>(&self, param_env: ty::ParamEnv<'tcx>, expected: T, actual: T) -> bool
    where
        T: at::ToTrace<'tcx>,
    {
        let origin = &ObligationCause::dummy();
        self.probe(|_| {
            self.at(origin, param_env).sub(DefineOpaqueTypes::No, expected, actual).is_ok()
        })
    }

    pub fn can_eq<T>(&self, param_env: ty::ParamEnv<'tcx>, a: T, b: T) -> bool
    where
        T: at::ToTrace<'tcx>,
    {
        let origin = &ObligationCause::dummy();
        self.probe(|_| self.at(origin, param_env).eq(DefineOpaqueTypes::No, a, b).is_ok())
    }

    #[instrument(skip(self), level = "debug")]
    pub fn sub_regions(
        &self,
        origin: SubregionOrigin<'tcx>,
        a: ty::Region<'tcx>,
        b: ty::Region<'tcx>,
    ) {
        self.inner.borrow_mut().unwrap_region_constraints().make_subregion(origin, a, b);
    }

    /// Require that the region `r` be equal to one of the regions in
    /// the set `regions`.
    #[instrument(skip(self), level = "debug")]
    pub fn member_constraint(
        &self,
        key: ty::OpaqueTypeKey<'tcx>,
        definition_span: Span,
        hidden_ty: Ty<'tcx>,
        region: ty::Region<'tcx>,
        in_regions: &Lrc<Vec<ty::Region<'tcx>>>,
    ) {
        self.inner.borrow_mut().unwrap_region_constraints().member_constraint(
            key,
            definition_span,
            hidden_ty,
            region,
            in_regions,
        );
    }

    /// Processes a `Coerce` predicate from the fulfillment context.
    /// This is NOT the preferred way to handle coercion, which is to
    /// invoke `FnCtxt::coerce` or a similar method (see `coercion.rs`).
    ///
    /// This method here is actually a fallback that winds up being
    /// invoked when `FnCtxt::coerce` encounters unresolved type variables
    /// and records a coercion predicate. Presently, this method is equivalent
    /// to `subtype_predicate` -- that is, "coercing" `a` to `b` winds up
    /// actually requiring `a <: b`. This is of course a valid coercion,
    /// but it's not as flexible as `FnCtxt::coerce` would be.
    ///
    /// (We may refactor this in the future, but there are a number of
    /// practical obstacles. Among other things, `FnCtxt::coerce` presently
    /// records adjustments that are required on the HIR in order to perform
    /// the coercion, and we don't currently have a way to manage that.)
    pub fn coerce_predicate(
        &self,
        cause: &ObligationCause<'tcx>,
        param_env: ty::ParamEnv<'tcx>,
        predicate: ty::PolyCoercePredicate<'tcx>,
    ) -> Result<InferResult<'tcx, ()>, (TyVid, TyVid)> {
        let subtype_predicate = predicate.map_bound(|p| ty::SubtypePredicate {
            a_is_expected: false, // when coercing from `a` to `b`, `b` is expected
            a: p.a,
            b: p.b,
        });
        self.subtype_predicate(cause, param_env, subtype_predicate)
    }

    pub fn subtype_predicate(
        &self,
        cause: &ObligationCause<'tcx>,
        param_env: ty::ParamEnv<'tcx>,
        predicate: ty::PolySubtypePredicate<'tcx>,
    ) -> Result<InferResult<'tcx, ()>, (TyVid, TyVid)> {
        // Check for two unresolved inference variables, in which case we can
        // make no progress. This is partly a micro-optimization, but it's
        // also an opportunity to "sub-unify" the variables. This isn't
        // *necessary* to prevent cycles, because they would eventually be sub-unified
        // anyhow during generalization, but it helps with diagnostics (we can detect
        // earlier that they are sub-unified).
        //
        // Note that we can just skip the binders here because
        // type variables can't (at present, at
        // least) capture any of the things bound by this binder.
        //
        // Note that this sub here is not just for diagnostics - it has semantic
        // effects as well.
        let r_a = self.shallow_resolve(predicate.skip_binder().a);
        let r_b = self.shallow_resolve(predicate.skip_binder().b);
        match (r_a.kind(), r_b.kind()) {
            (&ty::Infer(ty::TyVar(a_vid)), &ty::Infer(ty::TyVar(b_vid))) => {
                return Err((a_vid, b_vid));
            }
            _ => {}
        }

        self.enter_forall(predicate, |ty::SubtypePredicate { a_is_expected, a, b }| {
            if a_is_expected {
                Ok(self.at(cause, param_env).sub(DefineOpaqueTypes::No, a, b))
            } else {
                Ok(self.at(cause, param_env).sup(DefineOpaqueTypes::No, b, a))
            }
        })
    }

    pub fn region_outlives_predicate(
        &self,
        cause: &traits::ObligationCause<'tcx>,
        predicate: ty::PolyRegionOutlivesPredicate<'tcx>,
    ) {
        self.enter_forall(predicate, |ty::OutlivesPredicate(r_a, r_b)| {
            let origin = SubregionOrigin::from_obligation_cause(cause, || {
                RelateRegionParamBound(cause.span)
            });
            self.sub_regions(origin, r_b, r_a); // `b : a` ==> `a <= b`
        })
    }

    /// Number of type variables created so far.
    pub fn num_ty_vars(&self) -> usize {
        self.inner.borrow_mut().type_variables().num_vars()
    }

    pub fn next_ty_var_id(&self, origin: TypeVariableOrigin) -> TyVid {
        self.inner.borrow_mut().type_variables().new_var(self.universe(), origin)
    }

    pub fn next_ty_var(&self, origin: TypeVariableOrigin) -> Ty<'tcx> {
        Ty::new_var(self.tcx, self.next_ty_var_id(origin))
    }

    pub fn next_ty_var_id_in_universe(
        &self,
        origin: TypeVariableOrigin,
        universe: ty::UniverseIndex,
    ) -> TyVid {
        self.inner.borrow_mut().type_variables().new_var(universe, origin)
    }

    pub fn next_ty_var_in_universe(
        &self,
        origin: TypeVariableOrigin,
        universe: ty::UniverseIndex,
    ) -> Ty<'tcx> {
        let vid = self.next_ty_var_id_in_universe(origin, universe);
        Ty::new_var(self.tcx, vid)
    }

    pub fn next_const_var(&self, ty: Ty<'tcx>, origin: ConstVariableOrigin) -> ty::Const<'tcx> {
        ty::Const::new_var(self.tcx, self.next_const_var_id(origin), ty)
    }

    pub fn next_const_var_in_universe(
        &self,
        ty: Ty<'tcx>,
        origin: ConstVariableOrigin,
        universe: ty::UniverseIndex,
    ) -> ty::Const<'tcx> {
        let vid = self
            .inner
            .borrow_mut()
            .const_unification_table()
            .new_key(ConstVariableValue::Unknown { origin, universe })
            .vid;
        ty::Const::new_var(self.tcx, vid, ty)
    }

    pub fn next_const_var_id(&self, origin: ConstVariableOrigin) -> ConstVid {
        self.inner
            .borrow_mut()
            .const_unification_table()
            .new_key(ConstVariableValue::Unknown { origin, universe: self.universe() })
            .vid
    }

    fn next_int_var_id(&self) -> IntVid {
        self.inner.borrow_mut().int_unification_table().new_key(None)
    }

    pub fn next_int_var(&self) -> Ty<'tcx> {
        Ty::new_int_var(self.tcx, self.next_int_var_id())
    }

    fn next_float_var_id(&self) -> FloatVid {
        self.inner.borrow_mut().float_unification_table().new_key(None)
    }

    pub fn next_float_var(&self) -> Ty<'tcx> {
        Ty::new_float_var(self.tcx, self.next_float_var_id())
    }

    /// Creates a fresh region variable with the next available index.
    /// The variable will be created in the maximum universe created
    /// thus far, allowing it to name any region created thus far.
    pub fn next_region_var(&self, origin: RegionVariableOrigin) -> ty::Region<'tcx> {
        self.next_region_var_in_universe(origin, self.universe())
    }

    /// Creates a fresh region variable with the next available index
    /// in the given universe; typically, you can use
    /// `next_region_var` and just use the maximal universe.
    pub fn next_region_var_in_universe(
        &self,
        origin: RegionVariableOrigin,
        universe: ty::UniverseIndex,
    ) -> ty::Region<'tcx> {
        let region_var =
            self.inner.borrow_mut().unwrap_region_constraints().new_region_var(universe, origin);
        ty::Region::new_var(self.tcx, region_var)
    }

    /// Return the universe that the region `r` was created in. For
    /// most regions (e.g., `'static`, named regions from the user,
    /// etc) this is the root universe U0. For inference variables or
    /// placeholders, however, it will return the universe which they
    /// are associated.
    pub fn universe_of_region(&self, r: ty::Region<'tcx>) -> ty::UniverseIndex {
        self.inner.borrow_mut().unwrap_region_constraints().universe(r)
    }

    /// Number of region variables created so far.
    pub fn num_region_vars(&self) -> usize {
        self.inner.borrow_mut().unwrap_region_constraints().num_region_vars()
    }

    /// Just a convenient wrapper of `next_region_var` for using during NLL.
    #[instrument(skip(self), level = "debug")]
    pub fn next_nll_region_var(&self, origin: NllRegionVariableOrigin) -> ty::Region<'tcx> {
        self.next_region_var(RegionVariableOrigin::Nll(origin))
    }

    /// Just a convenient wrapper of `next_region_var` for using during NLL.
    #[instrument(skip(self), level = "debug")]
    pub fn next_nll_region_var_in_universe(
        &self,
        origin: NllRegionVariableOrigin,
        universe: ty::UniverseIndex,
    ) -> ty::Region<'tcx> {
        self.next_region_var_in_universe(RegionVariableOrigin::Nll(origin), universe)
    }

    pub fn var_for_def(&self, span: Span, param: &ty::GenericParamDef) -> GenericArg<'tcx> {
        match param.kind {
            GenericParamDefKind::Lifetime => {
                // Create a region inference variable for the given
                // region parameter definition.
                self.next_region_var(RegionParameterDefinition(span, param.name)).into()
            }
            GenericParamDefKind::Type { .. } => {
                // Create a type inference variable for the given
                // type parameter definition. The generic parameters are
                // for actual parameters that may be referred to by
                // the default of this type parameter, if it exists.
                // e.g., `struct Foo<A, B, C = (A, B)>(...);` when
                // used in a path such as `Foo::<T, U>::new()` will
                // use an inference variable for `C` with `[T, U]`
                // as the generic parameters for the default, `(T, U)`.
                let ty_var_id = self.inner.borrow_mut().type_variables().new_var(
                    self.universe(),
                    TypeVariableOrigin {
                        kind: TypeVariableOriginKind::TypeParameterDefinition(
                            param.name,
                            param.def_id,
                        ),
                        span,
                    },
                );

                Ty::new_var(self.tcx, ty_var_id).into()
            }
            GenericParamDefKind::Const { is_host_effect, .. } => {
                if is_host_effect {
                    return self.var_for_effect(param);
                }
                let origin = ConstVariableOrigin {
                    kind: ConstVariableOriginKind::ConstParameterDefinition(
                        param.name,
                        param.def_id,
                    ),
                    span,
                };
                let const_var_id = self
                    .inner
                    .borrow_mut()
                    .const_unification_table()
                    .new_key(ConstVariableValue::Unknown { origin, universe: self.universe() })
                    .vid;
                ty::Const::new_var(
                    self.tcx,
                    const_var_id,
                    self.tcx
                        .type_of(param.def_id)
                        .no_bound_vars()
                        .expect("const parameter types cannot be generic"),
                )
                .into()
            }
        }
    }

    pub fn var_for_effect(&self, param: &ty::GenericParamDef) -> GenericArg<'tcx> {
        let effect_vid =
            self.inner.borrow_mut().effect_unification_table().new_key(EffectVarValue::Unknown).vid;
        let ty = self
            .tcx
            .type_of(param.def_id)
            .no_bound_vars()
            .expect("const parameter types cannot be generic");
        debug_assert_eq!(self.tcx.types.bool, ty);
        ty::Const::new_infer(self.tcx, ty::InferConst::EffectVar(effect_vid), ty).into()
    }

    /// Given a set of generics defined on a type or impl, returns the generic parameters mapping each
    /// type/region parameter to a fresh inference variable.
    pub fn fresh_args_for_item(&self, span: Span, def_id: DefId) -> GenericArgsRef<'tcx> {
        GenericArgs::for_item(self.tcx, def_id, |param, _| self.var_for_def(span, param))
    }

    /// Returns `true` if errors have been reported since this infcx was
    /// created. This is sometimes used as a heuristic to skip
    /// reporting errors that often occur as a result of earlier
    /// errors, but where it's hard to be 100% sure (e.g., unresolved
    /// inference variables, regionck errors).
    #[must_use = "this method does not have any side effects"]
    pub fn tainted_by_errors(&self) -> Option<ErrorGuaranteed> {
        if let Some(guar) = self.tainted_by_errors.get() {
            Some(guar)
        } else if self.dcx().err_count_excluding_lint_errs() > self.err_count_on_creation {
            // Errors reported since this infcx was made. Lint errors are
            // excluded to avoid some being swallowed in the presence of
            // non-lint errors. (It's arguable whether or not this exclusion is
            // important.)
            let guar = self.dcx().has_errors().unwrap();
            self.set_tainted_by_errors(guar);
            Some(guar)
        } else {
            None
        }
    }

    /// Set the "tainted by errors" flag to true. We call this when we
    /// observe an error from a prior pass.
    pub fn set_tainted_by_errors(&self, e: ErrorGuaranteed) {
        debug!("set_tainted_by_errors(ErrorGuaranteed)");
        self.tainted_by_errors.set(Some(e));
    }

    pub fn region_var_origin(&self, vid: ty::RegionVid) -> RegionVariableOrigin {
        let mut inner = self.inner.borrow_mut();
        let inner = &mut *inner;
        inner.unwrap_region_constraints().var_origin(vid)
    }

    /// Clone the list of variable regions. This is used only during NLL processing
    /// to put the set of region variables into the NLL region context.
    pub fn get_region_var_origins(&self) -> VarInfos {
        let mut inner = self.inner.borrow_mut();
        let (var_infos, data) = inner
            .region_constraint_storage
            // We clone instead of taking because borrowck still wants to use
            // the inference context after calling this for diagnostics
            // and the new trait solver.
            .clone()
            .expect("regions already resolved")
            .with_log(&mut inner.undo_log)
            .into_infos_and_data();
        assert!(data.is_empty());
        var_infos
    }

    #[instrument(level = "debug", skip(self), ret)]
    pub fn take_opaque_types(&self) -> opaque_types::OpaqueTypeMap<'tcx> {
        debug_assert_ne!(self.defining_use_anchor, DefiningAnchor::Error);
        std::mem::take(&mut self.inner.borrow_mut().opaque_type_storage.opaque_types)
    }

    #[instrument(level = "debug", skip(self), ret)]
    pub fn clone_opaque_types(&self) -> opaque_types::OpaqueTypeMap<'tcx> {
        debug_assert_ne!(self.defining_use_anchor, DefiningAnchor::Error);
        self.inner.borrow().opaque_type_storage.opaque_types.clone()
    }

    pub fn ty_to_string(&self, t: Ty<'tcx>) -> String {
        self.resolve_vars_if_possible(t).to_string()
    }

    /// If `TyVar(vid)` resolves to a type, return that type. Else, return the
    /// universe index of `TyVar(vid)`.
    pub fn probe_ty_var(&self, vid: TyVid) -> Result<Ty<'tcx>, ty::UniverseIndex> {
        use self::type_variable::TypeVariableValue;

        match self.inner.borrow_mut().type_variables().probe(vid) {
            TypeVariableValue::Known { value } => Ok(value),
            TypeVariableValue::Unknown { universe } => Err(universe),
        }
    }

    /// Resolve any type variables found in `value` -- but only one
    /// level. So, if the variable `?X` is bound to some type
    /// `Foo<?Y>`, then this would return `Foo<?Y>` (but `?Y` may
    /// itself be bound to a type).
    ///
    /// Useful when you only need to inspect the outermost level of
    /// the type and don't care about nested types (or perhaps you
    /// will be resolving them as well, e.g. in a loop).
    pub fn shallow_resolve<T>(&self, value: T) -> T
    where
        T: TypeFoldable<TyCtxt<'tcx>>,
    {
        value.fold_with(&mut ShallowResolver { infcx: self })
    }

    pub fn root_var(&self, var: ty::TyVid) -> ty::TyVid {
        self.inner.borrow_mut().type_variables().root_var(var)
    }

    pub fn root_const_var(&self, var: ty::ConstVid) -> ty::ConstVid {
        self.inner.borrow_mut().const_unification_table().find(var).vid
    }

    pub fn root_effect_var(&self, var: ty::EffectVid) -> ty::EffectVid {
        self.inner.borrow_mut().effect_unification_table().find(var).vid
    }

    /// Resolves an int var to a rigid int type, if it was constrained to one,
    /// or else the root int var in the unification table.
    pub fn opportunistic_resolve_int_var(&self, vid: ty::IntVid) -> Ty<'tcx> {
        let mut inner = self.inner.borrow_mut();
        if let Some(value) = inner.int_unification_table().probe_value(vid) {
            value.to_type(self.tcx)
        } else {
            Ty::new_int_var(self.tcx, inner.int_unification_table().find(vid))
        }
    }

    /// Resolves a float var to a rigid int type, if it was constrained to one,
    /// or else the root float var in the unification table.
    pub fn opportunistic_resolve_float_var(&self, vid: ty::FloatVid) -> Ty<'tcx> {
        let mut inner = self.inner.borrow_mut();
        if let Some(value) = inner.float_unification_table().probe_value(vid) {
            value.to_type(self.tcx)
        } else {
            Ty::new_float_var(self.tcx, inner.float_unification_table().find(vid))
        }
    }

    /// Where possible, replaces type/const variables in
    /// `value` with their final value. Note that region variables
    /// are unaffected. If a type/const variable has not been unified, it
    /// is left as is. This is an idempotent operation that does
    /// not affect inference state in any way and so you can do it
    /// at will.
    pub fn resolve_vars_if_possible<T>(&self, value: T) -> T
    where
        T: TypeFoldable<TyCtxt<'tcx>>,
    {
        if !value.has_non_region_infer() {
            return value;
        }
        let mut r = resolve::OpportunisticVarResolver::new(self);
        value.fold_with(&mut r)
    }

    pub fn resolve_numeric_literals_with_default<T>(&self, value: T) -> T
    where
        T: TypeFoldable<TyCtxt<'tcx>>,
    {
        if !value.has_infer() {
            return value; // Avoid duplicated type-folding.
        }
        let mut r = InferenceLiteralEraser { tcx: self.tcx };
        value.fold_with(&mut r)
    }

    pub fn probe_const_var(&self, vid: ty::ConstVid) -> Result<ty::Const<'tcx>, ty::UniverseIndex> {
        match self.inner.borrow_mut().const_unification_table().probe_value(vid) {
            ConstVariableValue::Known { value } => Ok(value),
            ConstVariableValue::Unknown { origin: _, universe } => Err(universe),
        }
    }

    pub fn probe_effect_var(&self, vid: EffectVid) -> Option<ty::Const<'tcx>> {
        self.inner.borrow_mut().effect_unification_table().probe_value(vid).known()
    }

    /// Attempts to resolve all type/region/const variables in
    /// `value`. Region inference must have been run already (e.g.,
    /// by calling `resolve_regions_and_report_errors`). If some
    /// variable was never unified, an `Err` results.
    ///
    /// This method is idempotent, but it not typically not invoked
    /// except during the writeback phase.
    pub fn fully_resolve<T: TypeFoldable<TyCtxt<'tcx>>>(&self, value: T) -> FixupResult<T> {
        match resolve::fully_resolve(self, value) {
            Ok(value) => {
                if value.has_non_region_infer() {
                    bug!("`{value:?}` is not fully resolved");
                }
                if value.has_infer_regions() {
                    let guar =
                        self.tcx.dcx().delayed_bug(format!("`{value:?}` is not fully resolved"));
                    Ok(self.tcx.fold_regions(value, |re, _| {
                        if re.is_var() { ty::Region::new_error(self.tcx, guar) } else { re }
                    }))
                } else {
                    Ok(value)
                }
            }
            Err(e) => Err(e),
        }
    }

    // Instantiates the bound variables in a given binder with fresh inference
    // variables in the current universe.
    //
    // Use this method if you'd like to find some generic parameters of the binder's
    // variables (e.g. during a method call). If there isn't a [`BoundRegionConversionTime`]
    // that corresponds to your use case, consider whether or not you should
    // use [`InferCtxt::enter_forall`] instead.
    pub fn instantiate_binder_with_fresh_vars<T>(
        &self,
        span: Span,
        lbrct: BoundRegionConversionTime,
        value: ty::Binder<'tcx, T>,
    ) -> T
    where
        T: TypeFoldable<TyCtxt<'tcx>> + Copy,
    {
        if let Some(inner) = value.no_bound_vars() {
            return inner;
        }

        struct ToFreshVars<'a, 'tcx> {
            infcx: &'a InferCtxt<'tcx>,
            span: Span,
            lbrct: BoundRegionConversionTime,
            map: FxHashMap<ty::BoundVar, ty::GenericArg<'tcx>>,
        }

        impl<'tcx> BoundVarReplacerDelegate<'tcx> for ToFreshVars<'_, 'tcx> {
            fn replace_region(&mut self, br: ty::BoundRegion) -> ty::Region<'tcx> {
                self.map
                    .entry(br.var)
                    .or_insert_with(|| {
                        self.infcx
                            .next_region_var(BoundRegion(self.span, br.kind, self.lbrct))
                            .into()
                    })
                    .expect_region()
            }
            fn replace_ty(&mut self, bt: ty::BoundTy) -> Ty<'tcx> {
                self.map
                    .entry(bt.var)
                    .or_insert_with(|| {
                        self.infcx
                            .next_ty_var(TypeVariableOrigin {
                                kind: TypeVariableOriginKind::MiscVariable,
                                span: self.span,
                            })
                            .into()
                    })
                    .expect_ty()
            }
            fn replace_const(&mut self, bv: ty::BoundVar, ty: Ty<'tcx>) -> ty::Const<'tcx> {
                self.map
                    .entry(bv)
                    .or_insert_with(|| {
                        self.infcx
                            .next_const_var(
                                ty,
                                ConstVariableOrigin {
                                    kind: ConstVariableOriginKind::MiscVariable,
                                    span: self.span,
                                },
                            )
                            .into()
                    })
                    .expect_const()
            }
        }
        let delegate = ToFreshVars { infcx: self, span, lbrct, map: Default::default() };
        self.tcx.replace_bound_vars_uncached(value, delegate)
    }

    /// See the [`region_constraints::RegionConstraintCollector::verify_generic_bound`] method.
    pub fn verify_generic_bound(
        &self,
        origin: SubregionOrigin<'tcx>,
        kind: GenericKind<'tcx>,
        a: ty::Region<'tcx>,
        bound: VerifyBound<'tcx>,
    ) {
        debug!("verify_generic_bound({:?}, {:?} <: {:?})", kind, a, bound);

        self.inner
            .borrow_mut()
            .unwrap_region_constraints()
            .verify_generic_bound(origin, kind, a, bound);
    }

    /// Obtains the latest type of the given closure; this may be a
    /// closure in the current function, in which case its
    /// `ClosureKind` may not yet be known.
    pub fn closure_kind(&self, closure_ty: Ty<'tcx>) -> Option<ty::ClosureKind> {
        let unresolved_kind_ty = match *closure_ty.kind() {
            ty::Closure(_, args) => args.as_closure().kind_ty(),
            ty::CoroutineClosure(_, args) => args.as_coroutine_closure().kind_ty(),
            _ => bug!("unexpected type {closure_ty}"),
        };
        let closure_kind_ty = self.shallow_resolve(unresolved_kind_ty);
        closure_kind_ty.to_opt_closure_kind()
    }

    /// Clears the selection, evaluation, and projection caches. This is useful when
    /// repeatedly attempting to select an `Obligation` while changing only
    /// its `ParamEnv`, since `FulfillmentContext` doesn't use probing.
    pub fn clear_caches(&self) {
        self.selection_cache.clear();
        self.evaluation_cache.clear();
        self.inner.borrow_mut().projection_cache().clear();
    }

    pub fn universe(&self) -> ty::UniverseIndex {
        self.universe.get()
    }

    /// Creates and return a fresh universe that extends all previous
    /// universes. Updates `self.universe` to that new universe.
    pub fn create_next_universe(&self) -> ty::UniverseIndex {
        let u = self.universe.get().next_universe();
        debug!("create_next_universe {u:?}");
        self.universe.set(u);
        u
    }

    pub fn try_const_eval_resolve(
        &self,
        param_env: ty::ParamEnv<'tcx>,
        unevaluated: ty::UnevaluatedConst<'tcx>,
        ty: Ty<'tcx>,
        span: Option<Span>,
    ) -> Result<ty::Const<'tcx>, ErrorHandled> {
        match self.const_eval_resolve(param_env, unevaluated, span) {
            Ok(Some(val)) => Ok(ty::Const::new_value(self.tcx, val, ty)),
            Ok(None) => {
                let tcx = self.tcx;
                let def_id = unevaluated.def;
                span_bug!(
                    tcx.def_span(def_id),
                    "unable to construct a constant value for the unevaluated constant {:?}",
                    unevaluated
                );
            }
            Err(err) => Err(err),
        }
    }

    /// Resolves and evaluates a constant.
    ///
    /// The constant can be located on a trait like `<A as B>::C`, in which case the given
    /// generic parameters and environment are used to resolve the constant. Alternatively if the
    /// constant has generic parameters in scope the instantiations are used to evaluate the value of
    /// the constant. For example in `fn foo<T>() { let _ = [0; bar::<T>()]; }` the repeat count
    /// constant `bar::<T>()` requires a instantiation for `T`, if the instantiation for `T` is still
    /// too generic for the constant to be evaluated then `Err(ErrorHandled::TooGeneric)` is
    /// returned.
    ///
    /// This handles inferences variables within both `param_env` and `args` by
    /// performing the operation on their respective canonical forms.
    #[instrument(skip(self), level = "debug")]
    pub fn const_eval_resolve(
        &self,
        mut param_env: ty::ParamEnv<'tcx>,
        unevaluated: ty::UnevaluatedConst<'tcx>,
        span: Option<Span>,
    ) -> EvalToValTreeResult<'tcx> {
        let mut args = self.resolve_vars_if_possible(unevaluated.args);
        debug!(?args);

        // Postpone the evaluation of constants whose args depend on inference
        // variables
        let tcx = self.tcx;
        if args.has_non_region_infer() {
            if let Some(ct) = tcx.thir_abstract_const(unevaluated.def)? {
                let ct = tcx.expand_abstract_consts(ct.instantiate(tcx, args));
                if let Err(e) = ct.error_reported() {
                    return Err(ErrorHandled::Reported(
                        e.into(),
                        span.unwrap_or(rustc_span::DUMMY_SP),
                    ));
                } else if ct.has_non_region_infer() || ct.has_non_region_param() {
                    return Err(ErrorHandled::TooGeneric(span.unwrap_or(rustc_span::DUMMY_SP)));
                } else {
                    args = replace_param_and_infer_args_with_placeholder(tcx, args);
                }
            } else {
                args = GenericArgs::identity_for_item(tcx, unevaluated.def);
                param_env = tcx.param_env(unevaluated.def);
            }
        }

        let param_env_erased = tcx.erase_regions(param_env);
        let args_erased = tcx.erase_regions(args);
        debug!(?param_env_erased);
        debug!(?args_erased);

        let unevaluated = ty::UnevaluatedConst { def: unevaluated.def, args: args_erased };

        // The return value is the evaluated value which doesn't contain any reference to inference
        // variables, thus we don't need to instantiate back the original values.
        tcx.const_eval_resolve_for_typeck(param_env_erased, unevaluated, span)
    }

    /// The returned function is used in a fast path. If it returns `true` the variable is
    /// unchanged, `false` indicates that the status is unknown.
    #[inline]
    pub fn is_ty_infer_var_definitely_unchanged<'a>(
        &'a self,
    ) -> (impl Fn(TyOrConstInferVar) -> bool + Captures<'tcx> + 'a) {
        // This hoists the borrow/release out of the loop body.
        let inner = self.inner.try_borrow();

        return move |infer_var: TyOrConstInferVar| match (infer_var, &inner) {
            (TyOrConstInferVar::Ty(ty_var), Ok(inner)) => {
                use self::type_variable::TypeVariableValue;

                matches!(
                    inner.try_type_variables_probe_ref(ty_var),
                    Some(TypeVariableValue::Unknown { .. })
                )
            }
            _ => false,
        };
    }

    /// `ty_or_const_infer_var_changed` is equivalent to one of these two:
    ///   * `shallow_resolve(ty) != ty` (where `ty.kind = ty::Infer(_)`)
    ///   * `shallow_resolve(ct) != ct` (where `ct.kind = ty::ConstKind::Infer(_)`)
    ///
    /// However, `ty_or_const_infer_var_changed` is more efficient. It's always
    /// inlined, despite being large, because it has only two call sites that
    /// are extremely hot (both in `traits::fulfill`'s checking of `stalled_on`
    /// inference variables), and it handles both `Ty` and `ty::Const` without
    /// having to resort to storing full `GenericArg`s in `stalled_on`.
    #[inline(always)]
    pub fn ty_or_const_infer_var_changed(&self, infer_var: TyOrConstInferVar) -> bool {
        match infer_var {
            TyOrConstInferVar::Ty(v) => {
                use self::type_variable::TypeVariableValue;

                // If `inlined_probe` returns a `Known` value, it never equals
                // `ty::Infer(ty::TyVar(v))`.
                match self.inner.borrow_mut().type_variables().inlined_probe(v) {
                    TypeVariableValue::Unknown { .. } => false,
                    TypeVariableValue::Known { .. } => true,
                }
            }

            TyOrConstInferVar::TyInt(v) => {
                // If `inlined_probe_value` returns a value it's always a
                // `ty::Int(_)` or `ty::UInt(_)`, which never matches a
                // `ty::Infer(_)`.
                self.inner.borrow_mut().int_unification_table().inlined_probe_value(v).is_some()
            }

            TyOrConstInferVar::TyFloat(v) => {
                // If `probe_value` returns a value it's always a
                // `ty::Float(_)`, which never matches a `ty::Infer(_)`.
                //
                // Not `inlined_probe_value(v)` because this call site is colder.
                self.inner.borrow_mut().float_unification_table().probe_value(v).is_some()
            }

            TyOrConstInferVar::Const(v) => {
                // If `probe_value` returns a `Known` value, it never equals
                // `ty::ConstKind::Infer(ty::InferConst::Var(v))`.
                //
                // Not `inlined_probe_value(v)` because this call site is colder.
                match self.inner.borrow_mut().const_unification_table().probe_value(v) {
                    ConstVariableValue::Unknown { .. } => false,
                    ConstVariableValue::Known { .. } => true,
                }
            }

            TyOrConstInferVar::Effect(v) => {
                // If `probe_value` returns `Some`, it never equals
                // `ty::ConstKind::Infer(ty::InferConst::Effect(v))`.
                //
                // Not `inlined_probe_value(v)` because this call site is colder.
                self.probe_effect_var(v).is_some()
            }
        }
    }

    /// Attach a callback to be invoked on each root obligation evaluated in the new trait solver.
    pub fn attach_obligation_inspector(&self, inspector: ObligationInspector<'tcx>) {
        debug_assert!(
            self.obligation_inspector.get().is_none(),
            "shouldn't override a set obligation inspector"
        );
        self.obligation_inspector.set(Some(inspector));
    }
}

impl<'tcx> TypeErrCtxt<'_, 'tcx> {
    // [Note-Type-error-reporting]
    // An invariant is that anytime the expected or actual type is Error (the special
    // error type, meaning that an error occurred when typechecking this expression),
    // this is a derived error. The error cascaded from another error (that was already
    // reported), so it's not useful to display it to the user.
    // The following methods implement this logic.
    // They check if either the actual or expected type is Error, and don't print the error
    // in this case. The typechecker should only ever report type errors involving mismatched
    // types using one of these methods, and should not call span_err directly for such
    // errors.
    pub fn type_error_struct_with_diag<M>(
        &self,
        sp: Span,
        mk_diag: M,
        actual_ty: Ty<'tcx>,
    ) -> Diag<'tcx>
    where
        M: FnOnce(String) -> Diag<'tcx>,
    {
        let actual_ty = self.resolve_vars_if_possible(actual_ty);
        debug!("type_error_struct_with_diag({:?}, {:?})", sp, actual_ty);

        let mut err = mk_diag(self.ty_to_string(actual_ty));

        // Don't report an error if actual type is `Error`.
        if actual_ty.references_error() {
            err.downgrade_to_delayed_bug();
        }

        err
    }

    pub fn report_mismatched_types(
        &self,
        cause: &ObligationCause<'tcx>,
        expected: Ty<'tcx>,
        actual: Ty<'tcx>,
        err: TypeError<'tcx>,
    ) -> Diag<'tcx> {
        self.report_and_explain_type_error(TypeTrace::types(cause, true, expected, actual), err)
    }

    pub fn report_mismatched_consts(
        &self,
        cause: &ObligationCause<'tcx>,
        expected: ty::Const<'tcx>,
        actual: ty::Const<'tcx>,
        err: TypeError<'tcx>,
    ) -> Diag<'tcx> {
        self.report_and_explain_type_error(TypeTrace::consts(cause, true, expected, actual), err)
    }
}

/// Helper for [InferCtxt::ty_or_const_infer_var_changed] (see comment on that), currently
/// used only for `traits::fulfill`'s list of `stalled_on` inference variables.
#[derive(Copy, Clone, Debug)]
pub enum TyOrConstInferVar {
    /// Equivalent to `ty::Infer(ty::TyVar(_))`.
    Ty(TyVid),
    /// Equivalent to `ty::Infer(ty::IntVar(_))`.
    TyInt(IntVid),
    /// Equivalent to `ty::Infer(ty::FloatVar(_))`.
    TyFloat(FloatVid),

    /// Equivalent to `ty::ConstKind::Infer(ty::InferConst::Var(_))`.
    Const(ConstVid),
    /// Equivalent to `ty::ConstKind::Infer(ty::InferConst::EffectVar(_))`.
    Effect(EffectVid),
}

impl<'tcx> TyOrConstInferVar {
    /// Tries to extract an inference variable from a type or a constant, returns `None`
    /// for types other than `ty::Infer(_)` (or `InferTy::Fresh*`) and
    /// for constants other than `ty::ConstKind::Infer(_)` (or `InferConst::Fresh`).
    pub fn maybe_from_generic_arg(arg: GenericArg<'tcx>) -> Option<Self> {
        match arg.unpack() {
            GenericArgKind::Type(ty) => Self::maybe_from_ty(ty),
            GenericArgKind::Const(ct) => Self::maybe_from_const(ct),
            GenericArgKind::Lifetime(_) => None,
        }
    }

    /// Tries to extract an inference variable from a type, returns `None`
    /// for types other than `ty::Infer(_)` (or `InferTy::Fresh*`).
    fn maybe_from_ty(ty: Ty<'tcx>) -> Option<Self> {
        match *ty.kind() {
            ty::Infer(ty::TyVar(v)) => Some(TyOrConstInferVar::Ty(v)),
            ty::Infer(ty::IntVar(v)) => Some(TyOrConstInferVar::TyInt(v)),
            ty::Infer(ty::FloatVar(v)) => Some(TyOrConstInferVar::TyFloat(v)),
            _ => None,
        }
    }

    /// Tries to extract an inference variable from a constant, returns `None`
    /// for constants other than `ty::ConstKind::Infer(_)` (or `InferConst::Fresh`).
    fn maybe_from_const(ct: ty::Const<'tcx>) -> Option<Self> {
        match ct.kind() {
            ty::ConstKind::Infer(InferConst::Var(v)) => Some(TyOrConstInferVar::Const(v)),
            ty::ConstKind::Infer(InferConst::EffectVar(v)) => Some(TyOrConstInferVar::Effect(v)),
            _ => None,
        }
    }
}

/// Replace `{integer}` with `i32` and `{float}` with `f64`.
/// Used only for diagnostics.
struct InferenceLiteralEraser<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> TypeFolder<TyCtxt<'tcx>> for InferenceLiteralEraser<'tcx> {
    fn interner(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    fn fold_ty(&mut self, ty: Ty<'tcx>) -> Ty<'tcx> {
        match ty.kind() {
            ty::Infer(ty::IntVar(_) | ty::FreshIntTy(_)) => self.tcx.types.i32,
            ty::Infer(ty::FloatVar(_) | ty::FreshFloatTy(_)) => self.tcx.types.f64,
            _ => ty.super_fold_with(self),
        }
    }
}

struct ShallowResolver<'a, 'tcx> {
    infcx: &'a InferCtxt<'tcx>,
}

impl<'a, 'tcx> TypeFolder<TyCtxt<'tcx>> for ShallowResolver<'a, 'tcx> {
    fn interner(&self) -> TyCtxt<'tcx> {
        self.infcx.tcx
    }

    /// If `ty` is a type variable of some kind, resolve it one level
    /// (but do not resolve types found in the result). If `typ` is
    /// not a type variable, just return it unmodified.
    #[inline]
    fn fold_ty(&mut self, ty: Ty<'tcx>) -> Ty<'tcx> {
        if let ty::Infer(v) = ty.kind() { self.fold_infer_ty(*v).unwrap_or(ty) } else { ty }
    }

    fn fold_const(&mut self, ct: ty::Const<'tcx>) -> ty::Const<'tcx> {
        match ct.kind() {
            ty::ConstKind::Infer(InferConst::Var(vid)) => self
                .infcx
                .inner
                .borrow_mut()
                .const_unification_table()
                .probe_value(vid)
                .known()
                .unwrap_or(ct),
            ty::ConstKind::Infer(InferConst::EffectVar(vid)) => self
                .infcx
                .inner
                .borrow_mut()
                .effect_unification_table()
                .probe_value(vid)
                .known()
                .unwrap_or(ct),
            _ => ct,
        }
    }
}

impl<'a, 'tcx> ShallowResolver<'a, 'tcx> {
    // This is separate from `fold_ty` to keep that method small and inlinable.
    #[inline(never)]
    fn fold_infer_ty(&mut self, v: InferTy) -> Option<Ty<'tcx>> {
        match v {
            ty::TyVar(v) => {
                // Not entirely obvious: if `typ` is a type variable,
                // it can be resolved to an int/float variable, which
                // can then be recursively resolved, hence the
                // recursion. Note though that we prevent type
                // variables from unifying to other type variables
                // directly (though they may be embedded
                // structurally), and we prevent cycles in any case,
                // so this recursion should always be of very limited
                // depth.
                //
                // Note: if these two lines are combined into one we get
                // dynamic borrow errors on `self.inner`.
                let known = self.infcx.inner.borrow_mut().type_variables().probe(v).known();
                known.map(|t| self.fold_ty(t))
            }

            ty::IntVar(v) => self
                .infcx
                .inner
                .borrow_mut()
                .int_unification_table()
                .probe_value(v)
                .map(|v| v.to_type(self.infcx.tcx)),

            ty::FloatVar(v) => self
                .infcx
                .inner
                .borrow_mut()
                .float_unification_table()
                .probe_value(v)
                .map(|v| v.to_type(self.infcx.tcx)),

            ty::FreshTy(_) | ty::FreshIntTy(_) | ty::FreshFloatTy(_) => None,
        }
    }
}

impl<'tcx> TypeTrace<'tcx> {
    pub fn span(&self) -> Span {
        self.cause.span
    }

    pub fn types(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Ty<'tcx>,
        b: Ty<'tcx>,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: Terms(ExpectedFound::new(a_is_expected, a.into(), b.into())),
        }
    }

    pub fn poly_trait_refs(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: ty::PolyTraitRef<'tcx>,
        b: ty::PolyTraitRef<'tcx>,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: PolyTraitRefs(ExpectedFound::new(a_is_expected, a, b)),
        }
    }

    pub fn consts(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: ty::Const<'tcx>,
        b: ty::Const<'tcx>,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: Terms(ExpectedFound::new(a_is_expected, a.into(), b.into())),
        }
    }
}

impl<'tcx> SubregionOrigin<'tcx> {
    pub fn span(&self) -> Span {
        match *self {
            Subtype(ref a) => a.span(),
            RelateObjectBound(a) => a,
            RelateParamBound(a, ..) => a,
            RelateRegionParamBound(a) => a,
            Reborrow(a) => a,
            ReferenceOutlivesReferent(_, a) => a,
            CompareImplItemObligation { span, .. } => span,
            AscribeUserTypeProvePredicate(span) => span,
            CheckAssociatedTypeBounds { ref parent, .. } => parent.span(),
        }
    }

    pub fn from_obligation_cause<F>(cause: &traits::ObligationCause<'tcx>, default: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        match *cause.code() {
            traits::ObligationCauseCode::ReferenceOutlivesReferent(ref_type) => {
                SubregionOrigin::ReferenceOutlivesReferent(ref_type, cause.span)
            }

            traits::ObligationCauseCode::CompareImplItemObligation {
                impl_item_def_id,
                trait_item_def_id,
                kind: _,
            } => SubregionOrigin::CompareImplItemObligation {
                span: cause.span,
                impl_item_def_id,
                trait_item_def_id,
            },

            traits::ObligationCauseCode::CheckAssociatedTypeBounds {
                impl_item_def_id,
                trait_item_def_id,
            } => SubregionOrigin::CheckAssociatedTypeBounds {
                impl_item_def_id,
                trait_item_def_id,
                parent: Box::new(default()),
            },

            traits::ObligationCauseCode::AscribeUserTypeProvePredicate(span) => {
                SubregionOrigin::AscribeUserTypeProvePredicate(span)
            }

            _ => default(),
        }
    }
}

impl RegionVariableOrigin {
    pub fn span(&self) -> Span {
        match *self {
            MiscVariable(a)
            | PatternRegion(a)
            | AddrOfRegion(a)
            | Autoref(a)
            | Coercion(a)
            | RegionParameterDefinition(a, ..)
            | BoundRegion(a, ..)
            | UpvarRegion(_, a) => a,
            Nll(..) => bug!("NLL variable used with `span`"),
        }
    }
}

/// Replaces args that reference param or infer variables with suitable
/// placeholders. This function is meant to remove these param and infer
/// args when they're not actually needed to evaluate a constant.
fn replace_param_and_infer_args_with_placeholder<'tcx>(
    tcx: TyCtxt<'tcx>,
    args: GenericArgsRef<'tcx>,
) -> GenericArgsRef<'tcx> {
    struct ReplaceParamAndInferWithPlaceholder<'tcx> {
        tcx: TyCtxt<'tcx>,
        idx: u32,
    }

    impl<'tcx> TypeFolder<TyCtxt<'tcx>> for ReplaceParamAndInferWithPlaceholder<'tcx> {
        fn interner(&self) -> TyCtxt<'tcx> {
            self.tcx
        }

        fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx> {
            if let ty::Infer(_) = t.kind() {
                let idx = {
                    let idx = self.idx;
                    self.idx += 1;
                    idx
                };
                Ty::new_placeholder(
                    self.tcx,
                    ty::PlaceholderType {
                        universe: ty::UniverseIndex::ROOT,
                        bound: ty::BoundTy {
                            var: ty::BoundVar::from_u32(idx),
                            kind: ty::BoundTyKind::Anon,
                        },
                    },
                )
            } else {
                t.super_fold_with(self)
            }
        }

        fn fold_const(&mut self, c: ty::Const<'tcx>) -> ty::Const<'tcx> {
            if let ty::ConstKind::Infer(_) = c.kind() {
                let ty = c.ty();
                // If the type references param or infer then ICE ICE ICE
                if ty.has_non_region_param() || ty.has_non_region_infer() {
                    bug!("const `{c}`'s type should not reference params or types");
                }
                ty::Const::new_placeholder(
                    self.tcx,
                    ty::PlaceholderConst {
                        universe: ty::UniverseIndex::ROOT,
                        bound: ty::BoundVar::from_u32({
                            let idx = self.idx;
                            self.idx += 1;
                            idx
                        }),
                    },
                    ty,
                )
            } else {
                c.super_fold_with(self)
            }
        }
    }

    args.fold_with(&mut ReplaceParamAndInferWithPlaceholder { tcx, idx: 0 })
}
