//! There are four type combiners: [TypeRelating], [Lub], and [Glb],
//! and `NllTypeRelating` in rustc_borrowck, which is only used for NLL.
//!
//! Each implements the trait [TypeRelation] and contains methods for
//! combining two instances of various things and yielding a new instance.
//! These combiner methods always yield a `Result<T>`. To relate two
//! types, you can use `infcx.at(cause, param_env)` which then allows
//! you to use the relevant methods of [At](crate::infer::at::At).
//!
//! Combiners mostly do their specific behavior and then hand off the
//! bulk of the work to [InferCtxt::super_combine_tys] and
//! [InferCtxt::super_combine_consts].
//!
//! Combining two types may have side-effects on the inference contexts
//! which can be undone by using snapshots. You probably want to use
//! either [InferCtxt::commit_if_ok] or [InferCtxt::probe].
//!
//! On success, the  LUB/GLB operations return the appropriate bound. The
//! return value of `Equate` or `Sub` shouldn't really be used.

pub use rustc_next_trait_solver::relate::combine::*;

use super::glb::Glb;
use super::lub::Lub;
use super::type_relating::TypeRelating;
use super::RelateResult;
use super::StructurallyRelateAliases;
use crate::infer::relate;
use crate::infer::{DefineOpaqueTypes, InferCtxt, TypeTrace};
use crate::traits::{Obligation, PredicateObligation};
use rustc_middle::bug;
use rustc_middle::infer::unify_key::EffectVarValue;
use rustc_middle::traits::solve::Goal;
use rustc_middle::ty::error::{ExpectedFound, TypeError};
use rustc_middle::ty::{self, InferConst, Ty, TyCtxt, TypeVisitableExt, Upcast};
use rustc_middle::ty::{IntType, UintType};

#[derive(Clone)]
pub struct CombineFields<'infcx, 'tcx> {
    pub infcx: &'infcx InferCtxt<'tcx>,
    pub trace: TypeTrace<'tcx>,
    pub param_env: ty::ParamEnv<'tcx>,
    pub goals: Vec<Goal<'tcx, ty::Predicate<'tcx>>>,
    pub define_opaque_types: DefineOpaqueTypes,
}

impl<'infcx, 'tcx> CombineFields<'infcx, 'tcx> {
    pub fn new(
        infcx: &'infcx InferCtxt<'tcx>,
        trace: TypeTrace<'tcx>,
        param_env: ty::ParamEnv<'tcx>,
        define_opaque_types: DefineOpaqueTypes,
    ) -> Self {
        Self { infcx, trace, param_env, define_opaque_types, goals: vec![] }
    }

    pub(crate) fn into_obligations(self) -> Vec<PredicateObligation<'tcx>> {
        self.goals
            .into_iter()
            .map(|goal| {
                Obligation::new(
                    self.infcx.tcx,
                    self.trace.cause.clone(),
                    goal.param_env,
                    goal.predicate,
                )
            })
            .collect()
    }
}

impl<'tcx> InferCtxt<'tcx> {
    pub fn super_combine_tys<R>(
        &self,
        relation: &mut R,
        a: Ty<'tcx>,
        b: Ty<'tcx>,
    ) -> RelateResult<'tcx, Ty<'tcx>>
    where
        R: PredicateEmittingRelation<InferCtxt<'tcx>>,
    {
        debug!("super_combine_tys::<{}>({:?}, {:?})", std::any::type_name::<R>(), a, b);
        debug_assert!(!a.has_escaping_bound_vars());
        debug_assert!(!b.has_escaping_bound_vars());

        match (a.kind(), b.kind()) {
            // Relate integral variables to other types
            (&ty::Infer(ty::IntVar(a_id)), &ty::Infer(ty::IntVar(b_id))) => {
                self.inner.borrow_mut().int_unification_table().union(a_id, b_id);
                Ok(a)
            }
            (&ty::Infer(ty::IntVar(v_id)), &ty::Int(v)) => {
                self.unify_integral_variable(v_id, IntType(v));
                Ok(b)
            }
            (&ty::Int(v), &ty::Infer(ty::IntVar(v_id))) => {
                self.unify_integral_variable(v_id, IntType(v));
                Ok(a)
            }
            (&ty::Infer(ty::IntVar(v_id)), &ty::Uint(v)) => {
                self.unify_integral_variable(v_id, UintType(v));
                Ok(b)
            }
            (&ty::Uint(v), &ty::Infer(ty::IntVar(v_id))) => {
                self.unify_integral_variable(v_id, UintType(v));
                Ok(a)
            }

            // Relate floating-point variables to other types
            (&ty::Infer(ty::FloatVar(a_id)), &ty::Infer(ty::FloatVar(b_id))) => {
                self.inner.borrow_mut().float_unification_table().union(a_id, b_id);
                Ok(a)
            }
            (&ty::Infer(ty::FloatVar(v_id)), &ty::Float(v)) => {
                self.unify_float_variable(v_id, ty::FloatVarValue::Known(v));
                Ok(b)
            }
            (&ty::Float(v), &ty::Infer(ty::FloatVar(v_id))) => {
                self.unify_float_variable(v_id, ty::FloatVarValue::Known(v));
                Ok(a)
            }

            // We don't expect `TyVar` or `Fresh*` vars at this point with lazy norm.
            (ty::Alias(..), ty::Infer(ty::TyVar(_))) | (ty::Infer(ty::TyVar(_)), ty::Alias(..))
                if self.next_trait_solver() =>
            {
                bug!(
                    "We do not expect to encounter `TyVar` this late in combine \
                    -- they should have been handled earlier"
                )
            }
            (_, ty::Infer(ty::FreshTy(_) | ty::FreshIntTy(_) | ty::FreshFloatTy(_)))
            | (ty::Infer(ty::FreshTy(_) | ty::FreshIntTy(_) | ty::FreshFloatTy(_)), _)
                if self.next_trait_solver() =>
            {
                bug!("We do not expect to encounter `Fresh` variables in the new solver")
            }

            (_, ty::Alias(..)) | (ty::Alias(..), _) if self.next_trait_solver() => {
                match relation.structurally_relate_aliases() {
                    StructurallyRelateAliases::Yes => {
                        relate::structurally_relate_tys(relation, a, b)
                    }
                    StructurallyRelateAliases::No => {
                        relation.register_alias_relate_predicate(a, b);
                        Ok(a)
                    }
                }
            }

            // All other cases of inference are errors
            (&ty::Infer(_), _) | (_, &ty::Infer(_)) => {
                Err(TypeError::Sorts(ExpectedFound::new(true, a, b)))
            }

            // During coherence, opaque types should be treated as *possibly*
            // equal to any other type (except for possibly itself). This is an
            // extremely heavy hammer, but can be relaxed in a fowards-compatible
            // way later.
            (&ty::Alias(ty::Opaque, _), _) | (_, &ty::Alias(ty::Opaque, _)) if self.intercrate => {
                relation.register_predicates([ty::Binder::dummy(ty::PredicateKind::Ambiguous)]);
                Ok(a)
            }

            _ => relate::structurally_relate_tys(relation, a, b),
        }
    }

    pub fn super_combine_consts<R>(
        &self,
        relation: &mut R,
        a: ty::Const<'tcx>,
        b: ty::Const<'tcx>,
    ) -> RelateResult<'tcx, ty::Const<'tcx>>
    where
        R: PredicateEmittingRelation<InferCtxt<'tcx>>,
    {
        debug!("super_combine_consts::<{}>({:?}, {:?})", std::any::type_name::<R>(), a, b);
        debug_assert!(!a.has_escaping_bound_vars());
        debug_assert!(!b.has_escaping_bound_vars());

        if a == b {
            return Ok(a);
        }

        let a = self.shallow_resolve_const(a);
        let b = self.shallow_resolve_const(b);

        match (a.kind(), b.kind()) {
            (
                ty::ConstKind::Infer(InferConst::Var(a_vid)),
                ty::ConstKind::Infer(InferConst::Var(b_vid)),
            ) => {
                self.inner.borrow_mut().const_unification_table().union(a_vid, b_vid);
                Ok(a)
            }

            (
                ty::ConstKind::Infer(InferConst::EffectVar(a_vid)),
                ty::ConstKind::Infer(InferConst::EffectVar(b_vid)),
            ) => {
                self.inner.borrow_mut().effect_unification_table().union(a_vid, b_vid);
                Ok(a)
            }

            // All other cases of inference with other variables are errors.
            (
                ty::ConstKind::Infer(InferConst::Var(_) | InferConst::EffectVar(_)),
                ty::ConstKind::Infer(_),
            )
            | (
                ty::ConstKind::Infer(_),
                ty::ConstKind::Infer(InferConst::Var(_) | InferConst::EffectVar(_)),
            ) => {
                bug!(
                    "tried to combine ConstKind::Infer/ConstKind::Infer(InferConst::Var): {a:?} and {b:?}"
                )
            }

            (ty::ConstKind::Infer(InferConst::Var(vid)), _) => {
                self.instantiate_const_var(relation, true, vid, b)?;
                Ok(b)
            }

            (_, ty::ConstKind::Infer(InferConst::Var(vid))) => {
                self.instantiate_const_var(relation, false, vid, a)?;
                Ok(a)
            }

            (ty::ConstKind::Infer(InferConst::EffectVar(vid)), _) => {
                Ok(self.unify_effect_variable(vid, b))
            }

            (_, ty::ConstKind::Infer(InferConst::EffectVar(vid))) => {
                Ok(self.unify_effect_variable(vid, a))
            }

            (ty::ConstKind::Unevaluated(..), _) | (_, ty::ConstKind::Unevaluated(..))
                if self.tcx.features().generic_const_exprs || self.next_trait_solver() =>
            {
                match relation.structurally_relate_aliases() {
                    StructurallyRelateAliases::No => {
                        relation.register_predicates([if self.next_trait_solver() {
                            ty::PredicateKind::AliasRelate(
                                a.into(),
                                b.into(),
                                ty::AliasRelationDirection::Equate,
                            )
                        } else {
                            ty::PredicateKind::ConstEquate(a, b)
                        }]);

                        Ok(b)
                    }
                    StructurallyRelateAliases::Yes => {
                        relate::structurally_relate_consts(relation, a, b)
                    }
                }
            }
            _ => relate::structurally_relate_consts(relation, a, b),
        }
    }

    #[inline(always)]
    fn unify_integral_variable(&self, vid: ty::IntVid, val: ty::IntVarValue) {
        self.inner.borrow_mut().int_unification_table().union_value(vid, val);
    }

    #[inline(always)]
    fn unify_float_variable(&self, vid: ty::FloatVid, val: ty::FloatVarValue) {
        self.inner.borrow_mut().float_unification_table().union_value(vid, val);
    }

    fn unify_effect_variable(&self, vid: ty::EffectVid, val: ty::Const<'tcx>) -> ty::Const<'tcx> {
        self.inner
            .borrow_mut()
            .effect_unification_table()
            .union_value(vid, EffectVarValue::Known(val));
        val
    }
}

impl<'infcx, 'tcx> CombineFields<'infcx, 'tcx> {
    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.infcx.tcx
    }

    pub fn equate<'a>(
        &'a mut self,
        structurally_relate_aliases: StructurallyRelateAliases,
    ) -> TypeRelating<'a, 'infcx, 'tcx> {
        TypeRelating::new(self, structurally_relate_aliases, ty::Invariant)
    }

    pub fn sub<'a>(&'a mut self) -> TypeRelating<'a, 'infcx, 'tcx> {
        TypeRelating::new(self, StructurallyRelateAliases::No, ty::Covariant)
    }

    pub fn sup<'a>(&'a mut self) -> TypeRelating<'a, 'infcx, 'tcx> {
        TypeRelating::new(self, StructurallyRelateAliases::No, ty::Contravariant)
    }

    pub fn lub<'a>(&'a mut self) -> Lub<'a, 'infcx, 'tcx> {
        Lub::new(self)
    }

    pub fn glb<'a>(&'a mut self) -> Glb<'a, 'infcx, 'tcx> {
        Glb::new(self)
    }

    pub fn register_obligations(
        &mut self,
        obligations: impl IntoIterator<Item = Goal<'tcx, ty::Predicate<'tcx>>>,
    ) {
        self.goals.extend(obligations);
    }

    pub fn register_predicates(
        &mut self,
        obligations: impl IntoIterator<Item: Upcast<TyCtxt<'tcx>, ty::Predicate<'tcx>>>,
    ) {
        self.goals.extend(
            obligations
                .into_iter()
                .map(|to_pred| Goal::new(self.infcx.tcx, self.param_env, to_pred)),
        )
    }
}
