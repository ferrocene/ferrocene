//! base_db defines basic database traits. The concrete DB is defined by ide.

mod change;
mod input;

use std::panic;

use salsa::Durability;
use span::EditionedFileId;
use syntax::{ast, Parse, SourceFile, SyntaxError};
use triomphe::Arc;
use vfs::FileId;

pub use crate::{
    change::FileChange,
    input::{
        CrateData, CrateDisplayName, CrateGraph, CrateId, CrateName, CrateOrigin, Dependency, Env,
        LangCrateOrigin, ProcMacroPaths, ReleaseChannel, SourceRoot, SourceRootId,
        TargetLayoutLoadResult,
    },
};
pub use salsa::{self, Cancelled};
pub use vfs::{file_set::FileSet, AnchoredPath, AnchoredPathBuf, VfsPath};

pub use semver::{BuildMetadata, Prerelease, Version, VersionReq};

#[macro_export]
macro_rules! impl_intern_key {
    ($name:ident) => {
        impl $crate::salsa::InternKey for $name {
            fn from_intern_id(v: $crate::salsa::InternId) -> Self {
                $name(v)
            }
            fn as_intern_id(&self) -> $crate::salsa::InternId {
                self.0
            }
        }
    };
}

pub trait Upcast<T: ?Sized> {
    fn upcast(&self) -> &T;
}

pub const DEFAULT_FILE_TEXT_LRU_CAP: u16 = 16;
pub const DEFAULT_PARSE_LRU_CAP: u16 = 128;
pub const DEFAULT_BORROWCK_LRU_CAP: u16 = 2024;

pub trait FileLoader {
    /// Text of the file.
    fn file_text(&self, file_id: FileId) -> Arc<str>;
    fn resolve_path(&self, path: AnchoredPath<'_>) -> Option<FileId>;
    /// Crates whose root's source root is the same as the source root of `file_id`
    fn relevant_crates(&self, file_id: FileId) -> Arc<[CrateId]>;
}

/// Database which stores all significant input facts: source code and project
/// model. Everything else in rust-analyzer is derived from these queries.
#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: FileLoader + std::fmt::Debug {
    /// Parses the file into the syntax tree.
    #[salsa::lru]
    fn parse(&self, file_id: EditionedFileId) -> Parse<ast::SourceFile>;

    /// Returns the set of errors obtained from parsing the file including validation errors.
    fn parse_errors(&self, file_id: EditionedFileId) -> Option<Arc<[SyntaxError]>>;

    /// The crate graph.
    #[salsa::input]
    fn crate_graph(&self) -> Arc<CrateGraph>;

    // FIXME: Consider removing this, making HirDatabase::target_data_layout an input query
    #[salsa::input]
    fn data_layout(&self, krate: CrateId) -> TargetLayoutLoadResult;

    #[salsa::input]
    fn toolchain(&self, krate: CrateId) -> Option<Version>;

    #[salsa::transparent]
    fn toolchain_channel(&self, krate: CrateId) -> Option<ReleaseChannel>;
}

fn toolchain_channel(db: &dyn SourceDatabase, krate: CrateId) -> Option<ReleaseChannel> {
    db.toolchain(krate).as_ref().and_then(|v| ReleaseChannel::from_str(&v.pre))
}

fn parse(db: &dyn SourceDatabase, file_id: EditionedFileId) -> Parse<ast::SourceFile> {
    let _p = tracing::info_span!("parse", ?file_id).entered();
    let (file_id, edition) = file_id.unpack();
    let text = db.file_text(file_id);
    SourceFile::parse(&text, edition)
}

fn parse_errors(db: &dyn SourceDatabase, file_id: EditionedFileId) -> Option<Arc<[SyntaxError]>> {
    let errors = db.parse(file_id).errors();
    match &*errors {
        [] => None,
        [..] => Some(errors.into()),
    }
}

/// We don't want to give HIR knowledge of source roots, hence we extract these
/// methods into a separate DB.
#[salsa::query_group(SourceDatabaseExtStorage)]
pub trait SourceDatabaseExt: SourceDatabase {
    #[salsa::input]
    fn compressed_file_text(&self, file_id: FileId) -> Arc<[u8]>;

    #[salsa::lru]
    fn file_text(&self, file_id: FileId) -> Arc<str>;

    /// Path to a file, relative to the root of its source root.
    /// Source root of the file.
    #[salsa::input]
    fn file_source_root(&self, file_id: FileId) -> SourceRootId;
    /// Contents of the source root.
    #[salsa::input]
    fn source_root(&self, id: SourceRootId) -> Arc<SourceRoot>;

    /// Crates whose root fool is in `id`.
    fn source_root_crates(&self, id: SourceRootId) -> Arc<[CrateId]>;
}

fn file_text(db: &dyn SourceDatabaseExt, file_id: FileId) -> Arc<str> {
    let bytes = db.compressed_file_text(file_id);
    let bytes =
        lz4_flex::decompress_size_prepended(&bytes).expect("lz4 decompression should not fail");
    let text = std::str::from_utf8(&bytes).expect("file contents should be valid UTF-8");
    Arc::from(text)
}

pub trait SourceDatabaseExt2 {
    fn set_file_text(&mut self, file_id: FileId, text: &str) {
        self.set_file_text_with_durability(file_id, text, Durability::LOW);
    }

    fn set_file_text_with_durability(
        &mut self,
        file_id: FileId,
        text: &str,
        durability: Durability,
    );
}

impl<Db: ?Sized + SourceDatabaseExt> SourceDatabaseExt2 for Db {
    fn set_file_text_with_durability(
        &mut self,
        file_id: FileId,
        text: &str,
        durability: Durability,
    ) {
        let bytes = text.as_bytes();
        let compressed = lz4_flex::compress_prepend_size(bytes);
        self.set_compressed_file_text_with_durability(
            file_id,
            Arc::from(compressed.as_slice()),
            durability,
        )
    }
}

fn source_root_crates(db: &dyn SourceDatabaseExt, id: SourceRootId) -> Arc<[CrateId]> {
    let graph = db.crate_graph();
    let mut crates = graph
        .iter()
        .filter(|&krate| {
            let root_file = graph[krate].root_file_id;
            db.file_source_root(root_file) == id
        })
        .collect::<Vec<_>>();
    crates.sort();
    crates.dedup();
    crates.into_iter().collect()
}

/// Silly workaround for cyclic deps between the traits
pub struct FileLoaderDelegate<T>(pub T);

impl<T: SourceDatabaseExt> FileLoader for FileLoaderDelegate<&'_ T> {
    fn file_text(&self, file_id: FileId) -> Arc<str> {
        SourceDatabaseExt::file_text(self.0, file_id)
    }
    fn resolve_path(&self, path: AnchoredPath<'_>) -> Option<FileId> {
        // FIXME: this *somehow* should be platform agnostic...
        let source_root = self.0.file_source_root(path.anchor);
        let source_root = self.0.source_root(source_root);
        source_root.resolve_path(path)
    }

    fn relevant_crates(&self, file_id: FileId) -> Arc<[CrateId]> {
        let _p = tracing::info_span!("relevant_crates").entered();
        let source_root = self.0.file_source_root(file_id);
        self.0.source_root_crates(source_root)
    }
}
