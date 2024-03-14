use crate::filesearch::make_target_lib_path;
use crate::EarlyDiagCtxt;
use rustc_target::spec::TargetTriple;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct SearchPath {
    pub kind: PathKind,
    pub dir: PathBuf,
    pub files: Vec<SearchPathFile>,
}

/// The obvious implementation of `SearchPath::files` is a `Vec<PathBuf>`. But
/// it is searched repeatedly by `find_library_crate`, and the searches involve
/// checking the prefix and suffix of the filename of each `PathBuf`. This is
/// doable, but very slow, because it involves calls to `file_name` and
/// `extension` that are themselves slow.
///
/// This type augments the `PathBuf` with an `String` containing the
/// `PathBuf`'s filename. The prefix and suffix checking is much faster on the
/// `String` than the `PathBuf`. (The filename must be valid UTF-8. If it's
/// not, the entry should be skipped, because all Rust output files are valid
/// UTF-8, and so a non-UTF-8 filename couldn't be one we're looking for.)
#[derive(Clone, Debug)]
pub struct SearchPathFile {
    pub path: PathBuf,
    pub file_name_str: String,
}

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq, Encodable, Decodable, HashStable_Generic)]
pub enum PathKind {
    Native,
    Crate,
    Dependency,
    Framework,
    ExternFlag,
    All,
}

impl PathKind {
    pub fn matches(&self, kind: PathKind) -> bool {
        match (self, kind) {
            (PathKind::All, _) | (_, PathKind::All) => true,
            _ => *self == kind,
        }
    }
}

impl SearchPath {
    pub fn from_cli_opt(
        sysroot: Option<&Path>,
        triple: &TargetTriple,
        early_dcx: &EarlyDiagCtxt,
        path: &str,
    ) -> Self {
        let (kind, path) = if let Some(stripped) = path.strip_prefix("native=") {
            (PathKind::Native, stripped)
        } else if let Some(stripped) = path.strip_prefix("crate=") {
            (PathKind::Crate, stripped)
        } else if let Some(stripped) = path.strip_prefix("dependency=") {
            (PathKind::Dependency, stripped)
        } else if let Some(stripped) = path.strip_prefix("framework=") {
            (PathKind::Framework, stripped)
        } else if let Some(stripped) = path.strip_prefix("all=") {
            (PathKind::All, stripped)
        } else {
            (PathKind::All, path)
        };
        if path.is_empty() {
            #[allow(rustc::untranslatable_diagnostic)] // FIXME: make this translatable
            early_dcx.early_fatal("empty search path given via `-L`");
        }

        // Temporary implementation until https://github.com/rust-lang/compiler-team/issues/659 is
        // accepted and implemented upstream.
        let dir = if let Some(stripped) = path.strip_prefix("ferrocene-temp-builtin:") {
            let Some(sysroot) = sysroot else {
                early_dcx.early_fatal("`-L ferrocene-temp-builtin:` is not supported");
            };
            let triple = match triple {
                TargetTriple::TargetTriple(triple) => triple,
                TargetTriple::TargetJson { .. } => {
                    early_dcx.early_fatal(
                        "`-L ferrocene-temp-builtin:` is not supported with custom targets",
                    );
                }
            };

            if stripped == ".." || stripped.contains('/') {
                early_dcx.early_fatal(
                    "`-L ferrocene-temp-builtin:` does not accept \
                     subdirectories or parent directories",
                );
            }

            let path = make_target_lib_path(sysroot, triple).join("builtin").join(stripped);
            if !path.is_dir() {
                early_dcx.early_fatal(format!("ferrocene-temp-builtin:{stripped} does not exist"));
            }

            path
        } else {
            PathBuf::from(path)
        };

        Self::new(kind, dir)
    }

    pub fn from_sysroot_and_triple(sysroot: &Path, triple: &str) -> Self {
        Self::new(PathKind::All, make_target_lib_path(sysroot, triple))
    }

    fn new(kind: PathKind, dir: PathBuf) -> Self {
        // Get the files within the directory.
        let files = match std::fs::read_dir(&dir) {
            Ok(files) => files
                .filter_map(|e| {
                    e.ok().and_then(|e| {
                        e.file_name().to_str().map(|s| SearchPathFile {
                            path: e.path(),
                            file_name_str: s.to_string(),
                        })
                    })
                })
                .collect::<Vec<_>>(),
            Err(..) => vec![],
        };

        SearchPath { kind, dir, files }
    }
}
