use rustc_ast::TraitObjectSyntax;
use rustc_errors::codes::*;
use rustc_errors::{Diag, EmissionGuarantee, StashKey};
use rustc_hir as hir;
use rustc_hir::def::{DefKind, Res};
use rustc_lint_defs::builtin::BARE_TRAIT_OBJECTS;
use rustc_lint_defs::Applicability;
use rustc_span::Span;
use rustc_trait_selection::error_reporting::traits::suggestions::NextTypeParamName;

use super::HirTyLowerer;

impl<'tcx> dyn HirTyLowerer<'tcx> + '_ {
    /// Prohibit or lint against *bare* trait object types depending on the edition.
    ///
    /// *Bare* trait object types are ones that aren't preceded by the keyword `dyn`.
    /// In edition 2021 and onward we emit a hard error for them.
    pub(super) fn prohibit_or_lint_bare_trait_object_ty(
        &self,
        self_ty: &hir::Ty<'_>,
        in_path: bool,
    ) {
        let tcx = self.tcx();

        let hir::TyKind::TraitObject([poly_trait_ref, ..], _, TraitObjectSyntax::None) =
            self_ty.kind
        else {
            return;
        };

        let needs_bracket = in_path
            && !tcx
                .sess
                .source_map()
                .span_to_prev_source(self_ty.span)
                .ok()
                .is_some_and(|s| s.trim_end().ends_with('<'));

        let is_global = poly_trait_ref.0.trait_ref.path.is_global();

        let mut sugg = vec![(
            self_ty.span.shrink_to_lo(),
            format!(
                "{}dyn {}",
                if needs_bracket { "<" } else { "" },
                if is_global { "(" } else { "" },
            ),
        )];

        if is_global || needs_bracket {
            sugg.push((
                self_ty.span.shrink_to_hi(),
                format!(
                    "{}{}",
                    if is_global { ")" } else { "" },
                    if needs_bracket { ">" } else { "" },
                ),
            ));
        }

        if self_ty.span.edition().at_least_rust_2021() {
            let msg = "trait objects must include the `dyn` keyword";
            let label = "add `dyn` keyword before this trait";
            let mut diag =
                rustc_errors::struct_span_code_err!(self.dcx(), self_ty.span, E0782, "{}", msg);
            if self_ty.span.can_be_used_for_suggestions()
                && !self.maybe_suggest_impl_trait(self_ty, &mut diag)
            {
                // FIXME: Only emit this suggestion if the trait is object safe.
                diag.multipart_suggestion_verbose(label, sugg, Applicability::MachineApplicable);
            }
            // Check if the impl trait that we are considering is an impl of a local trait.
            self.maybe_suggest_blanket_trait_impl(self_ty, &mut diag);
            self.maybe_suggest_assoc_ty_bound(self_ty, &mut diag);
            diag.stash(self_ty.span, StashKey::TraitMissingMethod);
        } else {
            tcx.node_span_lint(BARE_TRAIT_OBJECTS, self_ty.hir_id, self_ty.span, |lint| {
                lint.primary_message("trait objects without an explicit `dyn` are deprecated");
                if self_ty.span.can_be_used_for_suggestions() {
                    lint.multipart_suggestion_verbose(
                        "if this is an object-safe trait, use `dyn`",
                        sugg,
                        Applicability::MachineApplicable,
                    );
                }
                self.maybe_suggest_blanket_trait_impl(self_ty, lint);
            });
        }
    }

    /// Make sure that we are in the condition to suggest the blanket implementation.
    fn maybe_suggest_blanket_trait_impl<G: EmissionGuarantee>(
        &self,
        self_ty: &hir::Ty<'_>,
        diag: &mut Diag<'_, G>,
    ) {
        let tcx = self.tcx();
        let parent_id = tcx.hir().get_parent_item(self_ty.hir_id).def_id;
        if let hir::Node::Item(hir::Item {
            kind:
                hir::ItemKind::Impl(hir::Impl {
                    self_ty: impl_self_ty,
                    of_trait: Some(of_trait_ref),
                    generics,
                    ..
                }),
            ..
        }) = tcx.hir_node_by_def_id(parent_id)
            && self_ty.hir_id == impl_self_ty.hir_id
        {
            if !of_trait_ref.trait_def_id().is_some_and(|def_id| def_id.is_local()) {
                return;
            }
            let of_trait_span = of_trait_ref.path.span;
            // make sure that we are not calling unwrap to abort during the compilation
            let Ok(of_trait_name) = tcx.sess.source_map().span_to_snippet(of_trait_span) else {
                return;
            };

            let Ok(impl_trait_name) = self.tcx().sess.source_map().span_to_snippet(self_ty.span)
            else {
                return;
            };
            let sugg = self.add_generic_param_suggestion(generics, self_ty.span, &impl_trait_name);
            if sugg.is_empty() {
                return;
            };
            diag.multipart_suggestion(
                format!(
                    "alternatively use a blanket implementation to implement `{of_trait_name}` for \
                     all types that also implement `{impl_trait_name}`"
                ),
                sugg,
                Applicability::MaybeIncorrect,
            );
        }
    }

    fn add_generic_param_suggestion(
        &self,
        generics: &hir::Generics<'_>,
        self_ty_span: Span,
        impl_trait_name: &str,
    ) -> Vec<(Span, String)> {
        // check if the trait has generics, to make a correct suggestion
        let param_name = generics.params.next_type_param_name(None);

        let add_generic_sugg = if let Some(span) = generics.span_for_param_suggestion() {
            (span, format!(", {param_name}: {impl_trait_name}"))
        } else {
            (generics.span, format!("<{param_name}: {impl_trait_name}>"))
        };
        vec![(self_ty_span, param_name), add_generic_sugg]
    }

    /// Make sure that we are in the condition to suggest `impl Trait`.
    fn maybe_suggest_impl_trait(&self, self_ty: &hir::Ty<'_>, diag: &mut Diag<'_>) -> bool {
        let tcx = self.tcx();
        let parent_id = tcx.hir().get_parent_item(self_ty.hir_id).def_id;
        // FIXME: If `type_alias_impl_trait` is enabled, also look for `Trait0<Ty = Trait1>`
        //        and suggest `Trait0<Ty = impl Trait1>`.
        let (sig, generics, owner) = match tcx.hir_node_by_def_id(parent_id) {
            hir::Node::Item(hir::Item { kind: hir::ItemKind::Fn(sig, generics, _), .. }) => {
                (sig, generics, None)
            }
            hir::Node::TraitItem(hir::TraitItem {
                kind: hir::TraitItemKind::Fn(sig, _),
                generics,
                owner_id,
                ..
            }) => (sig, generics, Some(tcx.parent(owner_id.to_def_id()))),
            _ => return false,
        };
        let Ok(trait_name) = tcx.sess.source_map().span_to_snippet(self_ty.span) else {
            return false;
        };
        let impl_sugg = vec![(self_ty.span.shrink_to_lo(), "impl ".to_string())];
        let mut is_downgradable = true;
        let is_object_safe = match self_ty.kind {
            hir::TyKind::TraitObject(objects, ..) => {
                objects.iter().all(|(o, _)| match o.trait_ref.path.res {
                    Res::Def(DefKind::Trait, id) => {
                        if Some(id) == owner {
                            // For recursive traits, don't downgrade the error. (#119652)
                            is_downgradable = false;
                        }
                        tcx.is_object_safe(id)
                    }
                    _ => false,
                })
            }
            _ => false,
        };
        if let hir::FnRetTy::Return(ty) = sig.decl.output
            && ty.hir_id == self_ty.hir_id
        {
            let pre = if !is_object_safe {
                format!("`{trait_name}` is not object safe, ")
            } else {
                String::new()
            };
            let msg = format!(
                "{pre}use `impl {trait_name}` to return an opaque type, as long as you return a \
                 single underlying type",
            );
            diag.multipart_suggestion_verbose(msg, impl_sugg, Applicability::MachineApplicable);
            if is_object_safe {
                diag.multipart_suggestion_verbose(
                    "alternatively, you can return an owned trait object",
                    vec![
                        (ty.span.shrink_to_lo(), "Box<dyn ".to_string()),
                        (ty.span.shrink_to_hi(), ">".to_string()),
                    ],
                    Applicability::MachineApplicable,
                );
            } else if is_downgradable {
                // We'll emit the object safety error already, with a structured suggestion.
                diag.downgrade_to_delayed_bug();
            }
            return true;
        }
        for ty in sig.decl.inputs {
            if ty.hir_id != self_ty.hir_id {
                continue;
            }
            let sugg = self.add_generic_param_suggestion(generics, self_ty.span, &trait_name);
            if !sugg.is_empty() {
                diag.multipart_suggestion_verbose(
                    format!("use a new generic type parameter, constrained by `{trait_name}`"),
                    sugg,
                    Applicability::MachineApplicable,
                );
                diag.multipart_suggestion_verbose(
                    "you can also use an opaque type, but users won't be able to specify the type \
                     parameter when calling the `fn`, having to rely exclusively on type inference",
                    impl_sugg,
                    Applicability::MachineApplicable,
                );
            }
            if !is_object_safe {
                diag.note(format!("`{trait_name}` it is not object safe, so it can't be `dyn`"));
                if is_downgradable {
                    // We'll emit the object safety error already, with a structured suggestion.
                    diag.downgrade_to_delayed_bug();
                }
            } else {
                let sugg = if let hir::TyKind::TraitObject([_, _, ..], _, _) = self_ty.kind {
                    // There are more than one trait bound, we need surrounding parentheses.
                    vec![
                        (self_ty.span.shrink_to_lo(), "&(dyn ".to_string()),
                        (self_ty.span.shrink_to_hi(), ")".to_string()),
                    ]
                } else {
                    vec![(self_ty.span.shrink_to_lo(), "&dyn ".to_string())]
                };
                diag.multipart_suggestion_verbose(
                    format!(
                        "alternatively, use a trait object to accept any type that implements \
                         `{trait_name}`, accessing its methods at runtime using dynamic dispatch",
                    ),
                    sugg,
                    Applicability::MachineApplicable,
                );
            }
            return true;
        }
        false
    }

    fn maybe_suggest_assoc_ty_bound(&self, self_ty: &hir::Ty<'_>, diag: &mut Diag<'_>) {
        let mut parents = self.tcx().hir().parent_iter(self_ty.hir_id);

        if let Some((_, hir::Node::AssocItemConstraint(constraint))) = parents.next()
            && let Some(obj_ty) = constraint.ty()
        {
            if let Some((_, hir::Node::TraitRef(..))) = parents.next()
                && let Some((_, hir::Node::Ty(ty))) = parents.next()
                && let hir::TyKind::TraitObject(..) = ty.kind
            {
                // Assoc ty bounds aren't permitted inside trait object types.
                return;
            }

            let lo = if constraint.gen_args.span_ext.is_dummy() {
                constraint.ident.span
            } else {
                constraint.gen_args.span_ext
            };
            let hi = obj_ty.span;

            if !lo.eq_ctxt(hi) {
                return;
            }

            diag.span_suggestion_verbose(
                lo.between(hi),
                "you might have meant to write a bound here",
                ": ",
                Applicability::MaybeIncorrect,
            );
        }
    }
}
