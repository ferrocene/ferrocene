use crate::mir::interpret::{AllocRange, GlobalAlloc, Pointer, Provenance, Scalar};
use crate::query::IntoQueryParam;
use crate::query::Providers;
use crate::traits::util::{super_predicates_for_pretty_printing, supertraits_for_pretty_printing};
use crate::ty::GenericArgKind;
use crate::ty::{
    ConstInt, ParamConst, ScalarInt, Term, TermKind, TypeFoldable, TypeSuperFoldable,
    TypeSuperVisitable, TypeVisitable, TypeVisitableExt,
};
use rustc_apfloat::ieee::{Double, Single};
use rustc_apfloat::Float;
use rustc_data_structures::fx::{FxHashMap, FxIndexMap};
use rustc_hir as hir;
use rustc_hir::def::{self, CtorKind, DefKind, Namespace};
use rustc_hir::def_id::{DefIdMap, DefIdSet, ModDefId, CRATE_DEF_ID, LOCAL_CRATE};
use rustc_hir::definitions::{DefKey, DefPathDataName};
use rustc_hir::LangItem;
use rustc_session::cstore::{ExternCrate, ExternCrateSource};
use rustc_session::Limit;
use rustc_span::symbol::{kw, Ident, Symbol};
use rustc_span::FileNameDisplayPreference;
use rustc_target::abi::Size;
use rustc_target::spec::abi::Abi;
use smallvec::SmallVec;

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt::{self, Write as _};
use std::iter;
use std::ops::{Deref, DerefMut};

// `pretty` is a separate module only for organization.
use super::*;

macro_rules! p {
    (@$lit:literal) => {
        write!(scoped_cx!(), $lit)?
    };
    (@write($($data:expr),+)) => {
        write!(scoped_cx!(), $($data),+)?
    };
    (@print($x:expr)) => {
        $x.print(scoped_cx!())?
    };
    (@$method:ident($($arg:expr),*)) => {
        scoped_cx!().$method($($arg),*)?
    };
    ($($elem:tt $(($($args:tt)*))?),+) => {{
        $(p!(@ $elem $(($($args)*))?);)+
    }};
}
macro_rules! define_scoped_cx {
    ($cx:ident) => {
        macro_rules! scoped_cx {
            () => {
                $cx
            };
        }
    };
}

thread_local! {
    static FORCE_IMPL_FILENAME_LINE: Cell<bool> = const { Cell::new(false) };
    static SHOULD_PREFIX_WITH_CRATE: Cell<bool> = const { Cell::new(false) };
    static NO_TRIMMED_PATH: Cell<bool> = const { Cell::new(false) };
    static FORCE_TRIMMED_PATH: Cell<bool> = const { Cell::new(false) };
    static REDUCED_QUERIES: Cell<bool> = const { Cell::new(false) };
    static NO_VISIBLE_PATH: Cell<bool> = const { Cell::new(false) };
}

macro_rules! define_helper {
    ($($(#[$a:meta])* fn $name:ident($helper:ident, $tl:ident);)+) => {
        $(
            #[must_use]
            pub struct $helper(bool);

            impl $helper {
                pub fn new() -> $helper {
                    $helper($tl.with(|c| c.replace(true)))
                }
            }

            $(#[$a])*
            pub macro $name($e:expr) {
                {
                    let _guard = $helper::new();
                    $e
                }
            }

            impl Drop for $helper {
                fn drop(&mut self) {
                    $tl.with(|c| c.set(self.0))
                }
            }

            pub fn $name() -> bool {
                $tl.with(|c| c.get())
            }
        )+
    }
}

define_helper!(
    /// Avoids running select queries during any prints that occur
    /// during the closure. This may alter the appearance of some
    /// types (e.g. forcing verbose printing for opaque types).
    /// This method is used during some queries (e.g. `explicit_item_bounds`
    /// for opaque types), to ensure that any debug printing that
    /// occurs during the query computation does not end up recursively
    /// calling the same query.
    fn with_reduced_queries(ReducedQueriesGuard, REDUCED_QUERIES);
    /// Force us to name impls with just the filename/line number. We
    /// normally try to use types. But at some points, notably while printing
    /// cycle errors, this can result in extra or suboptimal error output,
    /// so this variable disables that check.
    fn with_forced_impl_filename_line(ForcedImplGuard, FORCE_IMPL_FILENAME_LINE);
    /// Adds the `crate::` prefix to paths where appropriate.
    fn with_crate_prefix(CratePrefixGuard, SHOULD_PREFIX_WITH_CRATE);
    /// Prevent path trimming if it is turned on. Path trimming affects `Display` impl
    /// of various rustc types, for example `std::vec::Vec` would be trimmed to `Vec`,
    /// if no other `Vec` is found.
    fn with_no_trimmed_paths(NoTrimmedGuard, NO_TRIMMED_PATH);
    fn with_forced_trimmed_paths(ForceTrimmedGuard, FORCE_TRIMMED_PATH);
    /// Prevent selection of visible paths. `Display` impl of DefId will prefer
    /// visible (public) reexports of types as paths.
    fn with_no_visible_paths(NoVisibleGuard, NO_VISIBLE_PATH);
);

/// Avoids running any queries during prints.
pub macro with_no_queries($e:expr) {{
    $crate::ty::print::with_reduced_queries!($crate::ty::print::with_forced_impl_filename_line!(
        $crate::ty::print::with_no_trimmed_paths!($crate::ty::print::with_no_visible_paths!(
            $crate::ty::print::with_forced_impl_filename_line!($e)
        ))
    ))
}}

/// The "region highlights" are used to control region printing during
/// specific error messages. When a "region highlight" is enabled, it
/// gives an alternate way to print specific regions. For now, we
/// always print those regions using a number, so something like "`'0`".
///
/// Regions not selected by the region highlight mode are presently
/// unaffected.
#[derive(Copy, Clone, Default)]
pub struct RegionHighlightMode<'tcx> {
    /// If enabled, when we see the selected region, use "`'N`"
    /// instead of the ordinary behavior.
    highlight_regions: [Option<(ty::Region<'tcx>, usize)>; 3],

    /// If enabled, when printing a "free region" that originated from
    /// the given `ty::BoundRegionKind`, print it as "`'1`". Free regions that would ordinarily
    /// have names print as normal.
    ///
    /// This is used when you have a signature like `fn foo(x: &u32,
    /// y: &'a u32)` and we want to give a name to the region of the
    /// reference `x`.
    highlight_bound_region: Option<(ty::BoundRegionKind, usize)>,
}

impl<'tcx> RegionHighlightMode<'tcx> {
    /// If `region` and `number` are both `Some`, invokes
    /// `highlighting_region`.
    pub fn maybe_highlighting_region(
        &mut self,
        region: Option<ty::Region<'tcx>>,
        number: Option<usize>,
    ) {
        if let Some(k) = region {
            if let Some(n) = number {
                self.highlighting_region(k, n);
            }
        }
    }

    /// Highlights the region inference variable `vid` as `'N`.
    pub fn highlighting_region(&mut self, region: ty::Region<'tcx>, number: usize) {
        let num_slots = self.highlight_regions.len();
        let first_avail_slot =
            self.highlight_regions.iter_mut().find(|s| s.is_none()).unwrap_or_else(|| {
                bug!("can only highlight {} placeholders at a time", num_slots,)
            });
        *first_avail_slot = Some((region, number));
    }

    /// Convenience wrapper for `highlighting_region`.
    pub fn highlighting_region_vid(
        &mut self,
        tcx: TyCtxt<'tcx>,
        vid: ty::RegionVid,
        number: usize,
    ) {
        self.highlighting_region(ty::Region::new_var(tcx, vid), number)
    }

    /// Returns `Some(n)` with the number to use for the given region, if any.
    fn region_highlighted(&self, region: ty::Region<'tcx>) -> Option<usize> {
        self.highlight_regions.iter().find_map(|h| match h {
            Some((r, n)) if *r == region => Some(*n),
            _ => None,
        })
    }

    /// Highlight the given bound region.
    /// We can only highlight one bound region at a time. See
    /// the field `highlight_bound_region` for more detailed notes.
    pub fn highlighting_bound_region(&mut self, br: ty::BoundRegionKind, number: usize) {
        assert!(self.highlight_bound_region.is_none());
        self.highlight_bound_region = Some((br, number));
    }
}

/// Trait for printers that pretty-print using `fmt::Write` to the printer.
pub trait PrettyPrinter<'tcx>: Printer<'tcx> + fmt::Write {
    /// Like `print_def_path` but for value paths.
    fn print_value_path(
        &mut self,
        def_id: DefId,
        args: &'tcx [GenericArg<'tcx>],
    ) -> Result<(), PrintError> {
        self.print_def_path(def_id, args)
    }

    fn in_binder<T>(&mut self, value: &ty::Binder<'tcx, T>) -> Result<(), PrintError>
    where
        T: Print<'tcx, Self> + TypeFoldable<TyCtxt<'tcx>>,
    {
        value.as_ref().skip_binder().print(self)
    }

    fn wrap_binder<T, F: FnOnce(&T, &mut Self) -> Result<(), fmt::Error>>(
        &mut self,
        value: &ty::Binder<'tcx, T>,
        f: F,
    ) -> Result<(), PrintError>
    where
        T: Print<'tcx, Self> + TypeFoldable<TyCtxt<'tcx>>,
    {
        f(value.as_ref().skip_binder(), self)
    }

    /// Prints comma-separated elements.
    fn comma_sep<T>(&mut self, mut elems: impl Iterator<Item = T>) -> Result<(), PrintError>
    where
        T: Print<'tcx, Self>,
    {
        if let Some(first) = elems.next() {
            first.print(self)?;
            for elem in elems {
                self.write_str(", ")?;
                elem.print(self)?;
            }
        }
        Ok(())
    }

    /// Prints `{f: t}` or `{f as t}` depending on the `cast` argument
    fn typed_value(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), PrintError>,
        t: impl FnOnce(&mut Self) -> Result<(), PrintError>,
        conversion: &str,
    ) -> Result<(), PrintError> {
        self.write_str("{")?;
        f(self)?;
        self.write_str(conversion)?;
        t(self)?;
        self.write_str("}")?;
        Ok(())
    }

    /// Prints `<...>` around what `f` prints.
    fn generic_delimiters(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), PrintError>,
    ) -> Result<(), PrintError>;

    /// Returns `true` if the region should be printed in
    /// optional positions, e.g., `&'a T` or `dyn Tr + 'b`.
    /// This is typically the case for all non-`'_` regions.
    fn should_print_region(&self, region: ty::Region<'tcx>) -> bool;

    fn reset_type_limit(&mut self) {}

    // Defaults (should not be overridden):

    /// If possible, this returns a global path resolving to `def_id` that is visible
    /// from at least one local module, and returns `true`. If the crate defining `def_id` is
    /// declared with an `extern crate`, the path is guaranteed to use the `extern crate`.
    fn try_print_visible_def_path(&mut self, def_id: DefId) -> Result<bool, PrintError> {
        if with_no_visible_paths() {
            return Ok(false);
        }

        let mut callers = Vec::new();
        self.try_print_visible_def_path_recur(def_id, &mut callers)
    }

    // Given a `DefId`, produce a short name. For types and traits, it prints *only* its name,
    // For associated items on traits it prints out the trait's name and the associated item's name.
    // For enum variants, if they have an unique name, then we only print the name, otherwise we
    // print the enum name and the variant name. Otherwise, we do not print anything and let the
    // caller use the `print_def_path` fallback.
    fn force_print_trimmed_def_path(&mut self, def_id: DefId) -> Result<bool, PrintError> {
        let key = self.tcx().def_key(def_id);
        let visible_parent_map = self.tcx().visible_parent_map(());
        let kind = self.tcx().def_kind(def_id);

        let get_local_name = |this: &Self, name, def_id, key: DefKey| {
            if let Some(visible_parent) = visible_parent_map.get(&def_id)
                && let actual_parent = this.tcx().opt_parent(def_id)
                && let DefPathData::TypeNs(_) = key.disambiguated_data.data
                && Some(*visible_parent) != actual_parent
            {
                this.tcx()
                    // FIXME(typed_def_id): Further propagate ModDefId
                    .module_children(ModDefId::new_unchecked(*visible_parent))
                    .iter()
                    .filter(|child| child.res.opt_def_id() == Some(def_id))
                    .find(|child| child.vis.is_public() && child.ident.name != kw::Underscore)
                    .map(|child| child.ident.name)
                    .unwrap_or(name)
            } else {
                name
            }
        };
        if let DefKind::Variant = kind
            && let Some(symbol) = self.tcx().trimmed_def_paths(()).get(&def_id)
        {
            // If `Assoc` is unique, we don't want to talk about `Trait::Assoc`.
            self.write_str(get_local_name(self, *symbol, def_id, key).as_str())?;
            return Ok(true);
        }
        if let Some(symbol) = key.get_opt_name() {
            if let DefKind::AssocConst | DefKind::AssocFn | DefKind::AssocTy = kind
                && let Some(parent) = self.tcx().opt_parent(def_id)
                && let parent_key = self.tcx().def_key(parent)
                && let Some(symbol) = parent_key.get_opt_name()
            {
                // Trait
                self.write_str(get_local_name(self, symbol, parent, parent_key).as_str())?;
                self.write_str("::")?;
            } else if let DefKind::Variant = kind
                && let Some(parent) = self.tcx().opt_parent(def_id)
                && let parent_key = self.tcx().def_key(parent)
                && let Some(symbol) = parent_key.get_opt_name()
            {
                // Enum

                // For associated items and variants, we want the "full" path, namely, include
                // the parent type in the path. For example, `Iterator::Item`.
                self.write_str(get_local_name(self, symbol, parent, parent_key).as_str())?;
                self.write_str("::")?;
            } else if let DefKind::Struct
            | DefKind::Union
            | DefKind::Enum
            | DefKind::Trait
            | DefKind::TyAlias
            | DefKind::Fn
            | DefKind::Const
            | DefKind::Static { .. } = kind
            {
            } else {
                // If not covered above, like for example items out of `impl` blocks, fallback.
                return Ok(false);
            }
            self.write_str(get_local_name(self, symbol, def_id, key).as_str())?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Try to see if this path can be trimmed to a unique symbol name.
    fn try_print_trimmed_def_path(&mut self, def_id: DefId) -> Result<bool, PrintError> {
        if with_forced_trimmed_paths() && self.force_print_trimmed_def_path(def_id)? {
            return Ok(true);
        }
        if self.tcx().sess.opts.unstable_opts.trim_diagnostic_paths
            && self.tcx().sess.opts.trimmed_def_paths
            && !with_no_trimmed_paths()
            && !with_crate_prefix()
            && let Some(symbol) = self.tcx().trimmed_def_paths(()).get(&def_id)
        {
            write!(self, "{}", Ident::with_dummy_span(*symbol))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Does the work of `try_print_visible_def_path`, building the
    /// full definition path recursively before attempting to
    /// post-process it into the valid and visible version that
    /// accounts for re-exports.
    ///
    /// This method should only be called by itself or
    /// `try_print_visible_def_path`.
    ///
    /// `callers` is a chain of visible_parent's leading to `def_id`,
    /// to support cycle detection during recursion.
    ///
    /// This method returns false if we can't print the visible path, so
    /// `print_def_path` can fall back on the item's real definition path.
    fn try_print_visible_def_path_recur(
        &mut self,
        def_id: DefId,
        callers: &mut Vec<DefId>,
    ) -> Result<bool, PrintError> {
        debug!("try_print_visible_def_path: def_id={:?}", def_id);

        // If `def_id` is a direct or injected extern crate, return the
        // path to the crate followed by the path to the item within the crate.
        if let Some(cnum) = def_id.as_crate_root() {
            if cnum == LOCAL_CRATE {
                self.path_crate(cnum)?;
                return Ok(true);
            }

            // In local mode, when we encounter a crate other than
            // LOCAL_CRATE, execution proceeds in one of two ways:
            //
            // 1. For a direct dependency, where user added an
            //    `extern crate` manually, we put the `extern
            //    crate` as the parent. So you wind up with
            //    something relative to the current crate.
            // 2. For an extern inferred from a path or an indirect crate,
            //    where there is no explicit `extern crate`, we just prepend
            //    the crate name.
            match self.tcx().extern_crate(def_id) {
                Some(&ExternCrate { src, dependency_of, span, .. }) => match (src, dependency_of) {
                    (ExternCrateSource::Extern(def_id), LOCAL_CRATE) => {
                        // NOTE(eddyb) the only reason `span` might be dummy,
                        // that we're aware of, is that it's the `std`/`core`
                        // `extern crate` injected by default.
                        // FIXME(eddyb) find something better to key this on,
                        // or avoid ending up with `ExternCrateSource::Extern`,
                        // for the injected `std`/`core`.
                        if span.is_dummy() {
                            self.path_crate(cnum)?;
                            return Ok(true);
                        }

                        // Disable `try_print_trimmed_def_path` behavior within
                        // the `print_def_path` call, to avoid infinite recursion
                        // in cases where the `extern crate foo` has non-trivial
                        // parents, e.g. it's nested in `impl foo::Trait for Bar`
                        // (see also issues #55779 and #87932).
                        with_no_visible_paths!(self.print_def_path(def_id, &[])?);

                        return Ok(true);
                    }
                    (ExternCrateSource::Path, LOCAL_CRATE) => {
                        self.path_crate(cnum)?;
                        return Ok(true);
                    }
                    _ => {}
                },
                None => {
                    self.path_crate(cnum)?;
                    return Ok(true);
                }
            }
        }

        if def_id.is_local() {
            return Ok(false);
        }

        let visible_parent_map = self.tcx().visible_parent_map(());

        let mut cur_def_key = self.tcx().def_key(def_id);
        debug!("try_print_visible_def_path: cur_def_key={:?}", cur_def_key);

        // For a constructor, we want the name of its parent rather than <unnamed>.
        if let DefPathData::Ctor = cur_def_key.disambiguated_data.data {
            let parent = DefId {
                krate: def_id.krate,
                index: cur_def_key
                    .parent
                    .expect("`DefPathData::Ctor` / `VariantData` missing a parent"),
            };

            cur_def_key = self.tcx().def_key(parent);
        }

        let Some(visible_parent) = visible_parent_map.get(&def_id).cloned() else {
            return Ok(false);
        };

        let actual_parent = self.tcx().opt_parent(def_id);
        debug!(
            "try_print_visible_def_path: visible_parent={:?} actual_parent={:?}",
            visible_parent, actual_parent,
        );

        let mut data = cur_def_key.disambiguated_data.data;
        debug!(
            "try_print_visible_def_path: data={:?} visible_parent={:?} actual_parent={:?}",
            data, visible_parent, actual_parent,
        );

        match data {
            // In order to output a path that could actually be imported (valid and visible),
            // we need to handle re-exports correctly.
            //
            // For example, take `std::os::unix::process::CommandExt`, this trait is actually
            // defined at `std::sys::unix::ext::process::CommandExt` (at time of writing).
            //
            // `std::os::unix` reexports the contents of `std::sys::unix::ext`. `std::sys` is
            // private so the "true" path to `CommandExt` isn't accessible.
            //
            // In this case, the `visible_parent_map` will look something like this:
            //
            // (child) -> (parent)
            // `std::sys::unix::ext::process::CommandExt` -> `std::sys::unix::ext::process`
            // `std::sys::unix::ext::process` -> `std::sys::unix::ext`
            // `std::sys::unix::ext` -> `std::os`
            //
            // This is correct, as the visible parent of `std::sys::unix::ext` is in fact
            // `std::os`.
            //
            // When printing the path to `CommandExt` and looking at the `cur_def_key` that
            // corresponds to `std::sys::unix::ext`, we would normally print `ext` and then go
            // to the parent - resulting in a mangled path like
            // `std::os::ext::process::CommandExt`.
            //
            // Instead, we must detect that there was a re-export and instead print `unix`
            // (which is the name `std::sys::unix::ext` was re-exported as in `std::os`). To
            // do this, we compare the parent of `std::sys::unix::ext` (`std::sys::unix`) with
            // the visible parent (`std::os`). If these do not match, then we iterate over
            // the children of the visible parent (as was done when computing
            // `visible_parent_map`), looking for the specific child we currently have and then
            // have access to the re-exported name.
            DefPathData::TypeNs(ref mut name) if Some(visible_parent) != actual_parent => {
                // Item might be re-exported several times, but filter for the one
                // that's public and whose identifier isn't `_`.
                let reexport = self
                    .tcx()
                    // FIXME(typed_def_id): Further propagate ModDefId
                    .module_children(ModDefId::new_unchecked(visible_parent))
                    .iter()
                    .filter(|child| child.res.opt_def_id() == Some(def_id))
                    .find(|child| child.vis.is_public() && child.ident.name != kw::Underscore)
                    .map(|child| child.ident.name);

                if let Some(new_name) = reexport {
                    *name = new_name;
                } else {
                    // There is no name that is public and isn't `_`, so bail.
                    return Ok(false);
                }
            }
            // Re-exported `extern crate` (#43189).
            DefPathData::CrateRoot => {
                data = DefPathData::TypeNs(self.tcx().crate_name(def_id.krate));
            }
            _ => {}
        }
        debug!("try_print_visible_def_path: data={:?}", data);

        if callers.contains(&visible_parent) {
            return Ok(false);
        }
        callers.push(visible_parent);
        // HACK(eddyb) this bypasses `path_append`'s prefix printing to avoid
        // knowing ahead of time whether the entire path will succeed or not.
        // To support printers that do not implement `PrettyPrinter`, a `Vec` or
        // linked list on the stack would need to be built, before any printing.
        match self.try_print_visible_def_path_recur(visible_parent, callers)? {
            false => return Ok(false),
            true => {}
        }
        callers.pop();
        self.path_append(|_| Ok(()), &DisambiguatedDefPathData { data, disambiguator: 0 })?;
        Ok(true)
    }

    fn pretty_path_qualified(
        &mut self,
        self_ty: Ty<'tcx>,
        trait_ref: Option<ty::TraitRef<'tcx>>,
    ) -> Result<(), PrintError> {
        if trait_ref.is_none() {
            // Inherent impls. Try to print `Foo::bar` for an inherent
            // impl on `Foo`, but fallback to `<Foo>::bar` if self-type is
            // anything other than a simple path.
            match self_ty.kind() {
                ty::Adt(..)
                | ty::Foreign(_)
                | ty::Bool
                | ty::Char
                | ty::Str
                | ty::Int(_)
                | ty::Uint(_)
                | ty::Float(_) => {
                    return self_ty.print(self);
                }

                _ => {}
            }
        }

        self.generic_delimiters(|cx| {
            define_scoped_cx!(cx);

            p!(print(self_ty));
            if let Some(trait_ref) = trait_ref {
                p!(" as ", print(trait_ref.print_only_trait_path()));
            }
            Ok(())
        })
    }

    fn pretty_path_append_impl(
        &mut self,
        print_prefix: impl FnOnce(&mut Self) -> Result<(), PrintError>,
        self_ty: Ty<'tcx>,
        trait_ref: Option<ty::TraitRef<'tcx>>,
    ) -> Result<(), PrintError> {
        print_prefix(self)?;

        self.generic_delimiters(|cx| {
            define_scoped_cx!(cx);

            p!("impl ");
            if let Some(trait_ref) = trait_ref {
                p!(print(trait_ref.print_only_trait_path()), " for ");
            }
            p!(print(self_ty));

            Ok(())
        })
    }

    fn pretty_print_type(&mut self, ty: Ty<'tcx>) -> Result<(), PrintError> {
        define_scoped_cx!(self);

        match *ty.kind() {
            ty::Bool => p!("bool"),
            ty::Char => p!("char"),
            ty::Int(t) => p!(write("{}", t.name_str())),
            ty::Uint(t) => p!(write("{}", t.name_str())),
            ty::Float(t) => p!(write("{}", t.name_str())),
            ty::RawPtr(ref tm) => {
                p!(write(
                    "*{} ",
                    match tm.mutbl {
                        hir::Mutability::Mut => "mut",
                        hir::Mutability::Not => "const",
                    }
                ));
                p!(print(tm.ty))
            }
            ty::Ref(r, ty, mutbl) => {
                p!("&");
                if self.should_print_region(r) {
                    p!(print(r), " ");
                }
                p!(print(ty::TypeAndMut { ty, mutbl }))
            }
            ty::Never => p!("!"),
            ty::Tuple(tys) => {
                p!("(", comma_sep(tys.iter()));
                if tys.len() == 1 {
                    p!(",");
                }
                p!(")")
            }
            ty::FnDef(def_id, args) => {
                if with_reduced_queries() {
                    p!(print_def_path(def_id, args));
                } else {
                    let sig = self.tcx().fn_sig(def_id).instantiate(self.tcx(), args);
                    p!(print(sig), " {{", print_value_path(def_id, args), "}}");
                }
            }
            ty::FnPtr(ref bare_fn) => p!(print(bare_fn)),
            ty::Infer(infer_ty) => {
                if self.should_print_verbose() {
                    p!(write("{:?}", ty.kind()));
                    return Ok(());
                }

                if let ty::TyVar(ty_vid) = infer_ty {
                    if let Some(name) = self.ty_infer_name(ty_vid) {
                        p!(write("{}", name))
                    } else {
                        p!(write("{}", infer_ty))
                    }
                } else {
                    p!(write("{}", infer_ty))
                }
            }
            ty::Error(_) => p!("{{type error}}"),
            ty::Param(ref param_ty) => p!(print(param_ty)),
            ty::Bound(debruijn, bound_ty) => match bound_ty.kind {
                ty::BoundTyKind::Anon => {
                    rustc_type_ir::debug_bound_var(self, debruijn, bound_ty.var)?
                }
                ty::BoundTyKind::Param(_, s) => match self.should_print_verbose() {
                    true => p!(write("{:?}", ty.kind())),
                    false => p!(write("{s}")),
                },
            },
            ty::Adt(def, args) => {
                p!(print_def_path(def.did(), args));
            }
            ty::Dynamic(data, r, repr) => {
                let print_r = self.should_print_region(r);
                if print_r {
                    p!("(");
                }
                match repr {
                    ty::Dyn => p!("dyn "),
                    ty::DynStar => p!("dyn* "),
                }
                p!(print(data));
                if print_r {
                    p!(" + ", print(r), ")");
                }
            }
            ty::Foreign(def_id) => {
                p!(print_def_path(def_id, &[]));
            }
            ty::Alias(ty::Projection | ty::Inherent | ty::Weak, ref data) => {
                p!(print(data))
            }
            ty::Placeholder(placeholder) => match placeholder.bound.kind {
                ty::BoundTyKind::Anon => p!(write("{placeholder:?}")),
                ty::BoundTyKind::Param(_, name) => match self.should_print_verbose() {
                    true => p!(write("{:?}", ty.kind())),
                    false => p!(write("{name}")),
                },
            },
            ty::Alias(ty::Opaque, ty::AliasTy { def_id, args, .. }) => {
                // We use verbose printing in 'NO_QUERIES' mode, to
                // avoid needing to call `predicates_of`. This should
                // only affect certain debug messages (e.g. messages printed
                // from `rustc_middle::ty` during the computation of `tcx.predicates_of`),
                // and should have no effect on any compiler output.
                // [Unless `-Zverbose-internals` is used, e.g. in the output of
                // `tests/ui/nll/ty-outlives/impl-trait-captures.rs`, for
                // example.]
                if self.should_print_verbose() {
                    // FIXME(eddyb) print this with `print_def_path`.
                    p!(write("Opaque({:?}, {})", def_id, args.print_as_list()));
                    return Ok(());
                }

                let parent = self.tcx().parent(def_id);
                match self.tcx().def_kind(parent) {
                    DefKind::TyAlias | DefKind::AssocTy => {
                        // NOTE: I know we should check for NO_QUERIES here, but it's alright.
                        // `type_of` on a type alias or assoc type should never cause a cycle.
                        if let ty::Alias(ty::Opaque, ty::AliasTy { def_id: d, .. }) =
                            *self.tcx().type_of(parent).instantiate_identity().kind()
                        {
                            if d == def_id {
                                // If the type alias directly starts with the `impl` of the
                                // opaque type we're printing, then skip the `::{opaque#1}`.
                                p!(print_def_path(parent, args));
                                return Ok(());
                            }
                        }
                        // Complex opaque type, e.g. `type Foo = (i32, impl Debug);`
                        p!(print_def_path(def_id, args));
                        return Ok(());
                    }
                    _ => {
                        if with_reduced_queries() {
                            p!(print_def_path(def_id, &[]));
                            return Ok(());
                        } else {
                            return self.pretty_print_opaque_impl_type(def_id, args);
                        }
                    }
                }
            }
            ty::Str => p!("str"),
            ty::Coroutine(did, args) => {
                p!(write("{{"));
                let coroutine_kind = self.tcx().coroutine_kind(did).unwrap();
                let should_print_movability = self.should_print_verbose()
                    || matches!(coroutine_kind, hir::CoroutineKind::Coroutine(_));

                if should_print_movability {
                    match coroutine_kind.movability() {
                        hir::Movability::Movable => {}
                        hir::Movability::Static => p!("static "),
                    }
                }

                if !self.should_print_verbose() {
                    p!(write("{}", coroutine_kind));
                    // FIXME(eddyb) should use `def_span`.
                    if let Some(did) = did.as_local() {
                        let span = self.tcx().def_span(did);
                        p!(write(
                            "@{}",
                            // This may end up in stderr diagnostics but it may also be emitted
                            // into MIR. Hence we use the remapped path if available
                            self.tcx().sess.source_map().span_to_embeddable_string(span)
                        ));
                    } else {
                        p!(write("@"), print_def_path(did, args));
                    }
                } else {
                    p!(print_def_path(did, args));
                    p!(
                        " upvar_tys=",
                        print(args.as_coroutine().tupled_upvars_ty()),
                        " witness=",
                        print(args.as_coroutine().witness())
                    );
                }

                p!("}}")
            }
            ty::CoroutineWitness(did, args) => {
                p!(write("{{"));
                if !self.tcx().sess.verbose_internals() {
                    p!("coroutine witness");
                    // FIXME(eddyb) should use `def_span`.
                    if let Some(did) = did.as_local() {
                        let span = self.tcx().def_span(did);
                        p!(write(
                            "@{}",
                            // This may end up in stderr diagnostics but it may also be emitted
                            // into MIR. Hence we use the remapped path if available
                            self.tcx().sess.source_map().span_to_embeddable_string(span)
                        ));
                    } else {
                        p!(write("@"), print_def_path(did, args));
                    }
                } else {
                    p!(print_def_path(did, args));
                }

                p!("}}")
            }
            ty::Closure(did, args) => {
                p!(write("{{"));
                if !self.should_print_verbose() {
                    p!(write("closure"));
                    // FIXME(eddyb) should use `def_span`.
                    if let Some(did) = did.as_local() {
                        if self.tcx().sess.opts.unstable_opts.span_free_formats {
                            p!("@", print_def_path(did.to_def_id(), args));
                        } else {
                            let span = self.tcx().def_span(did);
                            let preference = if with_forced_trimmed_paths() {
                                FileNameDisplayPreference::Short
                            } else {
                                FileNameDisplayPreference::Remapped
                            };
                            p!(write(
                                "@{}",
                                // This may end up in stderr diagnostics but it may also be emitted
                                // into MIR. Hence we use the remapped path if available
                                self.tcx().sess.source_map().span_to_string(span, preference)
                            ));
                        }
                    } else {
                        p!(write("@"), print_def_path(did, args));
                    }
                } else {
                    p!(print_def_path(did, args));
                    p!(
                        " closure_kind_ty=",
                        print(args.as_closure().kind_ty()),
                        " closure_sig_as_fn_ptr_ty=",
                        print(args.as_closure().sig_as_fn_ptr_ty()),
                        " upvar_tys=",
                        print(args.as_closure().tupled_upvars_ty())
                    );
                }
                p!("}}");
            }
            ty::CoroutineClosure(did, args) => {
                p!(write("{{"));
                if !self.should_print_verbose() {
                    match self.tcx().coroutine_kind(self.tcx().coroutine_for_closure(did)).unwrap()
                    {
                        hir::CoroutineKind::Desugared(
                            hir::CoroutineDesugaring::Async,
                            hir::CoroutineSource::Closure,
                        ) => p!("async closure"),
                        hir::CoroutineKind::Desugared(
                            hir::CoroutineDesugaring::AsyncGen,
                            hir::CoroutineSource::Closure,
                        ) => p!("async gen closure"),
                        hir::CoroutineKind::Desugared(
                            hir::CoroutineDesugaring::Gen,
                            hir::CoroutineSource::Closure,
                        ) => p!("gen closure"),
                        _ => unreachable!(
                            "coroutine from coroutine-closure should have CoroutineSource::Closure"
                        ),
                    }
                    // FIXME(eddyb) should use `def_span`.
                    if let Some(did) = did.as_local() {
                        if self.tcx().sess.opts.unstable_opts.span_free_formats {
                            p!("@", print_def_path(did.to_def_id(), args));
                        } else {
                            let span = self.tcx().def_span(did);
                            let preference = if with_forced_trimmed_paths() {
                                FileNameDisplayPreference::Short
                            } else {
                                FileNameDisplayPreference::Remapped
                            };
                            p!(write(
                                "@{}",
                                // This may end up in stderr diagnostics but it may also be emitted
                                // into MIR. Hence we use the remapped path if available
                                self.tcx().sess.source_map().span_to_string(span, preference)
                            ));
                        }
                    } else {
                        p!(write("@"), print_def_path(did, args));
                    }
                } else {
                    p!(print_def_path(did, args));
                    p!(
                        " closure_kind_ty=",
                        print(args.as_coroutine_closure().kind_ty()),
                        " signature_parts_ty=",
                        print(args.as_coroutine_closure().signature_parts_ty()),
                        " upvar_tys=",
                        print(args.as_coroutine_closure().tupled_upvars_ty()),
                        " coroutine_captures_by_ref_ty=",
                        print(args.as_coroutine_closure().coroutine_captures_by_ref_ty()),
                        " coroutine_witness_ty=",
                        print(args.as_coroutine_closure().coroutine_witness_ty())
                    );
                }
                p!("}}");
            }
            ty::Array(ty, sz) => p!("[", print(ty), "; ", print(sz), "]"),
            ty::Slice(ty) => p!("[", print(ty), "]"),
        }

        Ok(())
    }

    fn pretty_print_opaque_impl_type(
        &mut self,
        def_id: DefId,
        args: &'tcx ty::List<ty::GenericArg<'tcx>>,
    ) -> Result<(), PrintError> {
        let tcx = self.tcx();

        // Grab the "TraitA + TraitB" from `impl TraitA + TraitB`,
        // by looking up the projections associated with the def_id.
        let bounds = tcx.explicit_item_bounds(def_id);

        let mut traits = FxIndexMap::default();
        let mut fn_traits = FxIndexMap::default();
        let mut has_sized_bound = false;
        let mut has_negative_sized_bound = false;
        let mut lifetimes = SmallVec::<[ty::Region<'tcx>; 1]>::new();

        for (predicate, _) in bounds.iter_instantiated_copied(tcx, args) {
            let bound_predicate = predicate.kind();

            match bound_predicate.skip_binder() {
                ty::ClauseKind::Trait(pred) => {
                    let trait_ref = bound_predicate.rebind(pred.trait_ref);

                    // Don't print `+ Sized`, but rather `+ ?Sized` if absent.
                    if Some(trait_ref.def_id()) == tcx.lang_items().sized_trait() {
                        match pred.polarity {
                            ty::ImplPolarity::Positive | ty::ImplPolarity::Reservation => {
                                has_sized_bound = true;
                                continue;
                            }
                            ty::ImplPolarity::Negative => has_negative_sized_bound = true,
                        }
                    }

                    self.insert_trait_and_projection(
                        trait_ref,
                        pred.polarity,
                        None,
                        &mut traits,
                        &mut fn_traits,
                    );
                }
                ty::ClauseKind::Projection(pred) => {
                    let proj_ref = bound_predicate.rebind(pred);
                    let trait_ref = proj_ref.required_poly_trait_ref(tcx);

                    // Projection type entry -- the def-id for naming, and the ty.
                    let proj_ty = (proj_ref.projection_def_id(), proj_ref.term());

                    self.insert_trait_and_projection(
                        trait_ref,
                        ty::ImplPolarity::Positive,
                        Some(proj_ty),
                        &mut traits,
                        &mut fn_traits,
                    );
                }
                ty::ClauseKind::TypeOutlives(outlives) => {
                    lifetimes.push(outlives.1);
                }
                _ => {}
            }
        }

        write!(self, "impl ")?;

        let mut first = true;
        // Insert parenthesis around (Fn(A, B) -> C) if the opaque ty has more than one other trait
        let paren_needed = fn_traits.len() > 1 || traits.len() > 0 || !has_sized_bound;

        for (fn_once_trait_ref, entry) in fn_traits {
            write!(self, "{}", if first { "" } else { " + " })?;
            write!(self, "{}", if paren_needed { "(" } else { "" })?;

            self.wrap_binder(&fn_once_trait_ref, |trait_ref, cx| {
                define_scoped_cx!(cx);
                // Get the (single) generic ty (the args) of this FnOnce trait ref.
                let generics = tcx.generics_of(trait_ref.def_id);
                let own_args = generics.own_args_no_defaults(tcx, trait_ref.args);

                match (entry.return_ty, own_args[0].expect_ty()) {
                    // We can only print `impl Fn() -> ()` if we have a tuple of args and we recorded
                    // a return type.
                    (Some(return_ty), arg_tys) if matches!(arg_tys.kind(), ty::Tuple(_)) => {
                        let name = if entry.fn_trait_ref.is_some() {
                            "Fn"
                        } else if entry.fn_mut_trait_ref.is_some() {
                            "FnMut"
                        } else {
                            "FnOnce"
                        };

                        p!(write("{}(", name));

                        for (idx, ty) in arg_tys.tuple_fields().iter().enumerate() {
                            if idx > 0 {
                                p!(", ");
                            }
                            p!(print(ty));
                        }

                        p!(")");
                        if let Some(ty) = return_ty.skip_binder().ty() {
                            if !ty.is_unit() {
                                p!(" -> ", print(return_ty));
                            }
                        }
                        p!(write("{}", if paren_needed { ")" } else { "" }));

                        first = false;
                    }
                    // If we got here, we can't print as a `impl Fn(A, B) -> C`. Just record the
                    // trait_refs we collected in the OpaqueFnEntry as normal trait refs.
                    _ => {
                        if entry.has_fn_once {
                            traits
                                .entry((fn_once_trait_ref, ty::ImplPolarity::Positive))
                                .or_default()
                                .extend(
                                    // Group the return ty with its def id, if we had one.
                                    entry.return_ty.map(|ty| {
                                        (tcx.require_lang_item(LangItem::FnOnceOutput, None), ty)
                                    }),
                                );
                        }
                        if let Some(trait_ref) = entry.fn_mut_trait_ref {
                            traits.entry((trait_ref, ty::ImplPolarity::Positive)).or_default();
                        }
                        if let Some(trait_ref) = entry.fn_trait_ref {
                            traits.entry((trait_ref, ty::ImplPolarity::Positive)).or_default();
                        }
                    }
                }

                Ok(())
            })?;
        }

        // Print the rest of the trait types (that aren't Fn* family of traits)
        for ((trait_ref, polarity), assoc_items) in traits {
            write!(self, "{}", if first { "" } else { " + " })?;

            self.wrap_binder(&trait_ref, |trait_ref, cx| {
                define_scoped_cx!(cx);

                if polarity == ty::ImplPolarity::Negative {
                    p!("!");
                }
                p!(print(trait_ref.print_only_trait_name()));

                let generics = tcx.generics_of(trait_ref.def_id);
                let own_args = generics.own_args_no_defaults(tcx, trait_ref.args);

                if !own_args.is_empty() || !assoc_items.is_empty() {
                    let mut first = true;

                    for ty in own_args {
                        if first {
                            p!("<");
                            first = false;
                        } else {
                            p!(", ");
                        }
                        p!(print(ty));
                    }

                    for (assoc_item_def_id, term) in assoc_items {
                        // Skip printing `<{coroutine@} as Coroutine<_>>::Return` from async blocks,
                        // unless we can find out what coroutine return type it comes from.
                        let term = if let Some(ty) = term.skip_binder().ty()
                            && let ty::Alias(ty::Projection, proj) = ty.kind()
                            && let Some(assoc) = tcx.opt_associated_item(proj.def_id)
                            && assoc.trait_container(tcx) == tcx.lang_items().coroutine_trait()
                            && assoc.name == rustc_span::sym::Return
                        {
                            if let ty::Coroutine(_, args) = args.type_at(0).kind() {
                                let return_ty = args.as_coroutine().return_ty();
                                if !return_ty.is_ty_var() {
                                    return_ty.into()
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        } else {
                            term.skip_binder()
                        };

                        if first {
                            p!("<");
                            first = false;
                        } else {
                            p!(", ");
                        }

                        p!(write("{} = ", tcx.associated_item(assoc_item_def_id).name));

                        match term.unpack() {
                            TermKind::Ty(ty) => p!(print(ty)),
                            TermKind::Const(c) => p!(print(c)),
                        };
                    }

                    if !first {
                        p!(">");
                    }
                }

                first = false;
                Ok(())
            })?;
        }

        let add_sized = has_sized_bound && (first || has_negative_sized_bound);
        let add_maybe_sized = !has_sized_bound && !has_negative_sized_bound;
        if add_sized || add_maybe_sized {
            if !first {
                write!(self, " + ")?;
            }
            if add_maybe_sized {
                write!(self, "?")?;
            }
            write!(self, "Sized")?;
        }

        if !with_forced_trimmed_paths() {
            for re in lifetimes {
                write!(self, " + ")?;
                self.print_region(re)?;
            }
        }

        if self.tcx().features().return_type_notation
            && let Some(ty::ImplTraitInTraitData::Trait { fn_def_id, .. }) =
                self.tcx().opt_rpitit_info(def_id)
            && let ty::Alias(_, alias_ty) =
                self.tcx().fn_sig(fn_def_id).skip_binder().output().skip_binder().kind()
            && alias_ty.def_id == def_id
        {
            let num_args = self.tcx().generics_of(fn_def_id).count();
            write!(self, " {{ ")?;
            self.print_def_path(fn_def_id, &args[..num_args])?;
            write!(self, "() }}")?;
        }

        Ok(())
    }

    /// Insert the trait ref and optionally a projection type associated with it into either the
    /// traits map or fn_traits map, depending on if the trait is in the Fn* family of traits.
    fn insert_trait_and_projection(
        &mut self,
        trait_ref: ty::PolyTraitRef<'tcx>,
        polarity: ty::ImplPolarity,
        proj_ty: Option<(DefId, ty::Binder<'tcx, Term<'tcx>>)>,
        traits: &mut FxIndexMap<
            (ty::PolyTraitRef<'tcx>, ty::ImplPolarity),
            FxIndexMap<DefId, ty::Binder<'tcx, Term<'tcx>>>,
        >,
        fn_traits: &mut FxIndexMap<ty::PolyTraitRef<'tcx>, OpaqueFnEntry<'tcx>>,
    ) {
        let trait_def_id = trait_ref.def_id();

        // If our trait_ref is FnOnce or any of its children, project it onto the parent FnOnce
        // super-trait ref and record it there.
        // We skip negative Fn* bounds since they can't use parenthetical notation anyway.
        if polarity == ty::ImplPolarity::Positive
            && let Some(fn_once_trait) = self.tcx().lang_items().fn_once_trait()
        {
            // If we have a FnOnce, then insert it into
            if trait_def_id == fn_once_trait {
                let entry = fn_traits.entry(trait_ref).or_default();
                // Optionally insert the return_ty as well.
                if let Some((_, ty)) = proj_ty {
                    entry.return_ty = Some(ty);
                }
                entry.has_fn_once = true;
                return;
            } else if Some(trait_def_id) == self.tcx().lang_items().fn_mut_trait() {
                let super_trait_ref = supertraits_for_pretty_printing(self.tcx(), trait_ref)
                    .find(|super_trait_ref| super_trait_ref.def_id() == fn_once_trait)
                    .unwrap();

                fn_traits.entry(super_trait_ref).or_default().fn_mut_trait_ref = Some(trait_ref);
                return;
            } else if Some(trait_def_id) == self.tcx().lang_items().fn_trait() {
                let super_trait_ref = supertraits_for_pretty_printing(self.tcx(), trait_ref)
                    .find(|super_trait_ref| super_trait_ref.def_id() == fn_once_trait)
                    .unwrap();

                fn_traits.entry(super_trait_ref).or_default().fn_trait_ref = Some(trait_ref);
                return;
            }
        }

        // Otherwise, just group our traits and projection types.
        traits.entry((trait_ref, polarity)).or_default().extend(proj_ty);
    }

    fn pretty_print_inherent_projection(
        &mut self,
        alias_ty: &ty::AliasTy<'tcx>,
    ) -> Result<(), PrintError> {
        let def_key = self.tcx().def_key(alias_ty.def_id);
        self.path_generic_args(
            |cx| {
                cx.path_append(
                    |cx| cx.path_qualified(alias_ty.self_ty(), None),
                    &def_key.disambiguated_data,
                )
            },
            &alias_ty.args[1..],
        )
    }

    fn ty_infer_name(&self, _: ty::TyVid) -> Option<Symbol> {
        None
    }

    fn const_infer_name(&self, _: ty::ConstVid) -> Option<Symbol> {
        None
    }

    fn pretty_print_dyn_existential(
        &mut self,
        predicates: &'tcx ty::List<ty::PolyExistentialPredicate<'tcx>>,
    ) -> Result<(), PrintError> {
        // Generate the main trait ref, including associated types.
        let mut first = true;

        if let Some(bound_principal) = predicates.principal() {
            self.wrap_binder(&bound_principal, |principal, cx| {
                define_scoped_cx!(cx);
                p!(print_def_path(principal.def_id, &[]));

                let mut resugared = false;

                // Special-case `Fn(...) -> ...` and re-sugar it.
                let fn_trait_kind = cx.tcx().fn_trait_kind_from_def_id(principal.def_id);
                if !cx.should_print_verbose() && fn_trait_kind.is_some() {
                    if let ty::Tuple(tys) = principal.args.type_at(0).kind() {
                        let mut projections = predicates.projection_bounds();
                        if let (Some(proj), None) = (projections.next(), projections.next()) {
                            p!(pretty_fn_sig(
                                tys,
                                false,
                                proj.skip_binder().term.ty().expect("Return type was a const")
                            ));
                            resugared = true;
                        }
                    }
                }

                // HACK(eddyb) this duplicates `FmtPrinter`'s `path_generic_args`,
                // in order to place the projections inside the `<...>`.
                if !resugared {
                    let principal_with_self =
                        principal.with_self_ty(cx.tcx(), cx.tcx().types.trait_object_dummy_self);

                    let args = cx
                        .tcx()
                        .generics_of(principal_with_self.def_id)
                        .own_args_no_defaults(cx.tcx(), principal_with_self.args);

                    let bound_principal_with_self = bound_principal
                        .with_self_ty(cx.tcx(), cx.tcx().types.trait_object_dummy_self);

                    let super_projections: Vec<_> =
                        super_predicates_for_pretty_printing(cx.tcx(), bound_principal_with_self)
                            .filter_map(|clause| clause.as_projection_clause())
                            .collect();

                    let mut projections: Vec<_> = predicates
                        .projection_bounds()
                        .filter(|&proj| {
                            // Filter out projections that are implied by the super predicates.
                            let proj_is_implied = super_projections.iter().any(|&super_proj| {
                                let super_proj = super_proj.map_bound(|super_proj| {
                                    ty::ExistentialProjection::erase_self_ty(cx.tcx(), super_proj)
                                });

                                // This function is sometimes called on types with erased and
                                // anonymized regions, but the super projections can still
                                // contain named regions. So we erase and anonymize everything
                                // here to compare the types modulo regions below.
                                let proj = cx.tcx().erase_regions(proj);
                                let proj = cx.tcx().anonymize_bound_vars(proj);
                                let super_proj = cx.tcx().erase_regions(super_proj);
                                let super_proj = cx.tcx().anonymize_bound_vars(super_proj);

                                proj == super_proj
                            });
                            !proj_is_implied
                        })
                        .map(|proj| {
                            // Skip the binder, because we don't want to print the binder in
                            // front of the associated item.
                            proj.skip_binder()
                        })
                        .collect();

                    projections
                        .sort_by_cached_key(|proj| cx.tcx().item_name(proj.def_id).to_string());

                    if !args.is_empty() || !projections.is_empty() {
                        p!(generic_delimiters(|cx| {
                            cx.comma_sep(args.iter().copied())?;
                            if !args.is_empty() && !projections.is_empty() {
                                write!(cx, ", ")?;
                            }
                            cx.comma_sep(projections.iter().copied())
                        }));
                    }
                }
                Ok(())
            })?;

            first = false;
        }

        define_scoped_cx!(self);

        // Builtin bounds.
        // FIXME(eddyb) avoid printing twice (needed to ensure
        // that the auto traits are sorted *and* printed via cx).
        let mut auto_traits: Vec<_> = predicates.auto_traits().collect();

        // The auto traits come ordered by `DefPathHash`. While
        // `DefPathHash` is *stable* in the sense that it depends on
        // neither the host nor the phase of the moon, it depends
        // "pseudorandomly" on the compiler version and the target.
        //
        // To avoid causing instabilities in compiletest
        // output, sort the auto-traits alphabetically.
        auto_traits.sort_by_cached_key(|did| with_no_trimmed_paths!(self.tcx().def_path_str(*did)));

        for def_id in auto_traits {
            if !first {
                p!(" + ");
            }
            first = false;

            p!(print_def_path(def_id, &[]));
        }

        Ok(())
    }

    fn pretty_fn_sig(
        &mut self,
        inputs: &[Ty<'tcx>],
        c_variadic: bool,
        output: Ty<'tcx>,
    ) -> Result<(), PrintError> {
        define_scoped_cx!(self);

        p!("(", comma_sep(inputs.iter().copied()));
        if c_variadic {
            if !inputs.is_empty() {
                p!(", ");
            }
            p!("...");
        }
        p!(")");
        if !output.is_unit() {
            p!(" -> ", print(output));
        }

        Ok(())
    }

    fn pretty_print_const(
        &mut self,
        ct: ty::Const<'tcx>,
        print_ty: bool,
    ) -> Result<(), PrintError> {
        define_scoped_cx!(self);

        if self.should_print_verbose() {
            p!(write("{:?}", ct));
            return Ok(());
        }

        macro_rules! print_underscore {
            () => {{
                if print_ty {
                    self.typed_value(
                        |this| {
                            write!(this, "_")?;
                            Ok(())
                        },
                        |this| this.print_type(ct.ty()),
                        ": ",
                    )?;
                } else {
                    write!(self, "_")?;
                }
            }};
        }

        match ct.kind() {
            ty::ConstKind::Unevaluated(ty::UnevaluatedConst { def, args }) => {
                match self.tcx().def_kind(def) {
                    DefKind::Const | DefKind::AssocConst => {
                        p!(print_value_path(def, args))
                    }
                    DefKind::AnonConst => {
                        if def.is_local()
                            && let span = self.tcx().def_span(def)
                            && let Ok(snip) = self.tcx().sess.source_map().span_to_snippet(span)
                        {
                            p!(write("{}", snip))
                        } else {
                            // Do not call `print_value_path` as if a parent of this anon const is an impl it will
                            // attempt to print out the impl trait ref i.e. `<T as Trait>::{constant#0}`. This would
                            // cause printing to enter an infinite recursion if the anon const is in the self type i.e.
                            // `impl<T: Default> Default for [T; 32 - 1 - 1 - 1] {`
                            // where we would try to print `<[T; /* print `constant#0` again */] as Default>::{constant#0}`
                            p!(write(
                                "{}::{}",
                                self.tcx().crate_name(def.krate),
                                self.tcx().def_path(def).to_string_no_crate_verbose()
                            ))
                        }
                    }
                    defkind => bug!("`{:?}` has unexpected defkind {:?}", ct, defkind),
                }
            }
            ty::ConstKind::Infer(infer_ct) => match infer_ct {
                ty::InferConst::Var(ct_vid) if let Some(name) = self.const_infer_name(ct_vid) => {
                    p!(write("{}", name))
                }
                _ => print_underscore!(),
            },
            ty::ConstKind::Param(ParamConst { name, .. }) => p!(write("{}", name)),
            ty::ConstKind::Value(value) => {
                return self.pretty_print_const_valtree(value, ct.ty(), print_ty);
            }

            ty::ConstKind::Bound(debruijn, bound_var) => {
                rustc_type_ir::debug_bound_var(self, debruijn, bound_var)?
            }
            ty::ConstKind::Placeholder(placeholder) => p!(write("{placeholder:?}")),
            // FIXME(generic_const_exprs):
            // write out some legible representation of an abstract const?
            ty::ConstKind::Expr(_) => p!("{{const expr}}"),
            ty::ConstKind::Error(_) => p!("{{const error}}"),
        };
        Ok(())
    }

    fn pretty_print_const_scalar(
        &mut self,
        scalar: Scalar,
        ty: Ty<'tcx>,
    ) -> Result<(), PrintError> {
        match scalar {
            Scalar::Ptr(ptr, _size) => self.pretty_print_const_scalar_ptr(ptr, ty),
            Scalar::Int(int) => {
                self.pretty_print_const_scalar_int(int, ty, /* print_ty */ true)
            }
        }
    }

    fn pretty_print_const_scalar_ptr(
        &mut self,
        ptr: Pointer,
        ty: Ty<'tcx>,
    ) -> Result<(), PrintError> {
        define_scoped_cx!(self);

        let (prov, offset) = ptr.into_parts();
        match ty.kind() {
            // Byte strings (&[u8; N])
            ty::Ref(_, inner, _) => {
                if let ty::Array(elem, len) = inner.kind() {
                    if let ty::Uint(ty::UintTy::U8) = elem.kind() {
                        if let ty::ConstKind::Value(ty::ValTree::Leaf(int)) = len.kind() {
                            match self.tcx().try_get_global_alloc(prov.alloc_id()) {
                                Some(GlobalAlloc::Memory(alloc)) => {
                                    let len = int.assert_bits(self.tcx().data_layout.pointer_size);
                                    let range =
                                        AllocRange { start: offset, size: Size::from_bytes(len) };
                                    if let Ok(byte_str) =
                                        alloc.inner().get_bytes_strip_provenance(&self.tcx(), range)
                                    {
                                        p!(pretty_print_byte_str(byte_str))
                                    } else {
                                        p!("<too short allocation>")
                                    }
                                }
                                // FIXME: for statics, vtables, and functions, we could in principle print more detail.
                                Some(GlobalAlloc::Static(def_id)) => {
                                    p!(write("<static({:?})>", def_id))
                                }
                                Some(GlobalAlloc::Function(_)) => p!("<function>"),
                                Some(GlobalAlloc::VTable(..)) => p!("<vtable>"),
                                None => p!("<dangling pointer>"),
                            }
                            return Ok(());
                        }
                    }
                }
            }
            ty::FnPtr(_) => {
                // FIXME: We should probably have a helper method to share code with the "Byte strings"
                // printing above (which also has to handle pointers to all sorts of things).
                if let Some(GlobalAlloc::Function(instance)) =
                    self.tcx().try_get_global_alloc(prov.alloc_id())
                {
                    self.typed_value(
                        |this| this.print_value_path(instance.def_id(), instance.args),
                        |this| this.print_type(ty),
                        " as ",
                    )?;
                    return Ok(());
                }
            }
            _ => {}
        }
        // Any pointer values not covered by a branch above
        self.pretty_print_const_pointer(ptr, ty)?;
        Ok(())
    }

    fn pretty_print_const_scalar_int(
        &mut self,
        int: ScalarInt,
        ty: Ty<'tcx>,
        print_ty: bool,
    ) -> Result<(), PrintError> {
        define_scoped_cx!(self);

        match ty.kind() {
            // Bool
            ty::Bool if int == ScalarInt::FALSE => p!("false"),
            ty::Bool if int == ScalarInt::TRUE => p!("true"),
            // Float
            ty::Float(ty::FloatTy::F32) => {
                let val = Single::try_from(int).unwrap();
                p!(write("{}{}f32", val, if val.is_finite() { "" } else { "_" }))
            }
            ty::Float(ty::FloatTy::F64) => {
                let val = Double::try_from(int).unwrap();
                p!(write("{}{}f64", val, if val.is_finite() { "" } else { "_" }))
            }
            // Int
            ty::Uint(_) | ty::Int(_) => {
                let int =
                    ConstInt::new(int, matches!(ty.kind(), ty::Int(_)), ty.is_ptr_sized_integral());
                if print_ty { p!(write("{:#?}", int)) } else { p!(write("{:?}", int)) }
            }
            // Char
            ty::Char if char::try_from(int).is_ok() => {
                p!(write("{:?}", char::try_from(int).unwrap()))
            }
            // Pointer types
            ty::Ref(..) | ty::RawPtr(_) | ty::FnPtr(_) => {
                let data = int.assert_bits(self.tcx().data_layout.pointer_size);
                self.typed_value(
                    |this| {
                        write!(this, "0x{data:x}")?;
                        Ok(())
                    },
                    |this| this.print_type(ty),
                    " as ",
                )?;
            }
            // Nontrivial types with scalar bit representation
            _ => {
                let print = |this: &mut Self| {
                    if int.size() == Size::ZERO {
                        write!(this, "transmute(())")?;
                    } else {
                        write!(this, "transmute(0x{int:x})")?;
                    }
                    Ok(())
                };
                if print_ty {
                    self.typed_value(print, |this| this.print_type(ty), ": ")?
                } else {
                    print(self)?
                };
            }
        }
        Ok(())
    }

    /// This is overridden for MIR printing because we only want to hide alloc ids from users, not
    /// from MIR where it is actually useful.
    fn pretty_print_const_pointer<Prov: Provenance>(
        &mut self,
        _: Pointer<Prov>,
        ty: Ty<'tcx>,
    ) -> Result<(), PrintError> {
        self.typed_value(
            |this| {
                this.write_str("&_")?;
                Ok(())
            },
            |this| this.print_type(ty),
            ": ",
        )
    }

    fn pretty_print_byte_str(&mut self, byte_str: &'tcx [u8]) -> Result<(), PrintError> {
        write!(self, "b\"{}\"", byte_str.escape_ascii())?;
        Ok(())
    }

    fn pretty_print_const_valtree(
        &mut self,
        valtree: ty::ValTree<'tcx>,
        ty: Ty<'tcx>,
        print_ty: bool,
    ) -> Result<(), PrintError> {
        define_scoped_cx!(self);

        if self.should_print_verbose() {
            p!(write("ValTree({:?}: ", valtree), print(ty), ")");
            return Ok(());
        }

        let u8_type = self.tcx().types.u8;
        match (valtree, ty.kind()) {
            (ty::ValTree::Branch(_), ty::Ref(_, inner_ty, _)) => match inner_ty.kind() {
                ty::Slice(t) if *t == u8_type => {
                    let bytes = valtree.try_to_raw_bytes(self.tcx(), ty).unwrap_or_else(|| {
                        bug!(
                            "expected to convert valtree {:?} to raw bytes for type {:?}",
                            valtree,
                            t
                        )
                    });
                    return self.pretty_print_byte_str(bytes);
                }
                ty::Str => {
                    let bytes = valtree.try_to_raw_bytes(self.tcx(), ty).unwrap_or_else(|| {
                        bug!("expected to convert valtree to raw bytes for type {:?}", ty)
                    });
                    p!(write("{:?}", String::from_utf8_lossy(bytes)));
                    return Ok(());
                }
                _ => {
                    p!("&");
                    p!(pretty_print_const_valtree(valtree, *inner_ty, print_ty));
                    return Ok(());
                }
            },
            (ty::ValTree::Branch(_), ty::Array(t, _)) if *t == u8_type => {
                let bytes = valtree.try_to_raw_bytes(self.tcx(), ty).unwrap_or_else(|| {
                    bug!("expected to convert valtree to raw bytes for type {:?}", t)
                });
                p!("*");
                p!(pretty_print_byte_str(bytes));
                return Ok(());
            }
            // Aggregates, printed as array/tuple/struct/variant construction syntax.
            (ty::ValTree::Branch(_), ty::Array(..) | ty::Tuple(..) | ty::Adt(..)) => {
                let contents =
                    self.tcx().destructure_const(ty::Const::new_value(self.tcx(), valtree, ty));
                let fields = contents.fields.iter().copied();
                match *ty.kind() {
                    ty::Array(..) => {
                        p!("[", comma_sep(fields), "]");
                    }
                    ty::Tuple(..) => {
                        p!("(", comma_sep(fields));
                        if contents.fields.len() == 1 {
                            p!(",");
                        }
                        p!(")");
                    }
                    ty::Adt(def, _) if def.variants().is_empty() => {
                        self.typed_value(
                            |this| {
                                write!(this, "unreachable()")?;
                                Ok(())
                            },
                            |this| this.print_type(ty),
                            ": ",
                        )?;
                    }
                    ty::Adt(def, args) => {
                        let variant_idx =
                            contents.variant.expect("destructed const of adt without variant idx");
                        let variant_def = &def.variant(variant_idx);
                        p!(print_value_path(variant_def.def_id, args));
                        match variant_def.ctor_kind() {
                            Some(CtorKind::Const) => {}
                            Some(CtorKind::Fn) => {
                                p!("(", comma_sep(fields), ")");
                            }
                            None => {
                                p!(" {{ ");
                                let mut first = true;
                                for (field_def, field) in iter::zip(&variant_def.fields, fields) {
                                    if !first {
                                        p!(", ");
                                    }
                                    p!(write("{}: ", field_def.name), print(field));
                                    first = false;
                                }
                                p!(" }}");
                            }
                        }
                    }
                    _ => unreachable!(),
                }
                return Ok(());
            }
            (ty::ValTree::Leaf(leaf), ty::Ref(_, inner_ty, _)) => {
                p!(write("&"));
                return self.pretty_print_const_scalar_int(leaf, *inner_ty, print_ty);
            }
            (ty::ValTree::Leaf(leaf), _) => {
                return self.pretty_print_const_scalar_int(leaf, ty, print_ty);
            }
            // FIXME(oli-obk): also pretty print arrays and other aggregate constants by reading
            // their fields instead of just dumping the memory.
            _ => {}
        }

        // fallback
        if valtree == ty::ValTree::zst() {
            p!(write("<ZST>"));
        } else {
            p!(write("{:?}", valtree));
        }
        if print_ty {
            p!(": ", print(ty));
        }
        Ok(())
    }

    fn pretty_closure_as_impl(&mut self, closure: ty::ClosureArgs<'tcx>) -> Result<(), PrintError> {
        let sig = closure.sig();
        let kind = closure.kind_ty().to_opt_closure_kind().unwrap_or(ty::ClosureKind::Fn);

        write!(self, "impl ")?;
        self.wrap_binder(&sig, |sig, cx| {
            define_scoped_cx!(cx);

            p!(write("{kind}("));
            for (i, arg) in sig.inputs()[0].tuple_fields().iter().enumerate() {
                if i > 0 {
                    p!(", ");
                }
                p!(print(arg));
            }
            p!(")");

            if !sig.output().is_unit() {
                p!(" -> ", print(sig.output()));
            }

            Ok(())
        })
    }

    fn pretty_print_bound_constness(
        &mut self,
        trait_ref: ty::TraitRef<'tcx>,
    ) -> Result<(), PrintError> {
        define_scoped_cx!(self);

        let Some(idx) = self.tcx().generics_of(trait_ref.def_id).host_effect_index else {
            return Ok(());
        };
        let arg = trait_ref.args.const_at(idx);

        if arg == self.tcx().consts.false_ {
            p!("const ");
        } else if arg != self.tcx().consts.true_ && !arg.has_infer() {
            p!("~const ");
        }
        Ok(())
    }

    fn should_print_verbose(&self) -> bool {
        self.tcx().sess.verbose_internals()
    }
}

pub(crate) fn pretty_print_const<'tcx>(
    c: ty::Const<'tcx>,
    fmt: &mut fmt::Formatter<'_>,
    print_types: bool,
) -> fmt::Result {
    ty::tls::with(|tcx| {
        let literal = tcx.lift(c).unwrap();
        let mut cx = FmtPrinter::new(tcx, Namespace::ValueNS);
        cx.print_alloc_ids = true;
        cx.pretty_print_const(literal, print_types)?;
        fmt.write_str(&cx.into_buffer())?;
        Ok(())
    })
}

// HACK(eddyb) boxed to avoid moving around a large struct by-value.
pub struct FmtPrinter<'a, 'tcx>(Box<FmtPrinterData<'a, 'tcx>>);

pub struct FmtPrinterData<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    fmt: String,

    empty_path: bool,
    in_value: bool,
    pub print_alloc_ids: bool,

    // set of all named (non-anonymous) region names
    used_region_names: FxHashSet<Symbol>,

    region_index: usize,
    binder_depth: usize,
    printed_type_count: usize,
    type_length_limit: Limit,
    truncated: bool,

    pub region_highlight_mode: RegionHighlightMode<'tcx>,

    pub ty_infer_name_resolver: Option<Box<dyn Fn(ty::TyVid) -> Option<Symbol> + 'a>>,
    pub const_infer_name_resolver: Option<Box<dyn Fn(ty::ConstVid) -> Option<Symbol> + 'a>>,
}

impl<'a, 'tcx> Deref for FmtPrinter<'a, 'tcx> {
    type Target = FmtPrinterData<'a, 'tcx>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FmtPrinter<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, 'tcx> FmtPrinter<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, ns: Namespace) -> Self {
        let limit =
            if with_reduced_queries() { Limit::new(1048576) } else { tcx.type_length_limit() };
        Self::new_with_limit(tcx, ns, limit)
    }

    pub fn print_string(
        tcx: TyCtxt<'tcx>,
        ns: Namespace,
        f: impl FnOnce(&mut Self) -> Result<(), PrintError>,
    ) -> Result<String, PrintError> {
        let mut c = FmtPrinter::new(tcx, ns);
        f(&mut c)?;
        Ok(c.into_buffer())
    }

    pub fn new_with_limit(tcx: TyCtxt<'tcx>, ns: Namespace, type_length_limit: Limit) -> Self {
        FmtPrinter(Box::new(FmtPrinterData {
            tcx,
            // Estimated reasonable capacity to allocate upfront based on a few
            // benchmarks.
            fmt: String::with_capacity(64),
            empty_path: false,
            in_value: ns == Namespace::ValueNS,
            print_alloc_ids: false,
            used_region_names: Default::default(),
            region_index: 0,
            binder_depth: 0,
            printed_type_count: 0,
            type_length_limit,
            truncated: false,
            region_highlight_mode: RegionHighlightMode::default(),
            ty_infer_name_resolver: None,
            const_infer_name_resolver: None,
        }))
    }

    pub fn into_buffer(self) -> String {
        self.0.fmt
    }
}

// HACK(eddyb) get rid of `def_path_str` and/or pass `Namespace` explicitly always
// (but also some things just print a `DefId` generally so maybe we need this?)
fn guess_def_namespace(tcx: TyCtxt<'_>, def_id: DefId) -> Namespace {
    match tcx.def_key(def_id).disambiguated_data.data {
        DefPathData::TypeNs(..) | DefPathData::CrateRoot | DefPathData::OpaqueTy => {
            Namespace::TypeNS
        }

        DefPathData::ValueNs(..)
        | DefPathData::AnonConst
        | DefPathData::Closure
        | DefPathData::Ctor => Namespace::ValueNS,

        DefPathData::MacroNs(..) => Namespace::MacroNS,

        _ => Namespace::TypeNS,
    }
}

impl<'t> TyCtxt<'t> {
    /// Returns a string identifying this `DefId`. This string is
    /// suitable for user output.
    pub fn def_path_str(self, def_id: impl IntoQueryParam<DefId>) -> String {
        self.def_path_str_with_args(def_id, &[])
    }

    pub fn def_path_str_with_args(
        self,
        def_id: impl IntoQueryParam<DefId>,
        args: &'t [GenericArg<'t>],
    ) -> String {
        let def_id = def_id.into_query_param();
        let ns = guess_def_namespace(self, def_id);
        debug!("def_path_str: def_id={:?}, ns={:?}", def_id, ns);

        FmtPrinter::print_string(self, ns, |cx| cx.print_def_path(def_id, args)).unwrap()
    }

    pub fn value_path_str_with_args(
        self,
        def_id: impl IntoQueryParam<DefId>,
        args: &'t [GenericArg<'t>],
    ) -> String {
        let def_id = def_id.into_query_param();
        let ns = guess_def_namespace(self, def_id);
        debug!("value_path_str: def_id={:?}, ns={:?}", def_id, ns);

        FmtPrinter::print_string(self, ns, |cx| cx.print_value_path(def_id, args)).unwrap()
    }
}

impl fmt::Write for FmtPrinter<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.fmt.push_str(s);
        Ok(())
    }
}

impl<'tcx> Printer<'tcx> for FmtPrinter<'_, 'tcx> {
    fn tcx<'a>(&'a self) -> TyCtxt<'tcx> {
        self.tcx
    }

    fn print_def_path(
        &mut self,
        def_id: DefId,
        args: &'tcx [GenericArg<'tcx>],
    ) -> Result<(), PrintError> {
        if args.is_empty() {
            match self.try_print_trimmed_def_path(def_id)? {
                true => return Ok(()),
                false => {}
            }

            match self.try_print_visible_def_path(def_id)? {
                true => return Ok(()),
                false => {}
            }
        }

        let key = self.tcx.def_key(def_id);
        if let DefPathData::Impl = key.disambiguated_data.data {
            // Always use types for non-local impls, where types are always
            // available, and filename/line-number is mostly uninteresting.
            let use_types = !def_id.is_local() || {
                // Otherwise, use filename/line-number if forced.
                let force_no_types = with_forced_impl_filename_line();
                !force_no_types
            };

            if !use_types {
                // If no type info is available, fall back to
                // pretty printing some span information. This should
                // only occur very early in the compiler pipeline.
                let parent_def_id = DefId { index: key.parent.unwrap(), ..def_id };
                let span = self.tcx.def_span(def_id);

                self.print_def_path(parent_def_id, &[])?;

                // HACK(eddyb) copy of `path_append` to avoid
                // constructing a `DisambiguatedDefPathData`.
                if !self.empty_path {
                    write!(self, "::")?;
                }
                write!(
                    self,
                    "<impl at {}>",
                    // This may end up in stderr diagnostics but it may also be emitted
                    // into MIR. Hence we use the remapped path if available
                    self.tcx.sess.source_map().span_to_embeddable_string(span)
                )?;
                self.empty_path = false;

                return Ok(());
            }
        }

        self.default_print_def_path(def_id, args)
    }

    fn print_region(&mut self, region: ty::Region<'tcx>) -> Result<(), PrintError> {
        self.pretty_print_region(region)
    }

    fn print_type(&mut self, ty: Ty<'tcx>) -> Result<(), PrintError> {
        if self.type_length_limit.value_within_limit(self.printed_type_count) {
            self.printed_type_count += 1;
            self.pretty_print_type(ty)
        } else {
            self.truncated = true;
            write!(self, "...")?;
            Ok(())
        }
    }

    fn print_dyn_existential(
        &mut self,
        predicates: &'tcx ty::List<ty::PolyExistentialPredicate<'tcx>>,
    ) -> Result<(), PrintError> {
        self.pretty_print_dyn_existential(predicates)
    }

    fn print_const(&mut self, ct: ty::Const<'tcx>) -> Result<(), PrintError> {
        self.pretty_print_const(ct, false)
    }

    fn path_crate(&mut self, cnum: CrateNum) -> Result<(), PrintError> {
        self.empty_path = true;
        if cnum == LOCAL_CRATE {
            if self.tcx.sess.at_least_rust_2018() {
                // We add the `crate::` keyword on Rust 2018, only when desired.
                if with_crate_prefix() {
                    write!(self, "{}", kw::Crate)?;
                    self.empty_path = false;
                }
            }
        } else {
            write!(self, "{}", self.tcx.crate_name(cnum))?;
            self.empty_path = false;
        }
        Ok(())
    }

    fn path_qualified(
        &mut self,
        self_ty: Ty<'tcx>,
        trait_ref: Option<ty::TraitRef<'tcx>>,
    ) -> Result<(), PrintError> {
        self.pretty_path_qualified(self_ty, trait_ref)?;
        self.empty_path = false;
        Ok(())
    }

    fn path_append_impl(
        &mut self,
        print_prefix: impl FnOnce(&mut Self) -> Result<(), PrintError>,
        _disambiguated_data: &DisambiguatedDefPathData,
        self_ty: Ty<'tcx>,
        trait_ref: Option<ty::TraitRef<'tcx>>,
    ) -> Result<(), PrintError> {
        self.pretty_path_append_impl(
            |cx| {
                print_prefix(cx)?;
                if !cx.empty_path {
                    write!(cx, "::")?;
                }

                Ok(())
            },
            self_ty,
            trait_ref,
        )?;
        self.empty_path = false;
        Ok(())
    }

    fn path_append(
        &mut self,
        print_prefix: impl FnOnce(&mut Self) -> Result<(), PrintError>,
        disambiguated_data: &DisambiguatedDefPathData,
    ) -> Result<(), PrintError> {
        print_prefix(self)?;

        // Skip `::{{extern}}` blocks and `::{{constructor}}` on tuple/unit structs.
        if let DefPathData::ForeignMod | DefPathData::Ctor = disambiguated_data.data {
            return Ok(());
        }

        let name = disambiguated_data.data.name();
        if !self.empty_path {
            write!(self, "::")?;
        }

        if let DefPathDataName::Named(name) = name {
            if Ident::with_dummy_span(name).is_raw_guess() {
                write!(self, "r#")?;
            }
        }

        let verbose = self.should_print_verbose();
        disambiguated_data.fmt_maybe_verbose(self, verbose)?;

        self.empty_path = false;

        Ok(())
    }

    fn path_generic_args(
        &mut self,
        print_prefix: impl FnOnce(&mut Self) -> Result<(), PrintError>,
        args: &[GenericArg<'tcx>],
    ) -> Result<(), PrintError> {
        print_prefix(self)?;

        if !args.is_empty() {
            if self.in_value {
                write!(self, "::")?;
            }
            self.generic_delimiters(|cx| cx.comma_sep(args.iter().copied()))
        } else {
            Ok(())
        }
    }
}

impl<'tcx> PrettyPrinter<'tcx> for FmtPrinter<'_, 'tcx> {
    fn ty_infer_name(&self, id: ty::TyVid) -> Option<Symbol> {
        self.0.ty_infer_name_resolver.as_ref().and_then(|func| func(id))
    }

    fn reset_type_limit(&mut self) {
        self.printed_type_count = 0;
    }

    fn const_infer_name(&self, id: ty::ConstVid) -> Option<Symbol> {
        self.0.const_infer_name_resolver.as_ref().and_then(|func| func(id))
    }

    fn print_value_path(
        &mut self,
        def_id: DefId,
        args: &'tcx [GenericArg<'tcx>],
    ) -> Result<(), PrintError> {
        let was_in_value = std::mem::replace(&mut self.in_value, true);
        self.print_def_path(def_id, args)?;
        self.in_value = was_in_value;

        Ok(())
    }

    fn in_binder<T>(&mut self, value: &ty::Binder<'tcx, T>) -> Result<(), PrintError>
    where
        T: Print<'tcx, Self> + TypeFoldable<TyCtxt<'tcx>>,
    {
        self.pretty_in_binder(value)
    }

    fn wrap_binder<T, C: FnOnce(&T, &mut Self) -> Result<(), PrintError>>(
        &mut self,
        value: &ty::Binder<'tcx, T>,
        f: C,
    ) -> Result<(), PrintError>
    where
        T: Print<'tcx, Self> + TypeFoldable<TyCtxt<'tcx>>,
    {
        self.pretty_wrap_binder(value, f)
    }

    fn typed_value(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), PrintError>,
        t: impl FnOnce(&mut Self) -> Result<(), PrintError>,
        conversion: &str,
    ) -> Result<(), PrintError> {
        self.write_str("{")?;
        f(self)?;
        self.write_str(conversion)?;
        let was_in_value = std::mem::replace(&mut self.in_value, false);
        t(self)?;
        self.in_value = was_in_value;
        self.write_str("}")?;
        Ok(())
    }

    fn generic_delimiters(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), PrintError>,
    ) -> Result<(), PrintError> {
        write!(self, "<")?;

        let was_in_value = std::mem::replace(&mut self.in_value, false);
        f(self)?;
        self.in_value = was_in_value;

        write!(self, ">")?;
        Ok(())
    }

    fn should_print_region(&self, region: ty::Region<'tcx>) -> bool {
        let highlight = self.region_highlight_mode;
        if highlight.region_highlighted(region).is_some() {
            return true;
        }

        if self.should_print_verbose() {
            return true;
        }

        if with_forced_trimmed_paths() {
            return false;
        }

        let identify_regions = self.tcx.sess.opts.unstable_opts.identify_regions;

        match *region {
            ty::ReEarlyParam(ref data) => data.has_name(),

            ty::ReBound(_, ty::BoundRegion { kind: br, .. })
            | ty::ReLateParam(ty::LateParamRegion { bound_region: br, .. })
            | ty::RePlaceholder(ty::Placeholder {
                bound: ty::BoundRegion { kind: br, .. }, ..
            }) => {
                if br.is_named() {
                    return true;
                }

                if let Some((region, _)) = highlight.highlight_bound_region {
                    if br == region {
                        return true;
                    }
                }

                false
            }

            ty::ReVar(_) if identify_regions => true,

            ty::ReVar(_) | ty::ReErased | ty::ReError(_) => false,

            ty::ReStatic => true,
        }
    }

    fn pretty_print_const_pointer<Prov: Provenance>(
        &mut self,
        p: Pointer<Prov>,
        ty: Ty<'tcx>,
    ) -> Result<(), PrintError> {
        let print = |this: &mut Self| {
            define_scoped_cx!(this);
            if this.print_alloc_ids {
                p!(write("{:?}", p));
            } else {
                p!("&_");
            }
            Ok(())
        };
        self.typed_value(print, |this| this.print_type(ty), ": ")
    }
}

// HACK(eddyb) limited to `FmtPrinter` because of `region_highlight_mode`.
impl<'tcx> FmtPrinter<'_, 'tcx> {
    pub fn pretty_print_region(&mut self, region: ty::Region<'tcx>) -> Result<(), fmt::Error> {
        define_scoped_cx!(self);

        // Watch out for region highlights.
        let highlight = self.region_highlight_mode;
        if let Some(n) = highlight.region_highlighted(region) {
            p!(write("'{}", n));
            return Ok(());
        }

        if self.should_print_verbose() {
            p!(write("{:?}", region));
            return Ok(());
        }

        let identify_regions = self.tcx.sess.opts.unstable_opts.identify_regions;

        // These printouts are concise. They do not contain all the information
        // the user might want to diagnose an error, but there is basically no way
        // to fit that into a short string. Hence the recommendation to use
        // `explain_region()` or `note_and_explain_region()`.
        match *region {
            ty::ReEarlyParam(ref data) => {
                if data.name != kw::Empty {
                    p!(write("{}", data.name));
                    return Ok(());
                }
            }
            ty::ReBound(_, ty::BoundRegion { kind: br, .. })
            | ty::ReLateParam(ty::LateParamRegion { bound_region: br, .. })
            | ty::RePlaceholder(ty::Placeholder {
                bound: ty::BoundRegion { kind: br, .. }, ..
            }) => {
                if let ty::BrNamed(_, name) = br
                    && br.is_named()
                {
                    p!(write("{}", name));
                    return Ok(());
                }

                if let Some((region, counter)) = highlight.highlight_bound_region {
                    if br == region {
                        p!(write("'{}", counter));
                        return Ok(());
                    }
                }
            }
            ty::ReVar(region_vid) if identify_regions => {
                p!(write("{:?}", region_vid));
                return Ok(());
            }
            ty::ReVar(_) => {}
            ty::ReErased => {}
            ty::ReError(_) => {}
            ty::ReStatic => {
                p!("'static");
                return Ok(());
            }
        }

        p!("'_");

        Ok(())
    }
}

/// Folds through bound vars and placeholders, naming them
struct RegionFolder<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    current_index: ty::DebruijnIndex,
    region_map: BTreeMap<ty::BoundRegion, ty::Region<'tcx>>,
    name: &'a mut (
                dyn FnMut(
        Option<ty::DebruijnIndex>, // Debruijn index of the folded late-bound region
        ty::DebruijnIndex,         // Index corresponding to binder level
        ty::BoundRegion,
    ) -> ty::Region<'tcx>
                    + 'a
            ),
}

impl<'a, 'tcx> ty::TypeFolder<TyCtxt<'tcx>> for RegionFolder<'a, 'tcx> {
    fn interner(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    fn fold_binder<T: TypeFoldable<TyCtxt<'tcx>>>(
        &mut self,
        t: ty::Binder<'tcx, T>,
    ) -> ty::Binder<'tcx, T> {
        self.current_index.shift_in(1);
        let t = t.super_fold_with(self);
        self.current_index.shift_out(1);
        t
    }

    fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx> {
        match *t.kind() {
            _ if t.has_vars_bound_at_or_above(self.current_index) || t.has_placeholders() => {
                return t.super_fold_with(self);
            }
            _ => {}
        }
        t
    }

    fn fold_region(&mut self, r: ty::Region<'tcx>) -> ty::Region<'tcx> {
        let name = &mut self.name;
        let region = match *r {
            ty::ReBound(db, br) if db >= self.current_index => {
                *self.region_map.entry(br).or_insert_with(|| name(Some(db), self.current_index, br))
            }
            ty::RePlaceholder(ty::PlaceholderRegion {
                bound: ty::BoundRegion { kind, .. },
                ..
            }) => {
                // If this is an anonymous placeholder, don't rename. Otherwise, in some
                // async fns, we get a `for<'r> Send` bound
                match kind {
                    ty::BrAnon | ty::BrEnv => r,
                    _ => {
                        // Index doesn't matter, since this is just for naming and these never get bound
                        let br = ty::BoundRegion { var: ty::BoundVar::from_u32(0), kind };
                        *self
                            .region_map
                            .entry(br)
                            .or_insert_with(|| name(None, self.current_index, br))
                    }
                }
            }
            _ => return r,
        };
        if let ty::ReBound(debruijn1, br) = *region {
            assert_eq!(debruijn1, ty::INNERMOST);
            ty::Region::new_bound(self.tcx, self.current_index, br)
        } else {
            region
        }
    }
}

// HACK(eddyb) limited to `FmtPrinter` because of `binder_depth`,
// `region_index` and `used_region_names`.
impl<'tcx> FmtPrinter<'_, 'tcx> {
    pub fn name_all_regions<T>(
        &mut self,
        value: &ty::Binder<'tcx, T>,
    ) -> Result<(T, BTreeMap<ty::BoundRegion, ty::Region<'tcx>>), fmt::Error>
    where
        T: Print<'tcx, Self> + TypeFoldable<TyCtxt<'tcx>>,
    {
        fn name_by_region_index(
            index: usize,
            available_names: &mut Vec<Symbol>,
            num_available: usize,
        ) -> Symbol {
            if let Some(name) = available_names.pop() {
                name
            } else {
                Symbol::intern(&format!("'z{}", index - num_available))
            }
        }

        debug!("name_all_regions");

        // Replace any anonymous late-bound regions with named
        // variants, using new unique identifiers, so that we can
        // clearly differentiate between named and unnamed regions in
        // the output. We'll probably want to tweak this over time to
        // decide just how much information to give.
        if self.binder_depth == 0 {
            self.prepare_region_info(value);
        }

        debug!("self.used_region_names: {:?}", &self.used_region_names);

        let mut empty = true;
        let mut start_or_continue = |cx: &mut Self, start: &str, cont: &str| {
            let w = if empty {
                empty = false;
                start
            } else {
                cont
            };
            let _ = write!(cx, "{w}");
        };
        let do_continue = |cx: &mut Self, cont: Symbol| {
            let _ = write!(cx, "{cont}");
        };

        let possible_names = ('a'..='z').rev().map(|s| Symbol::intern(&format!("'{s}")));

        let mut available_names = possible_names
            .filter(|name| !self.used_region_names.contains(name))
            .collect::<Vec<_>>();
        debug!(?available_names);
        let num_available = available_names.len();

        let mut region_index = self.region_index;
        let mut next_name = |this: &Self| {
            let mut name;

            loop {
                name = name_by_region_index(region_index, &mut available_names, num_available);
                region_index += 1;

                if !this.used_region_names.contains(&name) {
                    break;
                }
            }

            name
        };

        // If we want to print verbosely, then print *all* binders, even if they
        // aren't named. Eventually, we might just want this as the default, but
        // this is not *quite* right and changes the ordering of some output
        // anyways.
        let (new_value, map) = if self.should_print_verbose() {
            for var in value.bound_vars().iter() {
                start_or_continue(self, "for<", ", ");
                write!(self, "{var:?}")?;
            }
            start_or_continue(self, "", "> ");
            (value.clone().skip_binder(), BTreeMap::default())
        } else {
            let tcx = self.tcx;

            let trim_path = with_forced_trimmed_paths();
            // Closure used in `RegionFolder` to create names for anonymous late-bound
            // regions. We use two `DebruijnIndex`es (one for the currently folded
            // late-bound region and the other for the binder level) to determine
            // whether a name has already been created for the currently folded region,
            // see issue #102392.
            let mut name = |lifetime_idx: Option<ty::DebruijnIndex>,
                            binder_level_idx: ty::DebruijnIndex,
                            br: ty::BoundRegion| {
                let (name, kind) = match br.kind {
                    ty::BrAnon | ty::BrEnv => {
                        let name = next_name(self);

                        if let Some(lt_idx) = lifetime_idx {
                            if lt_idx > binder_level_idx {
                                let kind = ty::BrNamed(CRATE_DEF_ID.to_def_id(), name);
                                return ty::Region::new_bound(
                                    tcx,
                                    ty::INNERMOST,
                                    ty::BoundRegion { var: br.var, kind },
                                );
                            }
                        }

                        (name, ty::BrNamed(CRATE_DEF_ID.to_def_id(), name))
                    }
                    ty::BrNamed(def_id, kw::UnderscoreLifetime | kw::Empty) => {
                        let name = next_name(self);

                        if let Some(lt_idx) = lifetime_idx {
                            if lt_idx > binder_level_idx {
                                let kind = ty::BrNamed(def_id, name);
                                return ty::Region::new_bound(
                                    tcx,
                                    ty::INNERMOST,
                                    ty::BoundRegion { var: br.var, kind },
                                );
                            }
                        }

                        (name, ty::BrNamed(def_id, name))
                    }
                    ty::BrNamed(_, name) => {
                        if let Some(lt_idx) = lifetime_idx {
                            if lt_idx > binder_level_idx {
                                let kind = br.kind;
                                return ty::Region::new_bound(
                                    tcx,
                                    ty::INNERMOST,
                                    ty::BoundRegion { var: br.var, kind },
                                );
                            }
                        }

                        (name, br.kind)
                    }
                };

                if !trim_path {
                    start_or_continue(self, "for<", ", ");
                    do_continue(self, name);
                }
                ty::Region::new_bound(tcx, ty::INNERMOST, ty::BoundRegion { var: br.var, kind })
            };
            let mut folder = RegionFolder {
                tcx,
                current_index: ty::INNERMOST,
                name: &mut name,
                region_map: BTreeMap::new(),
            };
            let new_value = value.clone().skip_binder().fold_with(&mut folder);
            let region_map = folder.region_map;
            if !trim_path {
                start_or_continue(self, "", "> ");
            }
            (new_value, region_map)
        };

        self.binder_depth += 1;
        self.region_index = region_index;
        Ok((new_value, map))
    }

    pub fn pretty_in_binder<T>(&mut self, value: &ty::Binder<'tcx, T>) -> Result<(), fmt::Error>
    where
        T: Print<'tcx, Self> + TypeFoldable<TyCtxt<'tcx>>,
    {
        let old_region_index = self.region_index;
        let (new_value, _) = self.name_all_regions(value)?;
        new_value.print(self)?;
        self.region_index = old_region_index;
        self.binder_depth -= 1;
        Ok(())
    }

    pub fn pretty_wrap_binder<T, C: FnOnce(&T, &mut Self) -> Result<(), fmt::Error>>(
        &mut self,
        value: &ty::Binder<'tcx, T>,
        f: C,
    ) -> Result<(), fmt::Error>
    where
        T: Print<'tcx, Self> + TypeFoldable<TyCtxt<'tcx>>,
    {
        let old_region_index = self.region_index;
        let (new_value, _) = self.name_all_regions(value)?;
        f(&new_value, self)?;
        self.region_index = old_region_index;
        self.binder_depth -= 1;
        Ok(())
    }

    fn prepare_region_info<T>(&mut self, value: &ty::Binder<'tcx, T>)
    where
        T: TypeVisitable<TyCtxt<'tcx>>,
    {
        struct RegionNameCollector<'tcx> {
            used_region_names: FxHashSet<Symbol>,
            type_collector: SsoHashSet<Ty<'tcx>>,
        }

        impl<'tcx> RegionNameCollector<'tcx> {
            fn new() -> Self {
                RegionNameCollector {
                    used_region_names: Default::default(),
                    type_collector: SsoHashSet::new(),
                }
            }
        }

        impl<'tcx> ty::visit::TypeVisitor<TyCtxt<'tcx>> for RegionNameCollector<'tcx> {
            fn visit_region(&mut self, r: ty::Region<'tcx>) {
                trace!("address: {:p}", r.0.0);

                // Collect all named lifetimes. These allow us to prevent duplication
                // of already existing lifetime names when introducing names for
                // anonymous late-bound regions.
                if let Some(name) = r.get_name() {
                    self.used_region_names.insert(name);
                }
            }

            // We collect types in order to prevent really large types from compiling for
            // a really long time. See issue #83150 for why this is necessary.
            fn visit_ty(&mut self, ty: Ty<'tcx>) {
                let not_previously_inserted = self.type_collector.insert(ty);
                if not_previously_inserted {
                    ty.super_visit_with(self)
                }
            }
        }

        let mut collector = RegionNameCollector::new();
        value.visit_with(&mut collector);
        self.used_region_names = collector.used_region_names;
        self.region_index = 0;
    }
}

impl<'tcx, T, P: PrettyPrinter<'tcx>> Print<'tcx, P> for ty::Binder<'tcx, T>
where
    T: Print<'tcx, P> + TypeFoldable<TyCtxt<'tcx>>,
{
    fn print(&self, cx: &mut P) -> Result<(), PrintError> {
        cx.in_binder(self)
    }
}

impl<'tcx, T, U, P: PrettyPrinter<'tcx>> Print<'tcx, P> for ty::OutlivesPredicate<T, U>
where
    T: Print<'tcx, P>,
    U: Print<'tcx, P>,
{
    fn print(&self, cx: &mut P) -> Result<(), PrintError> {
        define_scoped_cx!(cx);
        p!(print(self.0), ": ", print(self.1));
        Ok(())
    }
}

/// Wrapper type for `ty::TraitRef` which opts-in to pretty printing only
/// the trait path. That is, it will print `Trait<U>` instead of
/// `<T as Trait<U>>`.
#[derive(Copy, Clone, TypeFoldable, TypeVisitable, Lift)]
pub struct TraitRefPrintOnlyTraitPath<'tcx>(ty::TraitRef<'tcx>);

impl<'tcx> rustc_errors::IntoDiagArg for TraitRefPrintOnlyTraitPath<'tcx> {
    fn into_diag_arg(self) -> rustc_errors::DiagArgValue {
        self.to_string().into_diag_arg()
    }
}

impl<'tcx> fmt::Debug for TraitRefPrintOnlyTraitPath<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Wrapper type for `ty::TraitRef` which opts-in to pretty printing only
/// the trait path, and additionally tries to "sugar" `Fn(...)` trait bounds.
#[derive(Copy, Clone, TypeFoldable, TypeVisitable, Lift)]
pub struct TraitRefPrintSugared<'tcx>(ty::TraitRef<'tcx>);

impl<'tcx> rustc_errors::IntoDiagArg for TraitRefPrintSugared<'tcx> {
    fn into_diag_arg(self) -> rustc_errors::DiagArgValue {
        self.to_string().into_diag_arg()
    }
}

impl<'tcx> fmt::Debug for TraitRefPrintSugared<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Wrapper type for `ty::TraitRef` which opts-in to pretty printing only
/// the trait name. That is, it will print `Trait` instead of
/// `<T as Trait<U>>`.
#[derive(Copy, Clone, TypeFoldable, TypeVisitable, Lift)]
pub struct TraitRefPrintOnlyTraitName<'tcx>(ty::TraitRef<'tcx>);

impl<'tcx> fmt::Debug for TraitRefPrintOnlyTraitName<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<'tcx> ty::TraitRef<'tcx> {
    pub fn print_only_trait_path(self) -> TraitRefPrintOnlyTraitPath<'tcx> {
        TraitRefPrintOnlyTraitPath(self)
    }

    pub fn print_trait_sugared(self) -> TraitRefPrintSugared<'tcx> {
        TraitRefPrintSugared(self)
    }

    pub fn print_only_trait_name(self) -> TraitRefPrintOnlyTraitName<'tcx> {
        TraitRefPrintOnlyTraitName(self)
    }
}

impl<'tcx> ty::Binder<'tcx, ty::TraitRef<'tcx>> {
    pub fn print_only_trait_path(self) -> ty::Binder<'tcx, TraitRefPrintOnlyTraitPath<'tcx>> {
        self.map_bound(|tr| tr.print_only_trait_path())
    }

    pub fn print_trait_sugared(self) -> ty::Binder<'tcx, TraitRefPrintSugared<'tcx>> {
        self.map_bound(|tr| tr.print_trait_sugared())
    }
}

#[derive(Copy, Clone, TypeFoldable, TypeVisitable, Lift)]
pub struct TraitPredPrintModifiersAndPath<'tcx>(ty::TraitPredicate<'tcx>);

impl<'tcx> fmt::Debug for TraitPredPrintModifiersAndPath<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<'tcx> ty::TraitPredicate<'tcx> {
    pub fn print_modifiers_and_trait_path(self) -> TraitPredPrintModifiersAndPath<'tcx> {
        TraitPredPrintModifiersAndPath(self)
    }
}

impl<'tcx> ty::PolyTraitPredicate<'tcx> {
    pub fn print_modifiers_and_trait_path(
        self,
    ) -> ty::Binder<'tcx, TraitPredPrintModifiersAndPath<'tcx>> {
        self.map_bound(TraitPredPrintModifiersAndPath)
    }
}

#[derive(Debug, Copy, Clone, Lift)]
pub struct PrintClosureAsImpl<'tcx> {
    pub closure: ty::ClosureArgs<'tcx>,
}

macro_rules! forward_display_to_print {
    ($($ty:ty),+) => {
        // Some of the $ty arguments may not actually use 'tcx
        $(#[allow(unused_lifetimes)] impl<'tcx> fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                ty::tls::with(|tcx| {
                    let mut cx = FmtPrinter::new(tcx, Namespace::TypeNS);
                    tcx.lift(*self)
                        .expect("could not lift for printing")
                        .print(&mut cx)?;
                    f.write_str(&cx.into_buffer())?;
                    Ok(())
                })
            }
        })+
    };
}

macro_rules! define_print {
    (($self:ident, $cx:ident): $($ty:ty $print:block)+) => {
        $(impl<'tcx, P: PrettyPrinter<'tcx>> Print<'tcx, P> for $ty {
            fn print(&$self, $cx: &mut P) -> Result<(), PrintError> {
                define_scoped_cx!($cx);
                let _: () = $print;
                Ok(())
            }
        })+
    };
}

macro_rules! define_print_and_forward_display {
    (($self:ident, $cx:ident): $($ty:ty $print:block)+) => {
        define_print!(($self, $cx): $($ty $print)*);
        forward_display_to_print!($($ty),+);
    };
}

forward_display_to_print! {
    ty::Region<'tcx>,
    Ty<'tcx>,
    &'tcx ty::List<ty::PolyExistentialPredicate<'tcx>>,
    ty::Const<'tcx>,

    // HACK(eddyb) these are exhaustive instead of generic,
    // because `for<'tcx>` isn't possible yet.
    ty::PolyExistentialProjection<'tcx>,
    ty::PolyExistentialTraitRef<'tcx>,
    ty::Binder<'tcx, ty::TraitRef<'tcx>>,
    ty::Binder<'tcx, TraitRefPrintOnlyTraitPath<'tcx>>,
    ty::Binder<'tcx, TraitRefPrintSugared<'tcx>>,
    ty::Binder<'tcx, ty::FnSig<'tcx>>,
    ty::Binder<'tcx, ty::TraitPredicate<'tcx>>,
    ty::Binder<'tcx, TraitPredPrintModifiersAndPath<'tcx>>,
    ty::Binder<'tcx, ty::ProjectionPredicate<'tcx>>,
    ty::OutlivesPredicate<Ty<'tcx>, ty::Region<'tcx>>,
    ty::OutlivesPredicate<ty::Region<'tcx>, ty::Region<'tcx>>
}

define_print! {
    (self, cx):

    ty::TypeAndMut<'tcx> {
        p!(write("{}", self.mutbl.prefix_str()), print(self.ty))
    }

    ty::ClauseKind<'tcx> {
        match *self {
            ty::ClauseKind::Trait(ref data) => {
                p!(print(data))
            }
            ty::ClauseKind::RegionOutlives(predicate) => p!(print(predicate)),
            ty::ClauseKind::TypeOutlives(predicate) => p!(print(predicate)),
            ty::ClauseKind::Projection(predicate) => p!(print(predicate)),
            ty::ClauseKind::ConstArgHasType(ct, ty) => {
                p!("the constant `", print(ct), "` has type `", print(ty), "`")
            },
            ty::ClauseKind::WellFormed(arg) => p!(print(arg), " well-formed"),
            ty::ClauseKind::ConstEvaluatable(ct) => {
                p!("the constant `", print(ct), "` can be evaluated")
            }
        }
    }

    ty::PredicateKind<'tcx> {
        match *self {
            ty::PredicateKind::Clause(data) => {
                p!(print(data))
            }
            ty::PredicateKind::Subtype(predicate) => p!(print(predicate)),
            ty::PredicateKind::Coerce(predicate) => p!(print(predicate)),
            ty::PredicateKind::ObjectSafe(trait_def_id) => {
                p!("the trait `", print_def_path(trait_def_id, &[]), "` is object-safe")
            }
            ty::PredicateKind::ConstEquate(c1, c2) => {
                p!("the constant `", print(c1), "` equals `", print(c2), "`")
            }
            ty::PredicateKind::Ambiguous => p!("ambiguous"),
            ty::PredicateKind::NormalizesTo(data) => p!(print(data)),
            ty::PredicateKind::AliasRelate(t1, t2, dir) => p!(print(t1), write(" {} ", dir), print(t2)),
        }
    }
}

define_print_and_forward_display! {
    (self, cx):

    &'tcx ty::List<Ty<'tcx>> {
        p!("{{", comma_sep(self.iter()), "}}")
    }

    ty::ExistentialTraitRef<'tcx> {
        // Use a type that can't appear in defaults of type parameters.
        let dummy_self = Ty::new_fresh(cx.tcx(), 0);
        let trait_ref = self.with_self_ty(cx.tcx(), dummy_self);
        p!(print(trait_ref.print_only_trait_path()))
    }

    ty::ExistentialProjection<'tcx> {
        let name = cx.tcx().associated_item(self.def_id).name;
        p!(write("{} = ", name), print(self.term))
    }

    ty::ExistentialPredicate<'tcx> {
        match *self {
            ty::ExistentialPredicate::Trait(x) => p!(print(x)),
            ty::ExistentialPredicate::Projection(x) => p!(print(x)),
            ty::ExistentialPredicate::AutoTrait(def_id) => {
                p!(print_def_path(def_id, &[]));
            }
        }
    }

    ty::FnSig<'tcx> {
        p!(write("{}", self.unsafety.prefix_str()));

        if self.abi != Abi::Rust {
            p!(write("extern {} ", self.abi));
        }

        p!("fn", pretty_fn_sig(self.inputs(), self.c_variadic, self.output()));
    }

    ty::TraitRef<'tcx> {
        p!(write("<{} as {}>", self.self_ty(), self.print_only_trait_path()))
    }

    TraitRefPrintOnlyTraitPath<'tcx> {
        p!(print_def_path(self.0.def_id, self.0.args));
    }

    TraitRefPrintSugared<'tcx> {
        if !with_reduced_queries()
            && let Some(kind) = cx.tcx().fn_trait_kind_from_def_id(self.0.def_id)
            && let ty::Tuple(args) = self.0.args.type_at(1).kind()
        {
            p!(write("{}", kind.as_str()), "(");
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    p!(", ");
                }
                p!(print(arg));
            }
            p!(")");
        } else {
            p!(print_def_path(self.0.def_id, self.0.args));
        }
    }

    TraitRefPrintOnlyTraitName<'tcx> {
        p!(print_def_path(self.0.def_id, &[]));
    }

    TraitPredPrintModifiersAndPath<'tcx> {
        p!(pretty_print_bound_constness(self.0.trait_ref));
        if let ty::ImplPolarity::Negative = self.0.polarity {
            p!("!")
        }
        p!(print(self.0.trait_ref.print_only_trait_path()));
    }

    PrintClosureAsImpl<'tcx> {
        p!(pretty_closure_as_impl(self.closure))
    }

    ty::ParamTy {
        p!(write("{}", self.name))
    }

    ty::ParamConst {
        p!(write("{}", self.name))
    }

    ty::SubtypePredicate<'tcx> {
        p!(print(self.a), " <: ");
        cx.reset_type_limit();
        p!(print(self.b))
    }

    ty::CoercePredicate<'tcx> {
        p!(print(self.a), " -> ");
        cx.reset_type_limit();
        p!(print(self.b))
    }

    ty::TraitPredicate<'tcx> {
        p!(print(self.trait_ref.self_ty()), ": ");
        p!(pretty_print_bound_constness(self.trait_ref));
        if let ty::ImplPolarity::Negative = self.polarity {
            p!("!");
        }
        p!(print(self.trait_ref.print_trait_sugared()))
    }

    ty::ProjectionPredicate<'tcx> {
        p!(print(self.projection_ty), " == ");
        cx.reset_type_limit();
        p!(print(self.term))
    }

    ty::NormalizesTo<'tcx> {
        p!(print(self.alias), " normalizes-to ");
        cx.reset_type_limit();
        p!(print(self.term))
    }

    ty::Term<'tcx> {
      match self.unpack() {
        ty::TermKind::Ty(ty) => p!(print(ty)),
        ty::TermKind::Const(c) => p!(print(c)),
      }
    }

    ty::AliasTy<'tcx> {
        if let DefKind::Impl { of_trait: false } = cx.tcx().def_kind(cx.tcx().parent(self.def_id)) {
            p!(pretty_print_inherent_projection(self))
        } else {
            // If we're printing verbosely, or don't want to invoke queries
            // (`is_impl_trait_in_trait`), then fall back to printing the def path.
            // This is likely what you want if you're debugging the compiler anyways.
            if !(cx.should_print_verbose() || with_reduced_queries())
                && cx.tcx().is_impl_trait_in_trait(self.def_id)
            {
                return cx.pretty_print_opaque_impl_type(self.def_id, self.args);
            } else {
                p!(print_def_path(self.def_id, self.args));
            }

        }
    }

    ty::Predicate<'tcx> {
        p!(print(self.kind()))
    }

    ty::Clause<'tcx> {
        p!(print(self.kind()))
    }

    GenericArg<'tcx> {
        match self.unpack() {
            GenericArgKind::Lifetime(lt) => p!(print(lt)),
            GenericArgKind::Type(ty) => p!(print(ty)),
            GenericArgKind::Const(ct) => p!(print(ct)),
        }
    }
}

fn for_each_def(tcx: TyCtxt<'_>, mut collect_fn: impl for<'b> FnMut(&'b Ident, Namespace, DefId)) {
    // Iterate all local crate items no matter where they are defined.
    let hir = tcx.hir();
    for id in hir.items() {
        if matches!(tcx.def_kind(id.owner_id), DefKind::Use) {
            continue;
        }

        let item = hir.item(id);
        if item.ident.name == kw::Empty {
            continue;
        }

        let def_id = item.owner_id.to_def_id();
        let ns = tcx.def_kind(def_id).ns().unwrap_or(Namespace::TypeNS);
        collect_fn(&item.ident, ns, def_id);
    }

    // Now take care of extern crate items.
    let queue = &mut Vec::new();
    let mut seen_defs: DefIdSet = Default::default();

    for &cnum in tcx.crates(()).iter() {
        let def_id = cnum.as_def_id();

        // Ignore crates that are not direct dependencies.
        match tcx.extern_crate(def_id) {
            None => continue,
            Some(extern_crate) => {
                if !extern_crate.is_direct() {
                    continue;
                }
            }
        }

        queue.push(def_id);
    }

    // Iterate external crate defs but be mindful about visibility
    while let Some(def) = queue.pop() {
        for child in tcx.module_children(def).iter() {
            if !child.vis.is_public() {
                continue;
            }

            match child.res {
                def::Res::Def(DefKind::AssocTy, _) => {}
                def::Res::Def(DefKind::TyAlias, _) => {}
                def::Res::Def(defkind, def_id) => {
                    if let Some(ns) = defkind.ns() {
                        collect_fn(&child.ident, ns, def_id);
                    }

                    if matches!(defkind, DefKind::Mod | DefKind::Enum | DefKind::Trait)
                        && seen_defs.insert(def_id)
                    {
                        queue.push(def_id);
                    }
                }
                _ => {}
            }
        }
    }
}

/// The purpose of this function is to collect public symbols names that are unique across all
/// crates in the build. Later, when printing about types we can use those names instead of the
/// full exported path to them.
///
/// So essentially, if a symbol name can only be imported from one place for a type, and as
/// long as it was not glob-imported anywhere in the current crate, we can trim its printed
/// path and print only the name.
///
/// This has wide implications on error messages with types, for example, shortening
/// `std::vec::Vec` to just `Vec`, as long as there is no other `Vec` importable anywhere.
///
/// The implementation uses similar import discovery logic to that of 'use' suggestions.
///
/// See also [`DelayDm`](rustc_error_messages::DelayDm) and [`with_no_trimmed_paths!`].
// this is pub to be able to intra-doc-link it
pub fn trimmed_def_paths(tcx: TyCtxt<'_>, (): ()) -> DefIdMap<Symbol> {
    // Trimming paths is expensive and not optimized, since we expect it to only be used for error
    // reporting. Record the fact that we did it, so we can abort if we later found it was
    // unnecessary.
    //
    // The `rustc_middle::ty::print::with_no_trimmed_paths` wrapper can be used to suppress this
    // checking, in exchange for full paths being formatted.
    tcx.sess.record_trimmed_def_paths();

    // Once constructed, unique namespace+symbol pairs will have a `Some(_)` entry, while
    // non-unique pairs will have a `None` entry.
    let unique_symbols_rev: &mut FxHashMap<(Namespace, Symbol), Option<DefId>> =
        &mut FxHashMap::default();

    for symbol_set in tcx.resolutions(()).glob_map.values() {
        for symbol in symbol_set {
            unique_symbols_rev.insert((Namespace::TypeNS, *symbol), None);
            unique_symbols_rev.insert((Namespace::ValueNS, *symbol), None);
            unique_symbols_rev.insert((Namespace::MacroNS, *symbol), None);
        }
    }

    for_each_def(tcx, |ident, ns, def_id| {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        match unique_symbols_rev.entry((ns, ident.name)) {
            Occupied(mut v) => match v.get() {
                None => {}
                Some(existing) => {
                    if *existing != def_id {
                        v.insert(None);
                    }
                }
            },
            Vacant(v) => {
                v.insert(Some(def_id));
            }
        }
    });

    // Put the symbol from all the unique namespace+symbol pairs into `map`.
    let mut map: DefIdMap<Symbol> = Default::default();
    for ((_, symbol), opt_def_id) in unique_symbols_rev.drain() {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        if let Some(def_id) = opt_def_id {
            match map.entry(def_id) {
                Occupied(mut v) => {
                    // A single DefId can be known under multiple names (e.g.,
                    // with a `pub use ... as ...;`). We need to ensure that the
                    // name placed in this map is chosen deterministically, so
                    // if we find multiple names (`symbol`) resolving to the
                    // same `def_id`, we prefer the lexicographically smallest
                    // name.
                    //
                    // Any stable ordering would be fine here though.
                    if *v.get() != symbol {
                        if v.get().as_str() > symbol.as_str() {
                            v.insert(symbol);
                        }
                    }
                }
                Vacant(v) => {
                    v.insert(symbol);
                }
            }
        }
    }

    map
}

pub fn provide(providers: &mut Providers) {
    *providers = Providers { trimmed_def_paths, ..*providers };
}

#[derive(Default)]
pub struct OpaqueFnEntry<'tcx> {
    // The trait ref is already stored as a key, so just track if we have it as a real predicate
    has_fn_once: bool,
    fn_mut_trait_ref: Option<ty::PolyTraitRef<'tcx>>,
    fn_trait_ref: Option<ty::PolyTraitRef<'tcx>>,
    return_ty: Option<ty::Binder<'tcx, Term<'tcx>>>,
}
