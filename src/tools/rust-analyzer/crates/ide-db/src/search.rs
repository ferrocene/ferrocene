//! Implementation of find-usages functionality.
//!
//! It is based on the standard ide trick: first, we run a fast text search to
//! get a super-set of matches. Then, we confirm each match using precise
//! name resolution.

use std::mem;

use base_db::{salsa::Database, SourceDatabase, SourceDatabaseExt};
use hir::{
    sym, AsAssocItem, DefWithBody, DescendPreference, FileRange, HasAttrs, HasSource, HirFileIdExt,
    InFile, InRealFile, ModuleSource, PathResolution, Semantics, Visibility,
};
use memchr::memmem::Finder;
use once_cell::unsync::Lazy;
use parser::SyntaxKind;
use rustc_hash::FxHashMap;
use span::EditionedFileId;
use syntax::{ast, match_ast, AstNode, AstToken, SyntaxElement, TextRange, TextSize, ToSmolStr};
use triomphe::Arc;

use crate::{
    defs::{Definition, NameClass, NameRefClass},
    traits::{as_trait_assoc_def, convert_to_def_in_trait},
    RootDatabase,
};

#[derive(Debug, Default, Clone)]
pub struct UsageSearchResult {
    pub references: FxHashMap<EditionedFileId, Vec<FileReference>>,
}

impl UsageSearchResult {
    pub fn is_empty(&self) -> bool {
        self.references.is_empty()
    }

    pub fn len(&self) -> usize {
        self.references.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (EditionedFileId, &[FileReference])> + '_ {
        self.references.iter().map(|(&file_id, refs)| (file_id, &**refs))
    }

    pub fn file_ranges(&self) -> impl Iterator<Item = FileRange> + '_ {
        self.references.iter().flat_map(|(&file_id, refs)| {
            refs.iter().map(move |&FileReference { range, .. }| FileRange { file_id, range })
        })
    }
}

impl IntoIterator for UsageSearchResult {
    type Item = (EditionedFileId, Vec<FileReference>);
    type IntoIter = <FxHashMap<EditionedFileId, Vec<FileReference>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.references.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct FileReference {
    /// The range of the reference in the original file
    pub range: TextRange,
    /// The node of the reference in the (macro-)file
    pub name: FileReferenceNode,
    pub category: ReferenceCategory,
}

#[derive(Debug, Clone)]
pub enum FileReferenceNode {
    Name(ast::Name),
    NameRef(ast::NameRef),
    Lifetime(ast::Lifetime),
    FormatStringEntry(ast::String, TextRange),
}

impl FileReferenceNode {
    pub fn text_range(&self) -> TextRange {
        match self {
            FileReferenceNode::Name(it) => it.syntax().text_range(),
            FileReferenceNode::NameRef(it) => it.syntax().text_range(),
            FileReferenceNode::Lifetime(it) => it.syntax().text_range(),
            FileReferenceNode::FormatStringEntry(_, range) => *range,
        }
    }
    pub fn syntax(&self) -> SyntaxElement {
        match self {
            FileReferenceNode::Name(it) => it.syntax().clone().into(),
            FileReferenceNode::NameRef(it) => it.syntax().clone().into(),
            FileReferenceNode::Lifetime(it) => it.syntax().clone().into(),
            FileReferenceNode::FormatStringEntry(it, _) => it.syntax().clone().into(),
        }
    }
    pub fn into_name_like(self) -> Option<ast::NameLike> {
        match self {
            FileReferenceNode::Name(it) => Some(ast::NameLike::Name(it)),
            FileReferenceNode::NameRef(it) => Some(ast::NameLike::NameRef(it)),
            FileReferenceNode::Lifetime(it) => Some(ast::NameLike::Lifetime(it)),
            FileReferenceNode::FormatStringEntry(_, _) => None,
        }
    }
    pub fn as_name_ref(&self) -> Option<&ast::NameRef> {
        match self {
            FileReferenceNode::NameRef(name_ref) => Some(name_ref),
            _ => None,
        }
    }
    pub fn as_lifetime(&self) -> Option<&ast::Lifetime> {
        match self {
            FileReferenceNode::Lifetime(lifetime) => Some(lifetime),
            _ => None,
        }
    }
    pub fn text(&self) -> syntax::TokenText<'_> {
        match self {
            FileReferenceNode::NameRef(name_ref) => name_ref.text(),
            FileReferenceNode::Name(name) => name.text(),
            FileReferenceNode::Lifetime(lifetime) => lifetime.text(),
            FileReferenceNode::FormatStringEntry(it, range) => {
                syntax::TokenText::borrowed(&it.text()[*range - it.syntax().text_range().start()])
            }
        }
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
    pub struct ReferenceCategory: u8 {
        // FIXME: Add this variant and delete the `retain_adt_literal_usages` function.
        // const CREATE = 1 << 0;
        const WRITE = 1 << 0;
        const READ = 1 << 1;
        const IMPORT = 1 << 2;
        const TEST = 1 << 3;
    }
}

/// Generally, `search_scope` returns files that might contain references for the element.
/// For `pub(crate)` things it's a crate, for `pub` things it's a crate and dependant crates.
/// In some cases, the location of the references is known to within a `TextRange`,
/// e.g. for things like local variables.
#[derive(Clone, Debug)]
pub struct SearchScope {
    entries: FxHashMap<EditionedFileId, Option<TextRange>>,
}

impl SearchScope {
    fn new(entries: FxHashMap<EditionedFileId, Option<TextRange>>) -> SearchScope {
        SearchScope { entries }
    }

    /// Build a search scope spanning the entire crate graph of files.
    fn crate_graph(db: &RootDatabase) -> SearchScope {
        let mut entries = FxHashMap::default();

        let graph = db.crate_graph();
        for krate in graph.iter() {
            let root_file = graph[krate].root_file_id;
            let source_root_id = db.file_source_root(root_file);
            let source_root = db.source_root(source_root_id);
            entries.extend(
                source_root.iter().map(|id| (EditionedFileId::new(id, graph[krate].edition), None)),
            );
        }
        SearchScope { entries }
    }

    /// Build a search scope spanning all the reverse dependencies of the given crate.
    fn reverse_dependencies(db: &RootDatabase, of: hir::Crate) -> SearchScope {
        let mut entries = FxHashMap::default();
        for rev_dep in of.transitive_reverse_dependencies(db) {
            let root_file = rev_dep.root_file(db);
            let source_root_id = db.file_source_root(root_file);
            let source_root = db.source_root(source_root_id);
            entries.extend(
                source_root.iter().map(|id| (EditionedFileId::new(id, rev_dep.edition(db)), None)),
            );
        }
        SearchScope { entries }
    }

    /// Build a search scope spanning the given crate.
    fn krate(db: &RootDatabase, of: hir::Crate) -> SearchScope {
        let root_file = of.root_file(db);
        let source_root_id = db.file_source_root(root_file);
        let source_root = db.source_root(source_root_id);
        SearchScope {
            entries: source_root
                .iter()
                .map(|id| (EditionedFileId::new(id, of.edition(db)), None))
                .collect(),
        }
    }

    /// Build a search scope spanning the given module and all its submodules.
    pub fn module_and_children(db: &RootDatabase, module: hir::Module) -> SearchScope {
        let mut entries = FxHashMap::default();

        let (file_id, range) = {
            let InFile { file_id, value } = module.definition_source_range(db);
            if let Some(InRealFile { file_id, value: call_source }) = file_id.original_call_node(db)
            {
                (file_id, Some(call_source.text_range()))
            } else {
                (file_id.original_file(db), Some(value))
            }
        };
        entries.entry(file_id).or_insert(range);

        let mut to_visit: Vec<_> = module.children(db).collect();
        while let Some(module) = to_visit.pop() {
            if let Some(file_id) = module.as_source_file_id(db) {
                entries.insert(file_id, None);
            }
            to_visit.extend(module.children(db));
        }
        SearchScope { entries }
    }

    /// Build an empty search scope.
    pub fn empty() -> SearchScope {
        SearchScope::new(FxHashMap::default())
    }

    /// Build a empty search scope spanning the given file.
    pub fn single_file(file: EditionedFileId) -> SearchScope {
        SearchScope::new(std::iter::once((file, None)).collect())
    }

    /// Build a empty search scope spanning the text range of the given file.
    pub fn file_range(range: FileRange) -> SearchScope {
        SearchScope::new(std::iter::once((range.file_id, Some(range.range))).collect())
    }

    /// Build a empty search scope spanning the given files.
    pub fn files(files: &[EditionedFileId]) -> SearchScope {
        SearchScope::new(files.iter().map(|f| (*f, None)).collect())
    }

    pub fn intersection(&self, other: &SearchScope) -> SearchScope {
        let (mut small, mut large) = (&self.entries, &other.entries);
        if small.len() > large.len() {
            mem::swap(&mut small, &mut large)
        }

        let intersect_ranges =
            |r1: Option<TextRange>, r2: Option<TextRange>| -> Option<Option<TextRange>> {
                match (r1, r2) {
                    (None, r) | (r, None) => Some(r),
                    (Some(r1), Some(r2)) => r1.intersect(r2).map(Some),
                }
            };
        let res = small
            .iter()
            .filter_map(|(&file_id, &r1)| {
                let &r2 = large.get(&file_id)?;
                let r = intersect_ranges(r1, r2)?;
                Some((file_id, r))
            })
            .collect();

        SearchScope::new(res)
    }
}

impl IntoIterator for SearchScope {
    type Item = (EditionedFileId, Option<TextRange>);
    type IntoIter = std::collections::hash_map::IntoIter<EditionedFileId, Option<TextRange>>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl Definition {
    fn search_scope(&self, db: &RootDatabase) -> SearchScope {
        let _p = tracing::info_span!("search_scope").entered();

        if let Definition::BuiltinType(_) = self {
            return SearchScope::crate_graph(db);
        }

        // def is crate root
        if let &Definition::Module(module) = self {
            if module.is_crate_root() {
                return SearchScope::reverse_dependencies(db, module.krate());
            }
        }

        let module = match self.module(db) {
            Some(it) => it,
            None => return SearchScope::empty(),
        };
        let InFile { file_id, value: module_source } = module.definition_source(db);
        let file_id = file_id.original_file(db);

        if let Definition::Local(var) = self {
            let def = match var.parent(db) {
                DefWithBody::Function(f) => f.source(db).map(|src| src.syntax().cloned()),
                DefWithBody::Const(c) => c.source(db).map(|src| src.syntax().cloned()),
                DefWithBody::Static(s) => s.source(db).map(|src| src.syntax().cloned()),
                DefWithBody::Variant(v) => v.source(db).map(|src| src.syntax().cloned()),
                // FIXME: implement
                DefWithBody::InTypeConst(_) => return SearchScope::empty(),
            };
            return match def {
                Some(def) => SearchScope::file_range(
                    def.as_ref().original_file_range_with_macro_call_body(db),
                ),
                None => SearchScope::single_file(file_id),
            };
        }

        if let Definition::SelfType(impl_) = self {
            return match impl_.source(db).map(|src| src.syntax().cloned()) {
                Some(def) => SearchScope::file_range(
                    def.as_ref().original_file_range_with_macro_call_body(db),
                ),
                None => SearchScope::single_file(file_id),
            };
        }

        if let Definition::GenericParam(hir::GenericParam::LifetimeParam(param)) = self {
            let def = match param.parent(db) {
                hir::GenericDef::Function(it) => it.source(db).map(|src| src.syntax().cloned()),
                hir::GenericDef::Adt(it) => it.source(db).map(|src| src.syntax().cloned()),
                hir::GenericDef::Trait(it) => it.source(db).map(|src| src.syntax().cloned()),
                hir::GenericDef::TraitAlias(it) => it.source(db).map(|src| src.syntax().cloned()),
                hir::GenericDef::TypeAlias(it) => it.source(db).map(|src| src.syntax().cloned()),
                hir::GenericDef::Impl(it) => it.source(db).map(|src| src.syntax().cloned()),
                hir::GenericDef::Const(it) => it.source(db).map(|src| src.syntax().cloned()),
            };
            return match def {
                Some(def) => SearchScope::file_range(
                    def.as_ref().original_file_range_with_macro_call_body(db),
                ),
                None => SearchScope::single_file(file_id),
            };
        }

        if let Definition::Macro(macro_def) = self {
            return match macro_def.kind(db) {
                hir::MacroKind::Declarative => {
                    if macro_def.attrs(db).by_key(&sym::macro_export).exists() {
                        SearchScope::reverse_dependencies(db, module.krate())
                    } else {
                        SearchScope::krate(db, module.krate())
                    }
                }
                hir::MacroKind::BuiltIn => SearchScope::crate_graph(db),
                hir::MacroKind::Derive | hir::MacroKind::Attr | hir::MacroKind::ProcMacro => {
                    SearchScope::reverse_dependencies(db, module.krate())
                }
            };
        }

        if let Definition::DeriveHelper(_) = self {
            return SearchScope::reverse_dependencies(db, module.krate());
        }

        let vis = self.visibility(db);
        if let Some(Visibility::Public) = vis {
            return SearchScope::reverse_dependencies(db, module.krate());
        }
        if let Some(Visibility::Module(module, _)) = vis {
            return SearchScope::module_and_children(db, module.into());
        }

        let range = match module_source {
            ModuleSource::Module(m) => Some(m.syntax().text_range()),
            ModuleSource::BlockExpr(b) => Some(b.syntax().text_range()),
            ModuleSource::SourceFile(_) => None,
        };
        match range {
            Some(range) => SearchScope::file_range(FileRange { file_id, range }),
            None => SearchScope::single_file(file_id),
        }
    }

    pub fn usages<'a>(self, sema: &'a Semantics<'_, RootDatabase>) -> FindUsages<'a> {
        FindUsages {
            def: self,
            assoc_item_container: self.as_assoc_item(sema.db).map(|a| a.container(sema.db)),
            sema,
            scope: None,
            include_self_kw_refs: None,
            search_self_mod: false,
        }
    }
}

#[derive(Clone)]
pub struct FindUsages<'a> {
    def: Definition,
    sema: &'a Semantics<'a, RootDatabase>,
    scope: Option<&'a SearchScope>,
    /// The container of our definition should it be an assoc item
    assoc_item_container: Option<hir::AssocItemContainer>,
    /// whether to search for the `Self` type of the definition
    include_self_kw_refs: Option<hir::Type>,
    /// whether to search for the `self` module
    search_self_mod: bool,
}

impl<'a> FindUsages<'a> {
    /// Enable searching for `Self` when the definition is a type or `self` for modules.
    pub fn include_self_refs(mut self) -> Self {
        self.include_self_kw_refs = def_to_ty(self.sema, &self.def);
        self.search_self_mod = true;
        self
    }

    /// Limit the search to a given [`SearchScope`].
    pub fn in_scope(self, scope: &'a SearchScope) -> Self {
        self.set_scope(Some(scope))
    }

    /// Limit the search to a given [`SearchScope`].
    pub fn set_scope(mut self, scope: Option<&'a SearchScope>) -> Self {
        assert!(self.scope.is_none());
        self.scope = scope;
        self
    }

    pub fn at_least_one(&self) -> bool {
        let mut found = false;
        self.search(&mut |_, _| {
            found = true;
            true
        });
        found
    }

    pub fn all(self) -> UsageSearchResult {
        let mut res = UsageSearchResult::default();
        self.search(&mut |file_id, reference| {
            res.references.entry(file_id).or_default().push(reference);
            false
        });
        res
    }

    pub fn search(&self, sink: &mut dyn FnMut(EditionedFileId, FileReference) -> bool) {
        let _p = tracing::info_span!("FindUsages:search").entered();
        let sema = self.sema;

        let search_scope = {
            // FIXME: Is the trait scope needed for trait impl assoc items?
            let base =
                as_trait_assoc_def(sema.db, self.def).unwrap_or(self.def).search_scope(sema.db);
            match &self.scope {
                None => base,
                Some(scope) => base.intersection(scope),
            }
        };

        let name = match self.def {
            // special case crate modules as these do not have a proper name
            Definition::Module(module) if module.is_crate_root() => {
                // FIXME: This assumes the crate name is always equal to its display name when it
                // really isn't
                // we should instead look at the dependency edge name and recursively search our way
                // up the ancestors
                module
                    .krate()
                    .display_name(self.sema.db)
                    .map(|crate_name| crate_name.crate_name().symbol().as_str().into())
            }
            _ => {
                let self_kw_refs = || {
                    self.include_self_kw_refs.as_ref().and_then(|ty| {
                        ty.as_adt()
                            .map(|adt| adt.name(self.sema.db))
                            .or_else(|| ty.as_builtin().map(|builtin| builtin.name()))
                    })
                };
                // We need to unescape the name in case it is written without "r#" in earlier
                // editions of Rust where it isn't a keyword.
                self.def
                    .name(sema.db)
                    .or_else(self_kw_refs)
                    .map(|it| it.unescaped().display(sema.db).to_smolstr())
            }
        };
        let name = match &name {
            Some(s) => s.as_str(),
            None => return,
        };
        let finder = &Finder::new(name);
        let include_self_kw_refs =
            self.include_self_kw_refs.as_ref().map(|ty| (ty, Finder::new("Self")));

        // for<'a> |text: &'a str, name: &'a str, search_range: TextRange| -> impl Iterator<Item = TextSize> + 'a { ... }
        fn match_indices<'a>(
            text: &'a str,
            finder: &'a Finder<'a>,
            search_range: TextRange,
        ) -> impl Iterator<Item = TextSize> + 'a {
            finder.find_iter(text.as_bytes()).filter_map(move |idx| {
                let offset: TextSize = idx.try_into().unwrap();
                if !search_range.contains_inclusive(offset) {
                    return None;
                }
                Some(offset)
            })
        }

        // for<'a> |scope: &'a SearchScope| -> impl Iterator<Item = (Arc<String>, EditionedFileId, TextRange)> + 'a { ... }
        fn scope_files<'a>(
            sema: &'a Semantics<'_, RootDatabase>,
            scope: &'a SearchScope,
        ) -> impl Iterator<Item = (Arc<str>, EditionedFileId, TextRange)> + 'a {
            scope.entries.iter().map(|(&file_id, &search_range)| {
                let text = sema.db.file_text(file_id.file_id());
                let search_range =
                    search_range.unwrap_or_else(|| TextRange::up_to(TextSize::of(&*text)));

                (text, file_id, search_range)
            })
        }

        let find_nodes = move |name: &str, node: &syntax::SyntaxNode, offset: TextSize| {
            node.token_at_offset(offset)
                .find(|it| {
                    // `name` is stripped of raw ident prefix. See the comment on name retrieval above.
                    it.text().trim_start_matches("r#") == name
                })
                .into_iter()
                .flat_map(move |token| {
                    // FIXME: There should be optimization potential here
                    // Currently we try to descend everything we find which
                    // means we call `Semantics::descend_into_macros` on
                    // every textual hit. That function is notoriously
                    // expensive even for things that do not get down mapped
                    // into macros.
                    sema.descend_into_macros(DescendPreference::None, token)
                        .into_iter()
                        .filter_map(|it| it.parent())
                })
        };

        for (text, file_id, search_range) in scope_files(sema, &search_scope) {
            self.sema.db.unwind_if_cancelled();
            let tree = Lazy::new(move || sema.parse(file_id).syntax().clone());

            // Search for occurrences of the items name
            for offset in match_indices(&text, finder, search_range) {
                tree.token_at_offset(offset).for_each(|token| {
                    let Some(str_token) = ast::String::cast(token.clone()) else { return };
                    if let Some((range, nameres)) =
                        sema.check_for_format_args_template(token, offset)
                    {
                        if self.found_format_args_ref(file_id, range, str_token, nameres, sink) {}
                    }
                });

                for name in find_nodes(name, &tree, offset).filter_map(ast::NameLike::cast) {
                    if match name {
                        ast::NameLike::NameRef(name_ref) => self.found_name_ref(&name_ref, sink),
                        ast::NameLike::Name(name) => self.found_name(&name, sink),
                        ast::NameLike::Lifetime(lifetime) => self.found_lifetime(&lifetime, sink),
                    } {
                        return;
                    }
                }
            }
            // Search for occurrences of the `Self` referring to our type
            if let Some((self_ty, finder)) = &include_self_kw_refs {
                for offset in match_indices(&text, finder, search_range) {
                    for name_ref in find_nodes("Self", &tree, offset).filter_map(ast::NameRef::cast)
                    {
                        if self.found_self_ty_name_ref(self_ty, &name_ref, sink) {
                            return;
                        }
                    }
                }
            }
        }

        // Search for `super` and `crate` resolving to our module
        if let Definition::Module(module) = self.def {
            let scope =
                search_scope.intersection(&SearchScope::module_and_children(self.sema.db, module));

            let is_crate_root = module.is_crate_root().then(|| Finder::new("crate"));
            let finder = &Finder::new("super");

            for (text, file_id, search_range) in scope_files(sema, &scope) {
                self.sema.db.unwind_if_cancelled();
                let tree = Lazy::new(move || sema.parse(file_id).syntax().clone());

                for offset in match_indices(&text, finder, search_range) {
                    for name_ref in
                        find_nodes("super", &tree, offset).filter_map(ast::NameRef::cast)
                    {
                        if self.found_name_ref(&name_ref, sink) {
                            return;
                        }
                    }
                }
                if let Some(finder) = &is_crate_root {
                    for offset in match_indices(&text, finder, search_range) {
                        for name_ref in
                            find_nodes("crate", &tree, offset).filter_map(ast::NameRef::cast)
                        {
                            if self.found_name_ref(&name_ref, sink) {
                                return;
                            }
                        }
                    }
                }
            }
        }

        // search for module `self` references in our module's definition source
        match self.def {
            Definition::Module(module) if self.search_self_mod => {
                let src = module.definition_source(sema.db);
                let file_id = src.file_id.original_file(sema.db);
                let (file_id, search_range) = match src.value {
                    ModuleSource::Module(m) => (file_id, Some(m.syntax().text_range())),
                    ModuleSource::BlockExpr(b) => (file_id, Some(b.syntax().text_range())),
                    ModuleSource::SourceFile(_) => (file_id, None),
                };

                let search_range = if let Some(&range) = search_scope.entries.get(&file_id) {
                    match (range, search_range) {
                        (None, range) | (range, None) => range,
                        (Some(range), Some(search_range)) => match range.intersect(search_range) {
                            Some(range) => Some(range),
                            None => return,
                        },
                    }
                } else {
                    return;
                };

                let text = sema.db.file_text(file_id.file_id());
                let search_range =
                    search_range.unwrap_or_else(|| TextRange::up_to(TextSize::of(&*text)));

                let tree = Lazy::new(|| sema.parse(file_id).syntax().clone());
                let finder = &Finder::new("self");

                for offset in match_indices(&text, finder, search_range) {
                    for name_ref in find_nodes("self", &tree, offset).filter_map(ast::NameRef::cast)
                    {
                        if self.found_self_module_name_ref(&name_ref, sink) {
                            return;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn found_self_ty_name_ref(
        &self,
        self_ty: &hir::Type,
        name_ref: &ast::NameRef,
        sink: &mut dyn FnMut(EditionedFileId, FileReference) -> bool,
    ) -> bool {
        match NameRefClass::classify(self.sema, name_ref) {
            Some(NameRefClass::Definition(Definition::SelfType(impl_)))
                if impl_.self_ty(self.sema.db).as_adt() == self_ty.as_adt() =>
            {
                let FileRange { file_id, range } = self.sema.original_range(name_ref.syntax());
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::NameRef(name_ref.clone()),
                    category: ReferenceCategory::empty(),
                };
                sink(file_id, reference)
            }
            _ => false,
        }
    }

    fn found_self_module_name_ref(
        &self,
        name_ref: &ast::NameRef,
        sink: &mut dyn FnMut(EditionedFileId, FileReference) -> bool,
    ) -> bool {
        match NameRefClass::classify(self.sema, name_ref) {
            Some(NameRefClass::Definition(def @ Definition::Module(_))) if def == self.def => {
                let FileRange { file_id, range } = self.sema.original_range(name_ref.syntax());
                let category = if is_name_ref_in_import(name_ref) {
                    ReferenceCategory::IMPORT
                } else {
                    ReferenceCategory::empty()
                };
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::NameRef(name_ref.clone()),
                    category,
                };
                sink(file_id, reference)
            }
            _ => false,
        }
    }

    fn found_format_args_ref(
        &self,
        file_id: EditionedFileId,
        range: TextRange,
        token: ast::String,
        res: Option<PathResolution>,
        sink: &mut dyn FnMut(EditionedFileId, FileReference) -> bool,
    ) -> bool {
        match res.map(Definition::from) {
            Some(def) if def == self.def => {
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::FormatStringEntry(token, range),
                    category: ReferenceCategory::READ,
                };
                sink(file_id, reference)
            }
            _ => false,
        }
    }

    fn found_lifetime(
        &self,
        lifetime: &ast::Lifetime,
        sink: &mut dyn FnMut(EditionedFileId, FileReference) -> bool,
    ) -> bool {
        match NameRefClass::classify_lifetime(self.sema, lifetime) {
            Some(NameRefClass::Definition(def)) if def == self.def => {
                let FileRange { file_id, range } = self.sema.original_range(lifetime.syntax());
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::Lifetime(lifetime.clone()),
                    category: ReferenceCategory::empty(),
                };
                sink(file_id, reference)
            }
            _ => false,
        }
    }

    fn found_name_ref(
        &self,
        name_ref: &ast::NameRef,
        sink: &mut dyn FnMut(EditionedFileId, FileReference) -> bool,
    ) -> bool {
        match NameRefClass::classify(self.sema, name_ref) {
            Some(NameRefClass::Definition(def))
                if self.def == def
                    // is our def a trait assoc item? then we want to find all assoc items from trait impls of our trait
                    || matches!(self.assoc_item_container, Some(hir::AssocItemContainer::Trait(_)))
                        && convert_to_def_in_trait(self.sema.db, def) == self.def =>
            {
                let FileRange { file_id, range } = self.sema.original_range(name_ref.syntax());
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::NameRef(name_ref.clone()),
                    category: ReferenceCategory::new(self.sema, &def, name_ref),
                };
                sink(file_id, reference)
            }
            // FIXME: special case type aliases, we can't filter between impl and trait defs here as we lack the substitutions
            // so we always resolve all assoc type aliases to both their trait def and impl defs
            Some(NameRefClass::Definition(def))
                if self.assoc_item_container.is_some()
                    && matches!(self.def, Definition::TypeAlias(_))
                    && convert_to_def_in_trait(self.sema.db, def)
                        == convert_to_def_in_trait(self.sema.db, self.def) =>
            {
                let FileRange { file_id, range } = self.sema.original_range(name_ref.syntax());
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::NameRef(name_ref.clone()),
                    category: ReferenceCategory::new(self.sema, &def, name_ref),
                };
                sink(file_id, reference)
            }
            Some(NameRefClass::Definition(def)) if self.include_self_kw_refs.is_some() => {
                if self.include_self_kw_refs == def_to_ty(self.sema, &def) {
                    let FileRange { file_id, range } = self.sema.original_range(name_ref.syntax());
                    let reference = FileReference {
                        range,
                        name: FileReferenceNode::NameRef(name_ref.clone()),
                        category: ReferenceCategory::new(self.sema, &def, name_ref),
                    };
                    sink(file_id, reference)
                } else {
                    false
                }
            }
            Some(NameRefClass::FieldShorthand { local_ref: local, field_ref: field }) => {
                let FileRange { file_id, range } = self.sema.original_range(name_ref.syntax());

                let field = Definition::Field(field);
                let local = Definition::Local(local);
                let access = match self.def {
                    Definition::Field(_) if field == self.def => {
                        ReferenceCategory::new(self.sema, &field, name_ref)
                    }
                    Definition::Local(_) if local == self.def => {
                        ReferenceCategory::new(self.sema, &local, name_ref)
                    }
                    _ => return false,
                };
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::NameRef(name_ref.clone()),
                    category: access,
                };
                sink(file_id, reference)
            }
            _ => false,
        }
    }

    fn found_name(
        &self,
        name: &ast::Name,
        sink: &mut dyn FnMut(EditionedFileId, FileReference) -> bool,
    ) -> bool {
        match NameClass::classify(self.sema, name) {
            Some(NameClass::PatFieldShorthand { local_def: _, field_ref })
                if matches!(
                    self.def, Definition::Field(_) if Definition::Field(field_ref) == self.def
                ) =>
            {
                let FileRange { file_id, range } = self.sema.original_range(name.syntax());
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::Name(name.clone()),
                    // FIXME: mutable patterns should have `Write` access
                    category: ReferenceCategory::READ,
                };
                sink(file_id, reference)
            }
            Some(NameClass::ConstReference(def)) if self.def == def => {
                let FileRange { file_id, range } = self.sema.original_range(name.syntax());
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::Name(name.clone()),
                    category: ReferenceCategory::empty(),
                };
                sink(file_id, reference)
            }
            Some(NameClass::Definition(def)) if def != self.def => {
                match (&self.assoc_item_container, self.def) {
                    // for type aliases we always want to reference the trait def and all the trait impl counterparts
                    // FIXME: only until we can resolve them correctly, see FIXME above
                    (Some(_), Definition::TypeAlias(_))
                        if convert_to_def_in_trait(self.sema.db, def)
                            != convert_to_def_in_trait(self.sema.db, self.def) =>
                    {
                        return false
                    }
                    (Some(_), Definition::TypeAlias(_)) => {}
                    // We looking at an assoc item of a trait definition, so reference all the
                    // corresponding assoc items belonging to this trait's trait implementations
                    (Some(hir::AssocItemContainer::Trait(_)), _)
                        if convert_to_def_in_trait(self.sema.db, def) == self.def => {}
                    _ => return false,
                }
                let FileRange { file_id, range } = self.sema.original_range(name.syntax());
                let reference = FileReference {
                    range,
                    name: FileReferenceNode::Name(name.clone()),
                    category: ReferenceCategory::empty(),
                };
                sink(file_id, reference)
            }
            _ => false,
        }
    }
}

fn def_to_ty(sema: &Semantics<'_, RootDatabase>, def: &Definition) -> Option<hir::Type> {
    match def {
        Definition::Adt(adt) => Some(adt.ty(sema.db)),
        Definition::TypeAlias(it) => Some(it.ty(sema.db)),
        Definition::BuiltinType(it) => Some(it.ty(sema.db)),
        Definition::SelfType(it) => Some(it.self_ty(sema.db)),
        _ => None,
    }
}

impl ReferenceCategory {
    fn new(
        sema: &Semantics<'_, RootDatabase>,
        def: &Definition,
        r: &ast::NameRef,
    ) -> ReferenceCategory {
        let mut result = ReferenceCategory::empty();
        if is_name_ref_in_test(sema, r) {
            result |= ReferenceCategory::TEST;
        }

        // Only Locals and Fields have accesses for now.
        if !matches!(def, Definition::Local(_) | Definition::Field(_)) {
            if is_name_ref_in_import(r) {
                result |= ReferenceCategory::IMPORT;
            }
            return result;
        }

        let mode = r.syntax().ancestors().find_map(|node| {
            match_ast! {
                match node {
                    ast::BinExpr(expr) => {
                        if matches!(expr.op_kind()?, ast::BinaryOp::Assignment { .. }) {
                            // If the variable or field ends on the LHS's end then it's a Write
                            // (covers fields and locals). FIXME: This is not terribly accurate.
                            if let Some(lhs) = expr.lhs() {
                                if lhs.syntax().text_range().end() == r.syntax().text_range().end() {
                                    return Some(ReferenceCategory::WRITE)
                                }
                            }
                        }
                        Some(ReferenceCategory::READ)
                    },
                    _ => None,
                }
            }
        }).unwrap_or(ReferenceCategory::READ);

        result | mode
    }
}

fn is_name_ref_in_import(name_ref: &ast::NameRef) -> bool {
    name_ref
        .syntax()
        .parent()
        .and_then(ast::PathSegment::cast)
        .and_then(|it| it.parent_path().top_path().syntax().parent())
        .map_or(false, |it| it.kind() == SyntaxKind::USE_TREE)
}

fn is_name_ref_in_test(sema: &Semantics<'_, RootDatabase>, name_ref: &ast::NameRef) -> bool {
    name_ref.syntax().ancestors().any(|node| match ast::Fn::cast(node) {
        Some(it) => sema.to_def(&it).map_or(false, |func| func.is_test(sema.db)),
        None => false,
    })
}
