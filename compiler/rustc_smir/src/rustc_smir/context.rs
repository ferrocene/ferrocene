//! Implementation of `[stable_mir::compiler_interface::Context]` trait.
//!
//! This trait is currently the main interface between the Rust compiler,
//! and the `stable_mir` crate.

#![allow(rustc::usage_of_qualified_ty)]

use rustc_abi::HasDataLayout;
use rustc_middle::ty;
use rustc_middle::ty::layout::{
    FnAbiOf, FnAbiOfHelpers, HasParamEnv, HasTyCtxt, LayoutOf, LayoutOfHelpers,
};
use rustc_middle::ty::print::{with_forced_trimmed_paths, with_no_trimmed_paths};
use rustc_middle::ty::{
    GenericPredicates, Instance, List, ParamEnv, ScalarInt, TyCtxt, TypeVisitableExt, ValTree,
};
use rustc_span::def_id::LOCAL_CRATE;
use stable_mir::abi::{FnAbi, Layout, LayoutShape};
use stable_mir::compiler_interface::Context;
use stable_mir::mir::alloc::GlobalAlloc;
use stable_mir::mir::mono::{InstanceDef, StaticDef};
use stable_mir::mir::Body;
use stable_mir::target::{MachineInfo, MachineSize};
use stable_mir::ty::{
    AdtDef, AdtKind, Allocation, ClosureDef, ClosureKind, Const, FieldDef, FnDef, ForeignDef,
    ForeignItemKind, GenericArgs, LineInfo, PolyFnSig, RigidTy, Span, Ty, TyKind, UintTy,
    VariantDef,
};
use stable_mir::{Crate, CrateDef, CrateItem, CrateNum, DefId, Error, Filename, ItemKind, Symbol};
use std::cell::RefCell;
use std::iter;

use crate::rustc_internal::RustcInternal;
use crate::rustc_smir::builder::BodyBuilder;
use crate::rustc_smir::{alloc, new_item_kind, smir_crate, Stable, Tables};

impl<'tcx> Context for TablesWrapper<'tcx> {
    fn target_info(&self) -> MachineInfo {
        let mut tables = self.0.borrow_mut();
        MachineInfo {
            endian: tables.tcx.data_layout.endian.stable(&mut *tables),
            pointer_width: MachineSize::from_bits(
                tables.tcx.data_layout.pointer_size.bits().try_into().unwrap(),
            ),
        }
    }

    fn entry_fn(&self) -> Option<stable_mir::CrateItem> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        Some(tables.crate_item(tcx.entry_fn(())?.0))
    }

    fn all_local_items(&self) -> stable_mir::CrateItems {
        let mut tables = self.0.borrow_mut();
        tables.tcx.mir_keys(()).iter().map(|item| tables.crate_item(item.to_def_id())).collect()
    }

    fn mir_body(&self, item: stable_mir::DefId) -> stable_mir::mir::Body {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[item];
        tables.tcx.instance_mir(rustc_middle::ty::InstanceDef::Item(def_id)).stable(&mut tables)
    }

    fn has_body(&self, def: DefId) -> bool {
        let tables = self.0.borrow();
        let def_id = tables[def];
        tables.tcx.is_mir_available(def_id)
    }

    fn foreign_modules(&self, crate_num: CrateNum) -> Vec<stable_mir::ty::ForeignModuleDef> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        tcx.foreign_modules(crate_num.internal(&mut *tables, tcx))
            .keys()
            .map(|mod_def_id| tables.foreign_module_def(*mod_def_id))
            .collect()
    }

    fn foreign_module(
        &self,
        mod_def: stable_mir::ty::ForeignModuleDef,
    ) -> stable_mir::ty::ForeignModule {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[mod_def.def_id()];
        let mod_def = tables.tcx.foreign_modules(def_id.krate).get(&def_id).unwrap();
        mod_def.stable(&mut *tables)
    }

    fn foreign_items(&self, mod_def: stable_mir::ty::ForeignModuleDef) -> Vec<ForeignDef> {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[mod_def.def_id()];
        tables
            .tcx
            .foreign_modules(def_id.krate)
            .get(&def_id)
            .unwrap()
            .foreign_items
            .iter()
            .map(|item_def| tables.foreign_def(*item_def))
            .collect()
    }

    fn all_trait_decls(&self) -> stable_mir::TraitDecls {
        let mut tables = self.0.borrow_mut();
        tables.tcx.all_traits().map(|trait_def_id| tables.trait_def(trait_def_id)).collect()
    }

    fn trait_decls(&self, crate_num: CrateNum) -> stable_mir::TraitDecls {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        tcx.traits(crate_num.internal(&mut *tables, tcx))
            .iter()
            .map(|trait_def_id| tables.trait_def(*trait_def_id))
            .collect()
    }

    fn trait_decl(&self, trait_def: &stable_mir::ty::TraitDef) -> stable_mir::ty::TraitDecl {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[trait_def.0];
        let trait_def = tables.tcx.trait_def(def_id);
        trait_def.stable(&mut *tables)
    }

    fn all_trait_impls(&self) -> stable_mir::ImplTraitDecls {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        iter::once(LOCAL_CRATE)
            .chain(tables.tcx.crates(()).iter().copied())
            .flat_map(|cnum| tcx.trait_impls_in_crate(cnum).iter())
            .map(|impl_def_id| tables.impl_def(*impl_def_id))
            .collect()
    }

    fn trait_impls(&self, crate_num: CrateNum) -> stable_mir::ImplTraitDecls {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        tcx.trait_impls_in_crate(crate_num.internal(&mut *tables, tcx))
            .iter()
            .map(|impl_def_id| tables.impl_def(*impl_def_id))
            .collect()
    }

    fn trait_impl(&self, impl_def: &stable_mir::ty::ImplDef) -> stable_mir::ty::ImplTrait {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[impl_def.0];
        let impl_trait = tables.tcx.impl_trait_ref(def_id).unwrap();
        impl_trait.stable(&mut *tables)
    }

    fn generics_of(&self, def_id: stable_mir::DefId) -> stable_mir::ty::Generics {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[def_id];
        let generics = tables.tcx.generics_of(def_id);
        generics.stable(&mut *tables)
    }

    fn predicates_of(&self, def_id: stable_mir::DefId) -> stable_mir::ty::GenericPredicates {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[def_id];
        let GenericPredicates { parent, predicates } = tables.tcx.predicates_of(def_id);
        stable_mir::ty::GenericPredicates {
            parent: parent.map(|did| tables.trait_def(did)),
            predicates: predicates
                .iter()
                .map(|(clause, span)| {
                    (
                        clause.as_predicate().kind().skip_binder().stable(&mut *tables),
                        span.stable(&mut *tables),
                    )
                })
                .collect(),
        }
    }

    fn explicit_predicates_of(
        &self,
        def_id: stable_mir::DefId,
    ) -> stable_mir::ty::GenericPredicates {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[def_id];
        let GenericPredicates { parent, predicates } = tables.tcx.explicit_predicates_of(def_id);
        stable_mir::ty::GenericPredicates {
            parent: parent.map(|did| tables.trait_def(did)),
            predicates: predicates
                .iter()
                .map(|(clause, span)| {
                    (
                        clause.as_predicate().kind().skip_binder().stable(&mut *tables),
                        span.stable(&mut *tables),
                    )
                })
                .collect(),
        }
    }

    fn local_crate(&self) -> stable_mir::Crate {
        let tables = self.0.borrow();
        smir_crate(tables.tcx, LOCAL_CRATE)
    }

    fn external_crates(&self) -> Vec<stable_mir::Crate> {
        let tables = self.0.borrow();
        tables.tcx.crates(()).iter().map(|crate_num| smir_crate(tables.tcx, *crate_num)).collect()
    }

    fn find_crates(&self, name: &str) -> Vec<stable_mir::Crate> {
        let tables = self.0.borrow();
        let crates: Vec<stable_mir::Crate> = [LOCAL_CRATE]
            .iter()
            .chain(tables.tcx.crates(()).iter())
            .filter_map(|crate_num| {
                let crate_name = tables.tcx.crate_name(*crate_num).to_string();
                (name == crate_name).then(|| smir_crate(tables.tcx, *crate_num))
            })
            .collect();
        crates
    }

    fn def_name(&self, def_id: stable_mir::DefId, trimmed: bool) -> Symbol {
        let tables = self.0.borrow();
        if trimmed {
            with_forced_trimmed_paths!(tables.tcx.def_path_str(tables[def_id]))
        } else {
            with_no_trimmed_paths!(tables.tcx.def_path_str(tables[def_id]))
        }
    }

    fn span_to_string(&self, span: stable_mir::ty::Span) -> String {
        let tables = self.0.borrow();
        tables.tcx.sess.source_map().span_to_diagnostic_string(tables[span])
    }

    fn get_filename(&self, span: &Span) -> Filename {
        let tables = self.0.borrow();
        tables
            .tcx
            .sess
            .source_map()
            .span_to_filename(tables[*span])
            .display(rustc_span::FileNameDisplayPreference::Local)
            .to_string()
    }

    fn get_lines(&self, span: &Span) -> LineInfo {
        let tables = self.0.borrow();
        let lines = &tables.tcx.sess.source_map().span_to_location_info(tables[*span]);
        LineInfo { start_line: lines.1, start_col: lines.2, end_line: lines.3, end_col: lines.4 }
    }

    fn item_kind(&self, item: CrateItem) -> ItemKind {
        let tables = self.0.borrow();
        new_item_kind(tables.tcx.def_kind(tables[item.0]))
    }

    fn is_foreign_item(&self, item: DefId) -> bool {
        let tables = self.0.borrow();
        tables.tcx.is_foreign_item(tables[item])
    }

    fn foreign_item_kind(&self, def: ForeignDef) -> ForeignItemKind {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[def.def_id()];
        let tcx = tables.tcx;
        use rustc_hir::def::DefKind;
        match tcx.def_kind(def_id) {
            DefKind::Fn => ForeignItemKind::Fn(tables.fn_def(def_id)),
            DefKind::Static { .. } => ForeignItemKind::Static(tables.static_def(def_id)),
            DefKind::ForeignTy => ForeignItemKind::Type(
                tables.intern_ty(rustc_middle::ty::Ty::new_foreign(tcx, def_id)),
            ),
            def_kind => unreachable!("Unexpected kind for a foreign item: {:?}", def_kind),
        }
    }

    fn adt_kind(&self, def: AdtDef) -> AdtKind {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        def.internal(&mut *tables, tcx).adt_kind().stable(&mut *tables)
    }

    fn adt_is_box(&self, def: AdtDef) -> bool {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        def.internal(&mut *tables, tcx).is_box()
    }

    fn adt_is_simd(&self, def: AdtDef) -> bool {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        def.internal(&mut *tables, tcx).repr().simd()
    }

    fn adt_is_cstr(&self, def: AdtDef) -> bool {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let def_id = def.0.internal(&mut *tables, tcx);
        tables.tcx.lang_items().c_str() == Some(def_id)
    }

    fn fn_sig(&self, def: FnDef, args: &GenericArgs) -> PolyFnSig {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let def_id = def.0.internal(&mut *tables, tcx);
        let sig =
            tables.tcx.fn_sig(def_id).instantiate(tables.tcx, args.internal(&mut *tables, tcx));
        sig.stable(&mut *tables)
    }

    fn closure_sig(&self, args: &GenericArgs) -> PolyFnSig {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let args_ref = args.internal(&mut *tables, tcx);
        let sig = args_ref.as_closure().sig();
        sig.stable(&mut *tables)
    }

    fn adt_variants_len(&self, def: AdtDef) -> usize {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        def.internal(&mut *tables, tcx).variants().len()
    }

    fn variant_name(&self, def: VariantDef) -> Symbol {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        def.internal(&mut *tables, tcx).name.to_string()
    }

    fn variant_fields(&self, def: VariantDef) -> Vec<FieldDef> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        def.internal(&mut *tables, tcx).fields.iter().map(|f| f.stable(&mut *tables)).collect()
    }

    fn eval_target_usize(&self, cnst: &Const) -> Result<u64, Error> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let mir_const = cnst.internal(&mut *tables, tcx);
        mir_const
            .try_eval_target_usize(tables.tcx, ParamEnv::empty())
            .ok_or_else(|| Error::new(format!("Const `{cnst:?}` cannot be encoded as u64")))
    }

    fn try_new_const_zst(&self, ty: Ty) -> Result<Const, Error> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let ty_internal = ty.internal(&mut *tables, tcx);
        let size = tables
            .tcx
            .layout_of(ParamEnv::empty().and(ty_internal))
            .map_err(|err| {
                Error::new(format!(
                    "Cannot create a zero-sized constant for type `{ty_internal}`: {err}"
                ))
            })?
            .size;
        if size.bytes() != 0 {
            return Err(Error::new(format!(
                "Cannot create a zero-sized constant for type `{ty_internal}`: \
                 Type `{ty_internal}` has {} bytes",
                size.bytes()
            )));
        }

        Ok(ty::Const::zero_sized(tables.tcx, ty_internal).stable(&mut *tables))
    }

    fn new_const_str(&self, value: &str) -> Const {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let ty = ty::Ty::new_static_str(tcx);
        let bytes = value.as_bytes();
        let val_tree = ty::ValTree::from_raw_bytes(tcx, bytes);

        ty::Const::new_value(tcx, val_tree, ty).stable(&mut *tables)
    }

    fn new_const_bool(&self, value: bool) -> Const {
        let mut tables = self.0.borrow_mut();
        ty::Const::from_bool(tables.tcx, value).stable(&mut *tables)
    }

    fn try_new_const_uint(&self, value: u128, uint_ty: UintTy) -> Result<Const, Error> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let ty = ty::Ty::new_uint(tcx, uint_ty.internal(&mut *tables, tcx));
        let size = tables.tcx.layout_of(ParamEnv::empty().and(ty)).unwrap().size;

        // We don't use Const::from_bits since it doesn't have any error checking.
        let scalar = ScalarInt::try_from_uint(value, size).ok_or_else(|| {
            Error::new(format!("Value overflow: cannot convert `{value}` to `{ty}`."))
        })?;
        Ok(ty::Const::new_value(tables.tcx, ValTree::from_scalar_int(scalar), ty)
            .stable(&mut *tables))
    }

    fn new_rigid_ty(&self, kind: RigidTy) -> stable_mir::ty::Ty {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let internal_kind = kind.internal(&mut *tables, tcx);
        tables.tcx.mk_ty_from_kind(internal_kind).stable(&mut *tables)
    }

    fn new_box_ty(&self, ty: stable_mir::ty::Ty) -> stable_mir::ty::Ty {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let inner = ty.internal(&mut *tables, tcx);
        ty::Ty::new_box(tables.tcx, inner).stable(&mut *tables)
    }

    fn def_ty(&self, item: stable_mir::DefId) -> stable_mir::ty::Ty {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        tcx.type_of(item.internal(&mut *tables, tcx)).instantiate_identity().stable(&mut *tables)
    }

    fn def_ty_with_args(&self, item: stable_mir::DefId, args: &GenericArgs) -> stable_mir::ty::Ty {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let args = args.internal(&mut *tables, tcx);
        let def_ty = tables.tcx.type_of(item.internal(&mut *tables, tcx));
        def_ty.instantiate(tables.tcx, args).stable(&mut *tables)
    }

    fn const_literal(&self, cnst: &stable_mir::ty::Const) -> String {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        cnst.internal(&mut *tables, tcx).to_string()
    }

    fn span_of_an_item(&self, def_id: stable_mir::DefId) -> Span {
        let mut tables = self.0.borrow_mut();
        tables.tcx.def_span(tables[def_id]).stable(&mut *tables)
    }

    fn ty_kind(&self, ty: stable_mir::ty::Ty) -> TyKind {
        let mut tables = self.0.borrow_mut();
        tables.types[ty].kind().stable(&mut *tables)
    }

    fn rigid_ty_discriminant_ty(&self, ty: &RigidTy) -> stable_mir::ty::Ty {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let internal_kind = ty.internal(&mut *tables, tcx);
        let internal_ty = tables.tcx.mk_ty_from_kind(internal_kind);
        internal_ty.discriminant_ty(tables.tcx).stable(&mut *tables)
    }

    fn instance_body(&self, def: InstanceDef) -> Option<Body> {
        let mut tables = self.0.borrow_mut();
        let instance = tables.instances[def];
        tables
            .has_body(instance)
            .then(|| BodyBuilder::new(tables.tcx, instance).build(&mut *tables))
    }

    fn instance_ty(&self, def: InstanceDef) -> stable_mir::ty::Ty {
        let mut tables = self.0.borrow_mut();
        let instance = tables.instances[def];
        assert!(!instance.has_non_region_param(), "{instance:?} needs further instantiation");
        instance.ty(tables.tcx, ParamEnv::reveal_all()).stable(&mut *tables)
    }

    fn instance_args(&self, def: InstanceDef) -> GenericArgs {
        let mut tables = self.0.borrow_mut();
        let instance = tables.instances[def];
        instance.args.stable(&mut *tables)
    }

    fn instance_abi(&self, def: InstanceDef) -> Result<FnAbi, Error> {
        let mut tables = self.0.borrow_mut();
        let instance = tables.instances[def];
        Ok(tables.fn_abi_of_instance(instance, List::empty())?.stable(&mut *tables))
    }

    fn instance_def_id(&self, def: InstanceDef) -> stable_mir::DefId {
        let mut tables = self.0.borrow_mut();
        let def_id = tables.instances[def].def_id();
        tables.create_def_id(def_id)
    }

    fn instance_mangled_name(&self, instance: InstanceDef) -> Symbol {
        let tables = self.0.borrow_mut();
        let instance = tables.instances[instance];
        tables.tcx.symbol_name(instance).name.to_string()
    }

    fn is_empty_drop_shim(&self, def: InstanceDef) -> bool {
        let tables = self.0.borrow_mut();
        let instance = tables.instances[def];
        matches!(instance.def, ty::InstanceDef::DropGlue(_, None))
    }

    fn mono_instance(&self, def_id: stable_mir::DefId) -> stable_mir::mir::mono::Instance {
        let mut tables = self.0.borrow_mut();
        let def_id = tables[def_id];
        Instance::mono(tables.tcx, def_id).stable(&mut *tables)
    }

    fn requires_monomorphization(&self, def_id: stable_mir::DefId) -> bool {
        let tables = self.0.borrow();
        let def_id = tables[def_id];
        let generics = tables.tcx.generics_of(def_id);
        let result = generics.requires_monomorphization(tables.tcx);
        result
    }

    fn resolve_instance(
        &self,
        def: stable_mir::ty::FnDef,
        args: &stable_mir::ty::GenericArgs,
    ) -> Option<stable_mir::mir::mono::Instance> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let def_id = def.0.internal(&mut *tables, tcx);
        let args_ref = args.internal(&mut *tables, tcx);
        match Instance::resolve(tables.tcx, ParamEnv::reveal_all(), def_id, args_ref) {
            Ok(Some(instance)) => Some(instance.stable(&mut *tables)),
            Ok(None) | Err(_) => None,
        }
    }

    fn resolve_drop_in_place(&self, ty: stable_mir::ty::Ty) -> stable_mir::mir::mono::Instance {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let internal_ty = ty.internal(&mut *tables, tcx);
        let instance = Instance::resolve_drop_in_place(tables.tcx, internal_ty);
        instance.stable(&mut *tables)
    }

    fn resolve_for_fn_ptr(
        &self,
        def: FnDef,
        args: &GenericArgs,
    ) -> Option<stable_mir::mir::mono::Instance> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let def_id = def.0.internal(&mut *tables, tcx);
        let args_ref = args.internal(&mut *tables, tcx);
        Instance::resolve_for_fn_ptr(tables.tcx, ParamEnv::reveal_all(), def_id, args_ref)
            .stable(&mut *tables)
    }

    fn resolve_closure(
        &self,
        def: ClosureDef,
        args: &GenericArgs,
        kind: ClosureKind,
    ) -> Option<stable_mir::mir::mono::Instance> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let def_id = def.0.internal(&mut *tables, tcx);
        let args_ref = args.internal(&mut *tables, tcx);
        let closure_kind = kind.internal(&mut *tables, tcx);
        Some(
            Instance::resolve_closure(tables.tcx, def_id, args_ref, closure_kind)
                .stable(&mut *tables),
        )
    }

    fn eval_instance(&self, def: InstanceDef, const_ty: Ty) -> Result<Allocation, Error> {
        let mut tables = self.0.borrow_mut();
        let instance = tables.instances[def];
        let tcx = tables.tcx;
        let result = tcx.const_eval_instance(
            ParamEnv::reveal_all(),
            instance,
            Some(tcx.def_span(instance.def_id())),
        );
        result
            .map(|const_val| {
                alloc::try_new_allocation(
                    const_ty.internal(&mut *tables, tcx),
                    const_val,
                    &mut *tables,
                )
            })
            .map_err(|e| e.stable(&mut *tables))?
    }

    fn eval_static_initializer(&self, def: StaticDef) -> Result<Allocation, Error> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let def_id = def.0.internal(&mut *tables, tcx);
        tables.tcx.eval_static_initializer(def_id).stable(&mut *tables)
    }

    fn global_alloc(&self, alloc: stable_mir::mir::alloc::AllocId) -> GlobalAlloc {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let alloc_id = alloc.internal(&mut *tables, tcx);
        tables.tcx.global_alloc(alloc_id).stable(&mut *tables)
    }

    fn vtable_allocation(
        &self,
        global_alloc: &GlobalAlloc,
    ) -> Option<stable_mir::mir::alloc::AllocId> {
        let mut tables = self.0.borrow_mut();
        let GlobalAlloc::VTable(ty, trait_ref) = global_alloc else {
            return None;
        };
        let tcx = tables.tcx;
        let alloc_id = tables.tcx.vtable_allocation((
            ty.internal(&mut *tables, tcx),
            trait_ref.internal(&mut *tables, tcx),
        ));
        Some(alloc_id.stable(&mut *tables))
    }

    fn krate(&self, def_id: stable_mir::DefId) -> Crate {
        let tables = self.0.borrow();
        smir_crate(tables.tcx, tables[def_id].krate)
    }

    /// Retrieve the instance name for diagnostic messages.
    ///
    /// This will return the specialized name, e.g., `Vec<char>::new`.
    fn instance_name(&self, def: InstanceDef, trimmed: bool) -> Symbol {
        let tables = self.0.borrow_mut();
        let instance = tables.instances[def];
        if trimmed {
            with_forced_trimmed_paths!(
                tables.tcx.def_path_str_with_args(instance.def_id(), instance.args)
            )
        } else {
            with_no_trimmed_paths!(
                tables.tcx.def_path_str_with_args(instance.def_id(), instance.args)
            )
        }
    }

    /// Retrieve the plain intrinsic name of an instance.
    ///
    /// This assumes that the instance is an intrinsic.
    fn intrinsic_name(&self, def: InstanceDef) -> Symbol {
        let tables = self.0.borrow_mut();
        let instance = tables.instances[def];
        let intrinsic = tables.tcx.intrinsic(instance.def_id()).unwrap();
        intrinsic.name.to_string()
    }

    fn ty_layout(&self, ty: Ty) -> Result<Layout, Error> {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        let ty = ty.internal(&mut *tables, tcx);
        let layout = tables.layout_of(ty)?.layout;
        Ok(layout.stable(&mut *tables))
    }

    fn layout_shape(&self, id: Layout) -> LayoutShape {
        let mut tables = self.0.borrow_mut();
        let tcx = tables.tcx;
        id.internal(&mut *tables, tcx).0.stable(&mut *tables)
    }
}

pub struct TablesWrapper<'tcx>(pub RefCell<Tables<'tcx>>);

/// Implement error handling for extracting function ABI information.
impl<'tcx> FnAbiOfHelpers<'tcx> for Tables<'tcx> {
    type FnAbiOfResult = Result<&'tcx rustc_target::abi::call::FnAbi<'tcx, ty::Ty<'tcx>>, Error>;

    #[inline]
    fn handle_fn_abi_err(
        &self,
        err: ty::layout::FnAbiError<'tcx>,
        _span: rustc_span::Span,
        fn_abi_request: ty::layout::FnAbiRequest<'tcx>,
    ) -> Error {
        Error::new(format!("Failed to get ABI for `{fn_abi_request:?}`: {err:?}"))
    }
}

impl<'tcx> LayoutOfHelpers<'tcx> for Tables<'tcx> {
    type LayoutOfResult = Result<ty::layout::TyAndLayout<'tcx>, Error>;

    #[inline]
    fn handle_layout_err(
        &self,
        err: ty::layout::LayoutError<'tcx>,
        _span: rustc_span::Span,
        ty: ty::Ty<'tcx>,
    ) -> Error {
        Error::new(format!("Failed to get layout for `{ty}`: {err}"))
    }
}

impl<'tcx> HasParamEnv<'tcx> for Tables<'tcx> {
    fn param_env(&self) -> ty::ParamEnv<'tcx> {
        ty::ParamEnv::reveal_all()
    }
}

impl<'tcx> HasTyCtxt<'tcx> for Tables<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

impl<'tcx> HasDataLayout for Tables<'tcx> {
    fn data_layout(&self) -> &rustc_abi::TargetDataLayout {
        self.tcx.data_layout()
    }
}
