//! A nice interface for working with the infcx. The basic idea is to
//! do `infcx.at(cause, param_env)`, which sets the "cause" of the
//! operation as well as the surrounding parameter environment. Then
//! you can do something like `.sub(a, b)` or `.eq(a, b)` to create a
//! subtype or equality relationship respectively. The first argument
//! is always the "expected" output from the POV of diagnostics.
//!
//! Examples:
//! ```ignore (fragment)
//!     infcx.at(cause, param_env).sub(a, b)
//!     // requires that `a <: b`, with `a` considered the "expected" type
//!
//!     infcx.at(cause, param_env).sup(a, b)
//!     // requires that `b <: a`, with `a` considered the "expected" type
//!
//!     infcx.at(cause, param_env).eq(a, b)
//!     // requires that `a == b`, with `a` considered the "expected" type
//! ```
//! For finer-grained control, you can also do use `trace`:
//! ```ignore (fragment)
//!     infcx.at(...).trace(a, b).sub(&c, &d)
//! ```
//! This will set `a` and `b` as the "root" values for
//! error-reporting, but actually operate on `c` and `d`. This is
//! sometimes useful when the types of `c` and `d` are not traceable
//! things. (That system should probably be refactored.)

use super::*;

use crate::infer::relate::{Relate, StructurallyRelateAliases, TypeRelation};
use rustc_middle::bug;
use rustc_middle::ty::{Const, ImplSubject};

/// Whether we should define opaque types or just treat them opaquely.
///
/// Currently only used to prevent predicate matching from matching anything
/// against opaque types.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DefineOpaqueTypes {
    Yes,
    No,
}

#[derive(Clone, Copy)]
pub struct At<'a, 'tcx> {
    pub infcx: &'a InferCtxt<'tcx>,
    pub cause: &'a ObligationCause<'tcx>,
    pub param_env: ty::ParamEnv<'tcx>,
}

impl<'tcx> InferCtxt<'tcx> {
    #[inline]
    pub fn at<'a>(
        &'a self,
        cause: &'a ObligationCause<'tcx>,
        param_env: ty::ParamEnv<'tcx>,
    ) -> At<'a, 'tcx> {
        At { infcx: self, cause, param_env }
    }

    /// Forks the inference context, creating a new inference context with the same inference
    /// variables in the same state. This can be used to "branch off" many tests from the same
    /// common state.
    pub fn fork(&self) -> Self {
        self.fork_with_intercrate(self.intercrate)
    }

    /// Forks the inference context, creating a new inference context with the same inference
    /// variables in the same state, except possibly changing the intercrate mode. This can be
    /// used to "branch off" many tests from the same common state. Used in negative coherence.
    pub fn fork_with_intercrate(&self, intercrate: bool) -> Self {
        Self {
            tcx: self.tcx,
            defining_opaque_types: self.defining_opaque_types,
            considering_regions: self.considering_regions,
            skip_leak_check: self.skip_leak_check,
            inner: self.inner.clone(),
            lexical_region_resolutions: self.lexical_region_resolutions.clone(),
            selection_cache: self.selection_cache.clone(),
            evaluation_cache: self.evaluation_cache.clone(),
            reported_trait_errors: self.reported_trait_errors.clone(),
            reported_signature_mismatch: self.reported_signature_mismatch.clone(),
            tainted_by_errors: self.tainted_by_errors.clone(),
            err_count_on_creation: self.err_count_on_creation,
            universe: self.universe.clone(),
            intercrate,
            next_trait_solver: self.next_trait_solver,
            obligation_inspector: self.obligation_inspector.clone(),
        }
    }
}

pub trait ToTrace<'tcx>: Relate<TyCtxt<'tcx>> + Copy {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx>;
}

impl<'a, 'tcx> At<'a, 'tcx> {
    /// Makes `actual <: expected`. For example, if type-checking a
    /// call like `foo(x)`, where `foo: fn(i32)`, you might have
    /// `sup(i32, x)`, since the "expected" type is the type that
    /// appears in the signature.
    pub fn sup<T>(
        self,
        define_opaque_types: DefineOpaqueTypes,
        expected: T,
        actual: T,
    ) -> InferResult<'tcx, ()>
    where
        T: ToTrace<'tcx>,
    {
        let mut fields = CombineFields::new(
            self.infcx,
            ToTrace::to_trace(self.cause, true, expected, actual),
            self.param_env,
            define_opaque_types,
        );
        fields
            .sup()
            .relate(expected, actual)
            .map(|_| InferOk { value: (), obligations: fields.obligations })
    }

    /// Makes `expected <: actual`.
    pub fn sub<T>(
        self,
        define_opaque_types: DefineOpaqueTypes,
        expected: T,
        actual: T,
    ) -> InferResult<'tcx, ()>
    where
        T: ToTrace<'tcx>,
    {
        let mut fields = CombineFields::new(
            self.infcx,
            ToTrace::to_trace(self.cause, true, expected, actual),
            self.param_env,
            define_opaque_types,
        );
        fields
            .sub()
            .relate(expected, actual)
            .map(|_| InferOk { value: (), obligations: fields.obligations })
    }

    /// Makes `expected == actual`.
    pub fn eq<T>(
        self,
        define_opaque_types: DefineOpaqueTypes,
        expected: T,
        actual: T,
    ) -> InferResult<'tcx, ()>
    where
        T: ToTrace<'tcx>,
    {
        let mut fields = CombineFields::new(
            self.infcx,
            ToTrace::to_trace(self.cause, true, expected, actual),
            self.param_env,
            define_opaque_types,
        );
        fields
            .equate(StructurallyRelateAliases::No)
            .relate(expected, actual)
            .map(|_| InferOk { value: (), obligations: fields.obligations })
    }

    /// Equates `expected` and `found` while structurally relating aliases.
    /// This should only be used inside of the next generation trait solver
    /// when relating rigid aliases.
    pub fn eq_structurally_relating_aliases<T>(
        self,
        expected: T,
        actual: T,
    ) -> InferResult<'tcx, ()>
    where
        T: ToTrace<'tcx>,
    {
        assert!(self.infcx.next_trait_solver());
        let mut fields = CombineFields::new(
            self.infcx,
            ToTrace::to_trace(self.cause, true, expected, actual),
            self.param_env,
            DefineOpaqueTypes::Yes,
        );
        fields
            .equate(StructurallyRelateAliases::Yes)
            .relate(expected, actual)
            .map(|_| InferOk { value: (), obligations: fields.obligations })
    }

    pub fn relate<T>(
        self,
        define_opaque_types: DefineOpaqueTypes,
        expected: T,
        variance: ty::Variance,
        actual: T,
    ) -> InferResult<'tcx, ()>
    where
        T: ToTrace<'tcx>,
    {
        match variance {
            ty::Variance::Covariant => self.sub(define_opaque_types, expected, actual),
            ty::Variance::Invariant => self.eq(define_opaque_types, expected, actual),
            ty::Variance::Contravariant => self.sup(define_opaque_types, expected, actual),

            // We could make this make sense but it's not readily
            // exposed and I don't feel like dealing with it. Note
            // that bivariance in general does a bit more than just
            // *nothing*, it checks that the types are the same
            // "modulo variance" basically.
            ty::Variance::Bivariant => panic!("Bivariant given to `relate()`"),
        }
    }

    /// Computes the least-upper-bound, or mutual supertype, of two
    /// values. The order of the arguments doesn't matter, but since
    /// this can result in an error (e.g., if asked to compute LUB of
    /// u32 and i32), it is meaningful to call one of them the
    /// "expected type".
    pub fn lub<T>(
        self,
        define_opaque_types: DefineOpaqueTypes,
        expected: T,
        actual: T,
    ) -> InferResult<'tcx, T>
    where
        T: ToTrace<'tcx>,
    {
        let mut fields = CombineFields::new(
            self.infcx,
            ToTrace::to_trace(self.cause, true, expected, actual),
            self.param_env,
            define_opaque_types,
        );
        fields
            .lub()
            .relate(expected, actual)
            .map(|value| InferOk { value, obligations: fields.obligations })
    }

    /// Computes the greatest-lower-bound, or mutual subtype, of two
    /// values. As with `lub` order doesn't matter, except for error
    /// cases.
    pub fn glb<T>(
        self,
        define_opaque_types: DefineOpaqueTypes,
        expected: T,
        actual: T,
    ) -> InferResult<'tcx, T>
    where
        T: ToTrace<'tcx>,
    {
        let mut fields = CombineFields::new(
            self.infcx,
            ToTrace::to_trace(self.cause, true, expected, actual),
            self.param_env,
            define_opaque_types,
        );
        fields
            .glb()
            .relate(expected, actual)
            .map(|value| InferOk { value, obligations: fields.obligations })
    }
}

impl<'tcx> ToTrace<'tcx> for ImplSubject<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        match (a, b) {
            (ImplSubject::Trait(trait_ref_a), ImplSubject::Trait(trait_ref_b)) => {
                ToTrace::to_trace(cause, a_is_expected, trait_ref_a, trait_ref_b)
            }
            (ImplSubject::Inherent(ty_a), ImplSubject::Inherent(ty_b)) => {
                ToTrace::to_trace(cause, a_is_expected, ty_a, ty_b)
            }
            (ImplSubject::Trait(_), ImplSubject::Inherent(_))
            | (ImplSubject::Inherent(_), ImplSubject::Trait(_)) => {
                bug!("can not trace TraitRef and Ty");
            }
        }
    }
}

impl<'tcx> ToTrace<'tcx> for Ty<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: Terms(ExpectedFound::new(a_is_expected, a.into(), b.into())),
        }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::Region<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace { cause: cause.clone(), values: Regions(ExpectedFound::new(a_is_expected, a, b)) }
    }
}

impl<'tcx> ToTrace<'tcx> for Const<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: Terms(ExpectedFound::new(a_is_expected, a.into(), b.into())),
        }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::GenericArg<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: match (a.unpack(), b.unpack()) {
                (GenericArgKind::Lifetime(a), GenericArgKind::Lifetime(b)) => {
                    Regions(ExpectedFound::new(a_is_expected, a, b))
                }
                (GenericArgKind::Type(a), GenericArgKind::Type(b)) => {
                    Terms(ExpectedFound::new(a_is_expected, a.into(), b.into()))
                }
                (GenericArgKind::Const(a), GenericArgKind::Const(b)) => {
                    Terms(ExpectedFound::new(a_is_expected, a.into(), b.into()))
                }

                (
                    GenericArgKind::Lifetime(_),
                    GenericArgKind::Type(_) | GenericArgKind::Const(_),
                )
                | (
                    GenericArgKind::Type(_),
                    GenericArgKind::Lifetime(_) | GenericArgKind::Const(_),
                )
                | (
                    GenericArgKind::Const(_),
                    GenericArgKind::Lifetime(_) | GenericArgKind::Type(_),
                ) => {
                    bug!("relating different kinds: {a:?} {b:?}")
                }
            },
        }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::Term<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace { cause: cause.clone(), values: Terms(ExpectedFound::new(a_is_expected, a, b)) }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::TraitRef<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: TraitRefs(ExpectedFound::new(a_is_expected, a, b)),
        }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::AliasTy<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: Aliases(ExpectedFound::new(a_is_expected, a.into(), b.into())),
        }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::AliasTerm<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace { cause: cause.clone(), values: Aliases(ExpectedFound::new(a_is_expected, a, b)) }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::FnSig<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: PolySigs(ExpectedFound::new(
                a_is_expected,
                ty::Binder::dummy(a),
                ty::Binder::dummy(b),
            )),
        }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::PolyFnSig<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: PolySigs(ExpectedFound::new(a_is_expected, a, b)),
        }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::PolyExistentialTraitRef<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: ExistentialTraitRef(ExpectedFound::new(a_is_expected, a, b)),
        }
    }
}

impl<'tcx> ToTrace<'tcx> for ty::PolyExistentialProjection<'tcx> {
    fn to_trace(
        cause: &ObligationCause<'tcx>,
        a_is_expected: bool,
        a: Self,
        b: Self,
    ) -> TypeTrace<'tcx> {
        TypeTrace {
            cause: cause.clone(),
            values: ExistentialProjection(ExpectedFound::new(a_is_expected, a, b)),
        }
    }
}
