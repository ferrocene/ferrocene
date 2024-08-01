//! Greatest lower bound. See [`lattice`].

use rustc_middle::traits::solve::Goal;
use rustc_middle::ty::relate::{Relate, RelateResult, TypeRelation};
use rustc_middle::ty::{self, Ty, TyCtxt, TypeVisitableExt};
use rustc_span::Span;

use super::combine::{CombineFields, PredicateEmittingRelation};
use super::lattice::{self, LatticeDir};
use super::StructurallyRelateAliases;
use crate::infer::{DefineOpaqueTypes, InferCtxt, SubregionOrigin};
use crate::traits::ObligationCause;

/// "Greatest lower bound" (common subtype)
pub struct Glb<'combine, 'infcx, 'tcx> {
    fields: &'combine mut CombineFields<'infcx, 'tcx>,
}

impl<'combine, 'infcx, 'tcx> Glb<'combine, 'infcx, 'tcx> {
    pub fn new(fields: &'combine mut CombineFields<'infcx, 'tcx>) -> Glb<'combine, 'infcx, 'tcx> {
        Glb { fields }
    }
}

impl<'tcx> TypeRelation<TyCtxt<'tcx>> for Glb<'_, '_, 'tcx> {
    fn cx(&self) -> TyCtxt<'tcx> {
        self.fields.tcx()
    }

    fn relate_with_variance<T: Relate<TyCtxt<'tcx>>>(
        &mut self,
        variance: ty::Variance,
        _info: ty::VarianceDiagInfo<TyCtxt<'tcx>>,
        a: T,
        b: T,
    ) -> RelateResult<'tcx, T> {
        match variance {
            ty::Invariant => self.fields.equate(StructurallyRelateAliases::No).relate(a, b),
            ty::Covariant => self.relate(a, b),
            // FIXME(#41044) -- not correct, need test
            ty::Bivariant => Ok(a),
            ty::Contravariant => self.fields.lub().relate(a, b),
        }
    }

    #[instrument(skip(self), level = "trace")]
    fn tys(&mut self, a: Ty<'tcx>, b: Ty<'tcx>) -> RelateResult<'tcx, Ty<'tcx>> {
        lattice::super_lattice_tys(self, a, b)
    }

    #[instrument(skip(self), level = "trace")]
    fn regions(
        &mut self,
        a: ty::Region<'tcx>,
        b: ty::Region<'tcx>,
    ) -> RelateResult<'tcx, ty::Region<'tcx>> {
        let origin = SubregionOrigin::Subtype(Box::new(self.fields.trace.clone()));
        // GLB(&'static u8, &'a u8) == &RegionLUB('static, 'a) u8 == &'static u8
        Ok(self.fields.infcx.inner.borrow_mut().unwrap_region_constraints().lub_regions(
            self.cx(),
            origin,
            a,
            b,
        ))
    }

    #[instrument(skip(self), level = "trace")]
    fn consts(
        &mut self,
        a: ty::Const<'tcx>,
        b: ty::Const<'tcx>,
    ) -> RelateResult<'tcx, ty::Const<'tcx>> {
        self.fields.infcx.super_combine_consts(self, a, b)
    }

    fn binders<T>(
        &mut self,
        a: ty::Binder<'tcx, T>,
        b: ty::Binder<'tcx, T>,
    ) -> RelateResult<'tcx, ty::Binder<'tcx, T>>
    where
        T: Relate<TyCtxt<'tcx>>,
    {
        // GLB of a binder and itself is just itself
        if a == b {
            return Ok(a);
        }

        debug!("binders(a={:?}, b={:?})", a, b);
        if a.skip_binder().has_escaping_bound_vars() || b.skip_binder().has_escaping_bound_vars() {
            // When higher-ranked types are involved, computing the GLB is
            // very challenging, switch to invariance. This is obviously
            // overly conservative but works ok in practice.
            self.relate_with_variance(ty::Invariant, ty::VarianceDiagInfo::default(), a, b)?;
            Ok(a)
        } else {
            Ok(ty::Binder::dummy(self.relate(a.skip_binder(), b.skip_binder())?))
        }
    }
}

impl<'combine, 'infcx, 'tcx> LatticeDir<'infcx, 'tcx> for Glb<'combine, 'infcx, 'tcx> {
    fn infcx(&self) -> &'infcx InferCtxt<'tcx> {
        self.fields.infcx
    }

    fn cause(&self) -> &ObligationCause<'tcx> {
        &self.fields.trace.cause
    }

    fn relate_bound(&mut self, v: Ty<'tcx>, a: Ty<'tcx>, b: Ty<'tcx>) -> RelateResult<'tcx, ()> {
        let mut sub = self.fields.sub();
        sub.relate(v, a)?;
        sub.relate(v, b)?;
        Ok(())
    }

    fn define_opaque_types(&self) -> DefineOpaqueTypes {
        self.fields.define_opaque_types
    }
}

impl<'tcx> PredicateEmittingRelation<InferCtxt<'tcx>> for Glb<'_, '_, 'tcx> {
    fn span(&self) -> Span {
        self.fields.trace.span()
    }

    fn structurally_relate_aliases(&self) -> StructurallyRelateAliases {
        StructurallyRelateAliases::No
    }

    fn param_env(&self) -> ty::ParamEnv<'tcx> {
        self.fields.param_env
    }

    fn register_predicates(
        &mut self,
        obligations: impl IntoIterator<Item: ty::Upcast<TyCtxt<'tcx>, ty::Predicate<'tcx>>>,
    ) {
        self.fields.register_predicates(obligations);
    }

    fn register_goals(
        &mut self,
        obligations: impl IntoIterator<Item = Goal<'tcx, ty::Predicate<'tcx>>>,
    ) {
        self.fields.register_obligations(obligations);
    }

    fn register_alias_relate_predicate(&mut self, a: Ty<'tcx>, b: Ty<'tcx>) {
        self.register_predicates([ty::Binder::dummy(ty::PredicateKind::AliasRelate(
            a.into(),
            b.into(),
            // FIXME(deferred_projection_equality): This isn't right, I think?
            ty::AliasRelationDirection::Equate,
        ))]);
    }
}
