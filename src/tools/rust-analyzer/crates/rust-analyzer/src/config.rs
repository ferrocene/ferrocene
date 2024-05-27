//! Config used by the language server.
//!
//! We currently get this config from `initialize` LSP request, which is not the
//! best way to do it, but was the simplest thing we could implement.
//!
//! Of particular interest is the `feature_flags` hash map: while other fields
//! configure the server itself, feature flags are passed into analysis, and
//! tweak things like automatic insertion of `()` in completions.
use std::{fmt, iter, ops::Not};

use cfg::{CfgAtom, CfgDiff};
use flycheck::{CargoOptions, FlycheckConfig};
use ide::{
    AssistConfig, CallableSnippets, CompletionConfig, DiagnosticsConfig, ExprFillDefaultMode,
    HighlightConfig, HighlightRelatedConfig, HoverConfig, HoverDocFormat, InlayFieldsToResolve,
    InlayHintsConfig, JoinLinesConfig, MemoryLayoutHoverConfig, MemoryLayoutHoverRenderKind,
    Snippet, SnippetScope, SourceRootId,
};
use ide_db::{
    imports::insert_use::{ImportGranularity, InsertUseConfig, PrefixKind},
    SnippetCap,
};
use indexmap::IndexMap;
use itertools::Itertools;
use lsp_types::{ClientCapabilities, MarkupKind};
use paths::{Utf8Path, Utf8PathBuf};
use project_model::{
    CargoConfig, CargoFeatures, ProjectJson, ProjectJsonData, ProjectManifest, RustLibSource,
};
use rustc_hash::{FxHashMap, FxHashSet};
use semver::Version;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use stdx::format_to_acc;
use vfs::{AbsPath, AbsPathBuf};

use crate::{
    caps::completion_item_edit_resolve,
    diagnostics::DiagnosticsMapConfig,
    line_index::PositionEncoding,
    lsp_ext::{self, negotiated_encoding, WorkspaceSymbolSearchKind, WorkspaceSymbolSearchScope},
};

mod patch_old_style;

// Conventions for configuration keys to preserve maximal extendability without breakage:
//  - Toggles (be it binary true/false or with more options in-between) should almost always suffix as `_enable`
//    This has the benefit of namespaces being extensible, and if the suffix doesn't fit later it can be changed without breakage.
//  - In general be wary of using the namespace of something verbatim, it prevents us from adding subkeys in the future
//  - Don't use abbreviations unless really necessary
//  - foo_command = overrides the subcommand, foo_overrideCommand allows full overwriting, extra args only applies for foo_command

// Defines the server-side configuration of the rust-analyzer. We generate
// *parts* of VS Code's `package.json` config from this. Run `cargo test` to
// re-generate that file.
//
// However, editor specific config, which the server doesn't know about, should
// be specified directly in `package.json`.
//
// To deprecate an option by replacing it with another name use `new_name | `old_name` so that we keep
// parsing the old name.
config_data! {
    /// Configs that apply on a workspace-wide scope. There are 3 levels on which a global configuration can be configured
    ///
    /// 1. `rust-analyzer.toml` file under user's config directory (e.g ~/.config/rust-analyzer.toml)
    /// 2. Client's own configurations (e.g `settings.json` on VS Code)
    /// 3. `rust-analyzer.toml` file located at the workspace root
    ///
    /// A config is searched for by traversing a "config tree" in a bottom up fashion. It is chosen by the nearest first principle.
    global: struct GlobalDefaultConfigData <- GlobalConfigInput -> {
        /// Whether to insert #[must_use] when generating `as_` methods
        /// for enum variants.
        assist_emitMustUse: bool               = false,
        /// Placeholder expression to use for missing expressions in assists.
        assist_expressionFillDefault: ExprFillDefaultDef              = ExprFillDefaultDef::Todo,

        /// Warm up caches on project load.
        cachePriming_enable: bool = true,
        /// How many worker threads to handle priming caches. The default `0` means to pick automatically.
        cachePriming_numThreads: ParallelCachePrimingNumThreads = 0u8,

        /// Pass `--all-targets` to cargo invocation.
        cargo_allTargets: bool           = true,
        /// Automatically refresh project info via `cargo metadata` on
        /// `Cargo.toml` or `.cargo/config.toml` changes.
        pub(crate) cargo_autoreload: bool           = true,
        /// Run build scripts (`build.rs`) for more precise code analysis.
        cargo_buildScripts_enable: bool  = true,
        /// Specifies the working directory for running build scripts.
        /// - "workspace": run build scripts for a workspace in the workspace's root directory.
        ///   This is incompatible with `#rust-analyzer.cargo.buildScripts.invocationStrategy#` set to `once`.
        /// - "root": run build scripts in the project's root directory.
        /// This config only has an effect when `#rust-analyzer.cargo.buildScripts.overrideCommand#`
        /// is set.
        cargo_buildScripts_invocationLocation: InvocationLocation = InvocationLocation::Workspace,
        /// Specifies the invocation strategy to use when running the build scripts command.
        /// If `per_workspace` is set, the command will be executed for each workspace.
        /// If `once` is set, the command will be executed once.
        /// This config only has an effect when `#rust-analyzer.cargo.buildScripts.overrideCommand#`
        /// is set.
        cargo_buildScripts_invocationStrategy: InvocationStrategy = InvocationStrategy::PerWorkspace,
        /// Override the command rust-analyzer uses to run build scripts and
        /// build procedural macros. The command is required to output json
        /// and should therefore include `--message-format=json` or a similar
        /// option.
        ///
        /// If there are multiple linked projects/workspaces, this command is invoked for
        /// each of them, with the working directory being the workspace root
        /// (i.e., the folder containing the `Cargo.toml`). This can be overwritten
        /// by changing `#rust-analyzer.cargo.buildScripts.invocationStrategy#` and
        /// `#rust-analyzer.cargo.buildScripts.invocationLocation#`.
        ///
        /// By default, a cargo invocation will be constructed for the configured
        /// targets and features, with the following base command line:
        ///
        /// ```bash
        /// cargo check --quiet --workspace --message-format=json --all-targets
        /// ```
        /// .
        cargo_buildScripts_overrideCommand: Option<Vec<String>> = None,
        /// Rerun proc-macros building/build-scripts running when proc-macro
        /// or build-script sources change and are saved.
        cargo_buildScripts_rebuildOnSave: bool = true,
        /// Use `RUSTC_WRAPPER=rust-analyzer` when running build scripts to
        /// avoid checking unnecessary things.
        cargo_buildScripts_useRustcWrapper: bool = true,
        /// List of cfg options to enable with the given values.
        cargo_cfgs: FxHashMap<String, Option<String>> = {
            let mut m = FxHashMap::default();
            m.insert("debug_assertions".to_owned(), None);
            m.insert("miri".to_owned(), None);
            m
        },
        /// Extra arguments that are passed to every cargo invocation.
        cargo_extraArgs: Vec<String> = vec![],
        /// Extra environment variables that will be set when running cargo, rustc
        /// or other commands within the workspace. Useful for setting RUSTFLAGS.
        cargo_extraEnv: FxHashMap<String, String> = FxHashMap::default(),
        /// List of features to activate.
        ///
        /// Set this to `"all"` to pass `--all-features` to cargo.
        cargo_features: CargoFeaturesDef      = CargoFeaturesDef::Selected(vec![]),
        /// Whether to pass `--no-default-features` to cargo.
        cargo_noDefaultFeatures: bool    = false,
        /// Relative path to the sysroot, or "discover" to try to automatically find it via
        /// "rustc --print sysroot".
        ///
        /// Unsetting this disables sysroot loading.
        ///
        /// This option does not take effect until rust-analyzer is restarted.
        cargo_sysroot: Option<String>    = Some("discover".to_owned()),
        /// Whether to run cargo metadata on the sysroot library allowing rust-analyzer to analyze
        /// third-party dependencies of the standard libraries.
        ///
        /// This will cause `cargo` to create a lockfile in your sysroot directory. rust-analyzer
        /// will attempt to clean up afterwards, but nevertheless requires the location to be
        /// writable to.
        cargo_sysrootQueryMetadata: bool     = false,
        /// Relative path to the sysroot library sources. If left unset, this will default to
        /// `{cargo.sysroot}/lib/rustlib/src/rust/library`.
        ///
        /// This option does not take effect until rust-analyzer is restarted.
        cargo_sysrootSrc: Option<String>    = None,
        /// Compilation target override (target triple).
        // FIXME(@poliorcetics): move to multiple targets here too, but this will need more work
        // than `checkOnSave_target`
        cargo_target: Option<String>     = None,
        /// Optional path to a rust-analyzer specific target directory.
        /// This prevents rust-analyzer's `cargo check` and initial build-script and proc-macro
        /// building from locking the `Cargo.lock` at the expense of duplicating build artifacts.
        ///
        /// Set to `true` to use a subdirectory of the existing target directory or
        /// set to a path relative to the workspace to use that path.
        cargo_targetDir | rust_analyzerTargetDir: Option<TargetDirectory> = None,

        /// Run the check command for diagnostics on save.
        checkOnSave | checkOnSave_enable: bool                         = true,

        /// Check all targets and tests (`--all-targets`). Defaults to
        /// `#rust-analyzer.cargo.allTargets#`.
        check_allTargets | checkOnSave_allTargets: Option<bool>          = None,
        /// Cargo command to use for `cargo check`.
        check_command | checkOnSave_command: String                      = "check".to_owned(),
        /// Extra arguments for `cargo check`.
        check_extraArgs | checkOnSave_extraArgs: Vec<String>             = vec![],
        /// Extra environment variables that will be set when running `cargo check`.
        /// Extends `#rust-analyzer.cargo.extraEnv#`.
        check_extraEnv | checkOnSave_extraEnv: FxHashMap<String, String> = FxHashMap::default(),
        /// List of features to activate. Defaults to
        /// `#rust-analyzer.cargo.features#`.
        ///
        /// Set to `"all"` to pass `--all-features` to Cargo.
        check_features | checkOnSave_features: Option<CargoFeaturesDef>  = None,
        /// List of `cargo check` (or other command specified in `check.command`) diagnostics to ignore.
        ///
        /// For example for `cargo check`: `dead_code`, `unused_imports`, `unused_variables`,...
        check_ignore: FxHashSet<String> = FxHashSet::default(),
        /// Specifies the working directory for running checks.
        /// - "workspace": run checks for workspaces in the corresponding workspaces' root directories.
        // FIXME: Ideally we would support this in some way
        ///   This falls back to "root" if `#rust-analyzer.check.invocationStrategy#` is set to `once`.
        /// - "root": run checks in the project's root directory.
        /// This config only has an effect when `#rust-analyzer.check.overrideCommand#`
        /// is set.
        check_invocationLocation | checkOnSave_invocationLocation: InvocationLocation = InvocationLocation::Workspace,
        /// Specifies the invocation strategy to use when running the check command.
        /// If `per_workspace` is set, the command will be executed for each workspace.
        /// If `once` is set, the command will be executed once.
        /// This config only has an effect when `#rust-analyzer.check.overrideCommand#`
        /// is set.
        check_invocationStrategy | checkOnSave_invocationStrategy: InvocationStrategy = InvocationStrategy::PerWorkspace,
        /// Whether to pass `--no-default-features` to Cargo. Defaults to
        /// `#rust-analyzer.cargo.noDefaultFeatures#`.
        check_noDefaultFeatures | checkOnSave_noDefaultFeatures: Option<bool>         = None,
        /// Override the command rust-analyzer uses instead of `cargo check` for
        /// diagnostics on save. The command is required to output json and
        /// should therefore include `--message-format=json` or a similar option
        /// (if your client supports the `colorDiagnosticOutput` experimental
        /// capability, you can use `--message-format=json-diagnostic-rendered-ansi`).
        ///
        /// If you're changing this because you're using some tool wrapping
        /// Cargo, you might also want to change
        /// `#rust-analyzer.cargo.buildScripts.overrideCommand#`.
        ///
        /// If there are multiple linked projects/workspaces, this command is invoked for
        /// each of them, with the working directory being the workspace root
        /// (i.e., the folder containing the `Cargo.toml`). This can be overwritten
        /// by changing `#rust-analyzer.check.invocationStrategy#` and
        /// `#rust-analyzer.check.invocationLocation#`.
        ///
        /// If `$saved_file` is part of the command, rust-analyzer will pass
        /// the absolute path of the saved file to the provided command. This is
        /// intended to be used with non-Cargo build systems.
        /// Note that `$saved_file` is experimental and may be removed in the future.
        ///
        /// An example command would be:
        ///
        /// ```bash
        /// cargo check --workspace --message-format=json --all-targets
        /// ```
        /// .
        check_overrideCommand | checkOnSave_overrideCommand: Option<Vec<String>>             = None,
        /// Check for specific targets. Defaults to `#rust-analyzer.cargo.target#` if empty.
        ///
        /// Can be a single target, e.g. `"x86_64-unknown-linux-gnu"` or a list of targets, e.g.
        /// `["aarch64-apple-darwin", "x86_64-apple-darwin"]`.
        ///
        /// Aliased as `"checkOnSave.targets"`.
        check_targets | checkOnSave_targets | checkOnSave_target: Option<CheckOnSaveTargets> = None,
        /// Whether `--workspace` should be passed to `cargo check`.
        /// If false, `-p <package>` will be passed instead.
        check_workspace: bool = true,

        /// List of rust-analyzer diagnostics to disable.
        diagnostics_disabled: FxHashSet<String> = FxHashSet::default(),
        /// Whether to show native rust-analyzer diagnostics.
        diagnostics_enable: bool                = true,
        /// Whether to show experimental rust-analyzer diagnostics that might
        /// have more false positives than usual.
        diagnostics_experimental_enable: bool    = false,
        /// Map of prefixes to be substituted when parsing diagnostic file paths.
        /// This should be the reverse mapping of what is passed to `rustc` as `--remap-path-prefix`.
        diagnostics_remapPrefix: FxHashMap<String, String> = FxHashMap::default(),
        /// Whether to run additional style lints.
        diagnostics_styleLints_enable: bool =    false,
        /// List of warnings that should be displayed with hint severity.
        ///
        /// The warnings will be indicated by faded text or three dots in code
        /// and will not show up in the `Problems Panel`.
        diagnostics_warningsAsHint: Vec<String> = vec![],
        /// List of warnings that should be displayed with info severity.
        ///
        /// The warnings will be indicated by a blue squiggly underline in code
        /// and a blue icon in the `Problems Panel`.
        diagnostics_warningsAsInfo: Vec<String> = vec![],
        /// These directories will be ignored by rust-analyzer. They are
        /// relative to the workspace root, and globs are not supported. You may
        /// also need to add the folders to Code's `files.watcherExclude`.
        files_excludeDirs: Vec<Utf8PathBuf> = vec![],
        /// Controls file watching implementation.
        files_watcher: FilesWatcherDef = FilesWatcherDef::Client,

        /// Whether to show `Debug` action. Only applies when
        /// `#rust-analyzer.hover.actions.enable#` is set.
        hover_actions_debug_enable: bool           = true,
        /// Whether to show HoverActions in Rust files.
        hover_actions_enable: bool          = true,
        /// Whether to show `Go to Type Definition` action. Only applies when
        /// `#rust-analyzer.hover.actions.enable#` is set.
        hover_actions_gotoTypeDef_enable: bool     = true,
        /// Whether to show `Implementations` action. Only applies when
        /// `#rust-analyzer.hover.actions.enable#` is set.
        hover_actions_implementations_enable: bool = true,
        /// Whether to show `References` action. Only applies when
        /// `#rust-analyzer.hover.actions.enable#` is set.
        hover_actions_references_enable: bool      = false,
        /// Whether to show `Run` action. Only applies when
        /// `#rust-analyzer.hover.actions.enable#` is set.
        hover_actions_run_enable: bool             = true,

        /// Whether to show documentation on hover.
        hover_documentation_enable: bool           = true,
        /// Whether to show keyword hover popups. Only applies when
        /// `#rust-analyzer.hover.documentation.enable#` is set.
        hover_documentation_keywords_enable: bool  = true,
        /// Use markdown syntax for links on hover.
        hover_links_enable: bool = true,
        /// How to render the align information in a memory layout hover.
        hover_memoryLayout_alignment: Option<MemoryLayoutHoverRenderKindDef> = Some(MemoryLayoutHoverRenderKindDef::Hexadecimal),
        /// Whether to show memory layout data on hover.
        hover_memoryLayout_enable: bool = true,
        /// How to render the niche information in a memory layout hover.
        hover_memoryLayout_niches: Option<bool> = Some(false),
        /// How to render the offset information in a memory layout hover.
        hover_memoryLayout_offset: Option<MemoryLayoutHoverRenderKindDef> = Some(MemoryLayoutHoverRenderKindDef::Hexadecimal),
        /// How to render the size information in a memory layout hover.
        hover_memoryLayout_size: Option<MemoryLayoutHoverRenderKindDef> = Some(MemoryLayoutHoverRenderKindDef::Both),

        /// How many variants of an enum to display when hovering on. Show none if empty.
        hover_show_enumVariants: Option<usize> = Some(5),
        /// How many fields of a struct, variant or union to display when hovering on. Show none if empty.
        hover_show_fields: Option<usize> = Some(5),
        /// How many associated items of a trait to display when hovering a trait.
        hover_show_traitAssocItems: Option<usize> = None,

        /// Enables the experimental support for interpreting tests.
        interpret_tests: bool                                      = false,

        /// Whether to show `Debug` lens. Only applies when
        /// `#rust-analyzer.lens.enable#` is set.
        lens_debug_enable: bool            = true,
        /// Whether to show CodeLens in Rust files.
        lens_enable: bool           = true,
        /// Internal config: use custom client-side commands even when the
        /// client doesn't set the corresponding capability.
        lens_forceCustomCommands: bool = true,
        /// Whether to show `Implementations` lens. Only applies when
        /// `#rust-analyzer.lens.enable#` is set.
        lens_implementations_enable: bool  = true,
        /// Where to render annotations.
        lens_location: AnnotationLocation = AnnotationLocation::AboveName,
        /// Whether to show `References` lens for Struct, Enum, and Union.
        /// Only applies when `#rust-analyzer.lens.enable#` is set.
        lens_references_adt_enable: bool = false,
        /// Whether to show `References` lens for Enum Variants.
        /// Only applies when `#rust-analyzer.lens.enable#` is set.
        lens_references_enumVariant_enable: bool = false,
        /// Whether to show `Method References` lens. Only applies when
        /// `#rust-analyzer.lens.enable#` is set.
        lens_references_method_enable: bool = false,
        /// Whether to show `References` lens for Trait.
        /// Only applies when `#rust-analyzer.lens.enable#` is set.
        lens_references_trait_enable: bool = false,
        /// Whether to show `Run` lens. Only applies when
        /// `#rust-analyzer.lens.enable#` is set.
        lens_run_enable: bool              = true,

        /// Disable project auto-discovery in favor of explicitly specified set
        /// of projects.
        ///
        /// Elements must be paths pointing to `Cargo.toml`,
        /// `rust-project.json`, `.rs` files (which will be treated as standalone files) or JSON
        /// objects in `rust-project.json` format.
        linkedProjects: Vec<ManifestOrProjectJson> = vec![],

        /// Number of syntax trees rust-analyzer keeps in memory. Defaults to 128.
        lru_capacity: Option<usize>                 = None,
        /// Sets the LRU capacity of the specified queries.
        lru_query_capacities: FxHashMap<Box<str>, usize> = FxHashMap::default(),

        /// Whether to show `can't find Cargo.toml` error message.
        notifications_cargoTomlNotFound: bool      = true,

        /// Whether to send an UnindexedProject notification to the client.
        notifications_unindexedProject: bool      = false,

        /// How many worker threads in the main loop. The default `null` means to pick automatically.
        numThreads: Option<usize> = None,

        /// Expand attribute macros. Requires `#rust-analyzer.procMacro.enable#` to be set.
        procMacro_attributes_enable: bool = true,
        /// Enable support for procedural macros, implies `#rust-analyzer.cargo.buildScripts.enable#`.
        procMacro_enable: bool                     = true,
        /// These proc-macros will be ignored when trying to expand them.
        ///
        /// This config takes a map of crate names with the exported proc-macro names to ignore as values.
        procMacro_ignored: FxHashMap<Box<str>, Box<[Box<str>]>>          = FxHashMap::default(),
        /// Internal config, path to proc-macro server executable.
        procMacro_server: Option<Utf8PathBuf>          = None,

        /// Exclude imports from find-all-references.
        references_excludeImports: bool = false,

        /// Exclude tests from find-all-references.
        references_excludeTests: bool = false,

        /// Command to be executed instead of 'cargo' for runnables.
        runnables_command: Option<String> = None,
        /// Additional arguments to be passed to cargo for runnables such as
        /// tests or binaries. For example, it may be `--release`.
        runnables_extraArgs: Vec<String>   = vec![],
        /// Additional arguments to be passed through Cargo to launched tests, benchmarks, or
        /// doc-tests.
        ///
        /// Unless the launched target uses a
        /// [custom test harness](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-harness-field),
        /// they will end up being interpreted as options to
        /// [`rustc`’s built-in test harness (“libtest”)](https://doc.rust-lang.org/rustc/tests/index.html#cli-arguments).
        runnables_extraTestBinaryArgs: Vec<String> = vec!["--show-output".to_owned()],

        /// Path to the Cargo.toml of the rust compiler workspace, for usage in rustc_private
        /// projects, or "discover" to try to automatically find it if the `rustc-dev` component
        /// is installed.
        ///
        /// Any project which uses rust-analyzer with the rustcPrivate
        /// crates must set `[package.metadata.rust-analyzer] rustc_private=true` to use it.
        ///
        /// This option does not take effect until rust-analyzer is restarted.
        rustc_source: Option<String> = None,

        /// Additional arguments to `rustfmt`.
        rustfmt_extraArgs: Vec<String>               = vec![],
        /// Advanced option, fully override the command rust-analyzer uses for
        /// formatting. This should be the equivalent of `rustfmt` here, and
        /// not that of `cargo fmt`. The file contents will be passed on the
        /// standard input and the formatted result will be read from the
        /// standard output.
        rustfmt_overrideCommand: Option<Vec<String>> = None,
        /// Enables the use of rustfmt's unstable range formatting command for the
        /// `textDocument/rangeFormatting` request. The rustfmt option is unstable and only
        /// available on a nightly build.
        rustfmt_rangeFormatting_enable: bool = false,


        /// Show full signature of the callable. Only shows parameters if disabled.
        signatureInfo_detail: SignatureDetail                           = SignatureDetail::Full,
        /// Show documentation.
        signatureInfo_documentation_enable: bool                       = true,

        /// Whether to insert closing angle brackets when typing an opening angle bracket of a generic argument list.
        typing_autoClosingAngleBrackets_enable: bool = false,

        /// Workspace symbol search kind.
        workspace_symbol_search_kind: WorkspaceSymbolSearchKindDef = WorkspaceSymbolSearchKindDef::OnlyTypes,
        /// Limits the number of items returned from a workspace symbol search (Defaults to 128).
        /// Some clients like vs-code issue new searches on result filtering and don't require all results to be returned in the initial search.
        /// Other clients requires all results upfront and might require a higher limit.
        workspace_symbol_search_limit: usize = 128,
        /// Workspace symbol search scope.
        workspace_symbol_search_scope: WorkspaceSymbolSearchScopeDef = WorkspaceSymbolSearchScopeDef::Workspace,
    }
}

config_data! {
    /// Local configurations can be overridden for every crate by placing a `rust-analyzer.toml` on crate root.
    /// A config is searched for by traversing a "config tree" in a bottom up fashion. It is chosen by the nearest first principle.
    local: struct LocalDefaultConfigData <- LocalConfigInput ->  {
        /// Term search fuel in "units of work" for assists (Defaults to 400).
        assist_termSearch_fuel: usize = 400,

        /// Toggles the additional completions that automatically add imports when completed.
        /// Note that your client must specify the `additionalTextEdits` LSP client capability to truly have this feature enabled.
        completion_autoimport_enable: bool       = true,
        /// Toggles the additional completions that automatically show method calls and field accesses
        /// with `self` prefixed to them when inside a method.
        completion_autoself_enable: bool        = true,
        /// Whether to add parenthesis and argument snippets when completing function.
        completion_callable_snippets: CallableCompletionDef  = CallableCompletionDef::FillArguments,
        /// Whether to show full function/method signatures in completion docs.
        completion_fullFunctionSignatures_enable: bool = false,
        /// Maximum number of completions to return. If `None`, the limit is infinite.
        completion_limit: Option<usize> = None,
        /// Whether to show postfix snippets like `dbg`, `if`, `not`, etc.
        completion_postfix_enable: bool         = true,
        /// Enables completions of private items and fields that are defined in the current workspace even if they are not visible at the current position.
        completion_privateEditable_enable: bool = false,
        /// Custom completion snippets.
        // NOTE: we use IndexMap for deterministic serialization ordering
        completion_snippets_custom: IndexMap<String, SnippetDef> = serde_json::from_str(r#"{
            "Arc::new": {
                "postfix": "arc",
                "body": "Arc::new(${receiver})",
                "requires": "std::sync::Arc",
                "description": "Put the expression into an `Arc`",
                "scope": "expr"
            },
            "Rc::new": {
                "postfix": "rc",
                "body": "Rc::new(${receiver})",
                "requires": "std::rc::Rc",
                "description": "Put the expression into an `Rc`",
                "scope": "expr"
            },
            "Box::pin": {
                "postfix": "pinbox",
                "body": "Box::pin(${receiver})",
                "requires": "std::boxed::Box",
                "description": "Put the expression into a pinned `Box`",
                "scope": "expr"
            },
            "Ok": {
                "postfix": "ok",
                "body": "Ok(${receiver})",
                "description": "Wrap the expression in a `Result::Ok`",
                "scope": "expr"
            },
            "Err": {
                "postfix": "err",
                "body": "Err(${receiver})",
                "description": "Wrap the expression in a `Result::Err`",
                "scope": "expr"
            },
            "Some": {
                "postfix": "some",
                "body": "Some(${receiver})",
                "description": "Wrap the expression in an `Option::Some`",
                "scope": "expr"
            }
        }"#).unwrap(),
        /// Whether to enable term search based snippets like `Some(foo.bar().baz())`.
        completion_termSearch_enable: bool = false,
        /// Term search fuel in "units of work" for autocompletion (Defaults to 200).
        completion_termSearch_fuel: usize = 200,

        /// Enables highlighting of related references while the cursor is on `break`, `loop`, `while`, or `for` keywords.
        highlightRelated_breakPoints_enable: bool = true,
        /// Enables highlighting of all captures of a closure while the cursor is on the `|` or move keyword of a closure.
        highlightRelated_closureCaptures_enable: bool = true,
        /// Enables highlighting of all exit points while the cursor is on any `return`, `?`, `fn`, or return type arrow (`->`).
        highlightRelated_exitPoints_enable: bool = true,
        /// Enables highlighting of related references while the cursor is on any identifier.
        highlightRelated_references_enable: bool = true,
        /// Enables highlighting of all break points for a loop or block context while the cursor is on any `async` or `await` keywords.
        highlightRelated_yieldPoints_enable: bool = true,

        /// Whether to enforce the import granularity setting for all files. If set to false rust-analyzer will try to keep import styles consistent per file.
        imports_granularity_enforce: bool              = false,
        /// How imports should be grouped into use statements.
        imports_granularity_group: ImportGranularityDef  = ImportGranularityDef::Crate,
        /// Group inserted imports by the https://rust-analyzer.github.io/manual.html#auto-import[following order]. Groups are separated by newlines.
        imports_group_enable: bool                           = true,
        /// Whether to allow import insertion to merge new imports into single path glob imports like `use std::fmt::*;`.
        imports_merge_glob: bool           = true,
        /// Prefer to unconditionally use imports of the core and alloc crate, over the std crate.
        imports_preferNoStd | imports_prefer_no_std: bool = false,
         /// Whether to prefer import paths containing a `prelude` module.
        imports_preferPrelude: bool                       = false,
        /// The path structure for newly inserted paths to use.
        imports_prefix: ImportPrefixDef               = ImportPrefixDef::Plain,


        /// Whether to show inlay type hints for binding modes.
        inlayHints_bindingModeHints_enable: bool                   = false,
        /// Whether to show inlay type hints for method chains.
        inlayHints_chainingHints_enable: bool                      = true,
        /// Whether to show inlay hints after a closing `}` to indicate what item it belongs to.
        inlayHints_closingBraceHints_enable: bool                  = true,
        /// Minimum number of lines required before the `}` until the hint is shown (set to 0 or 1
        /// to always show them).
        inlayHints_closingBraceHints_minLines: usize               = 25,
        /// Whether to show inlay hints for closure captures.
        inlayHints_closureCaptureHints_enable: bool                          = false,
        /// Whether to show inlay type hints for return types of closures.
        inlayHints_closureReturnTypeHints_enable: ClosureReturnTypeHintsDef  = ClosureReturnTypeHintsDef::Never,
        /// Closure notation in type and chaining inlay hints.
        inlayHints_closureStyle: ClosureStyle                                = ClosureStyle::ImplFn,
        /// Whether to show enum variant discriminant hints.
        inlayHints_discriminantHints_enable: DiscriminantHintsDef            = DiscriminantHintsDef::Never,
        /// Whether to show inlay hints for type adjustments.
        inlayHints_expressionAdjustmentHints_enable: AdjustmentHintsDef = AdjustmentHintsDef::Never,
        /// Whether to hide inlay hints for type adjustments outside of `unsafe` blocks.
        inlayHints_expressionAdjustmentHints_hideOutsideUnsafe: bool = false,
        /// Whether to show inlay hints as postfix ops (`.*` instead of `*`, etc).
        inlayHints_expressionAdjustmentHints_mode: AdjustmentHintsModeDef = AdjustmentHintsModeDef::Prefix,
        /// Whether to show implicit drop hints.
        inlayHints_implicitDrops_enable: bool                      = false,
        /// Whether to show inlay type hints for elided lifetimes in function signatures.
        inlayHints_lifetimeElisionHints_enable: LifetimeElisionDef = LifetimeElisionDef::Never,
        /// Whether to prefer using parameter names as the name for elided lifetime hints if possible.
        inlayHints_lifetimeElisionHints_useParameterNames: bool    = false,
        /// Maximum length for inlay hints. Set to null to have an unlimited length.
        inlayHints_maxLength: Option<usize>                        = Some(25),
        /// Whether to show function parameter name inlay hints at the call
        /// site.
        inlayHints_parameterHints_enable: bool                     = true,
        /// Whether to show exclusive range inlay hints.
        inlayHints_rangeExclusiveHints_enable: bool                = false,
        /// Whether to show inlay hints for compiler inserted reborrows.
        /// This setting is deprecated in favor of #rust-analyzer.inlayHints.expressionAdjustmentHints.enable#.
        inlayHints_reborrowHints_enable: ReborrowHintsDef          = ReborrowHintsDef::Never,
        /// Whether to render leading colons for type hints, and trailing colons for parameter hints.
        inlayHints_renderColons: bool                              = true,
        /// Whether to show inlay type hints for variables.
        inlayHints_typeHints_enable: bool                          = true,
        /// Whether to hide inlay type hints for `let` statements that initialize to a closure.
        /// Only applies to closures with blocks, same as `#rust-analyzer.inlayHints.closureReturnTypeHints.enable#`.
        inlayHints_typeHints_hideClosureInitialization: bool       = false,
        /// Whether to hide inlay type hints for constructors.
        inlayHints_typeHints_hideNamedConstructor: bool            = false,


        /// Join lines merges consecutive declaration and initialization of an assignment.
        joinLines_joinAssignments: bool = true,
        /// Join lines inserts else between consecutive ifs.
        joinLines_joinElseIf: bool = true,
        /// Join lines removes trailing commas.
        joinLines_removeTrailingComma: bool = true,
        /// Join lines unwraps trivial blocks.
        joinLines_unwrapTrivialBlock: bool = true,

        /// Inject additional highlighting into doc comments.
        ///
        /// When enabled, rust-analyzer will highlight rust source in doc comments as well as intra
        /// doc links.
        semanticHighlighting_doc_comment_inject_enable: bool = true,
        /// Whether the server is allowed to emit non-standard tokens and modifiers.
        semanticHighlighting_nonStandardTokens: bool = true,
        /// Use semantic tokens for operators.
        ///
        /// When disabled, rust-analyzer will emit semantic tokens only for operator tokens when
        /// they are tagged with modifiers.
        semanticHighlighting_operator_enable: bool = true,
        /// Use specialized semantic tokens for operators.
        ///
        /// When enabled, rust-analyzer will emit special token types for operator tokens instead
        /// of the generic `operator` token type.
        semanticHighlighting_operator_specialization_enable: bool = false,
        /// Use semantic tokens for punctuation.
        ///
        /// When disabled, rust-analyzer will emit semantic tokens only for punctuation tokens when
        /// they are tagged with modifiers or have a special role.
        semanticHighlighting_punctuation_enable: bool = false,
        /// When enabled, rust-analyzer will emit a punctuation semantic token for the `!` of macro
        /// calls.
        semanticHighlighting_punctuation_separate_macro_bang: bool = false,
        /// Use specialized semantic tokens for punctuation.
        ///
        /// When enabled, rust-analyzer will emit special token types for punctuation tokens instead
        /// of the generic `punctuation` token type.
        semanticHighlighting_punctuation_specialization_enable: bool = false,
        /// Use semantic tokens for strings.
        ///
        /// In some editors (e.g. vscode) semantic tokens override other highlighting grammars.
        /// By disabling semantic tokens for strings, other grammars can be used to highlight
        /// their contents.
        semanticHighlighting_strings_enable: bool = true,
    }
}

config_data! {
    /// Configs that only make sense when they are set by a client. As such they can only be defined
    /// by setting them using client's settings (e.g `settings.json` on VS Code).
    client: struct ClientDefaultConfigData <- ClientConfigInput -> {}
}

#[derive(Debug, Clone)]
pub struct Config {
    discovered_projects: Vec<ProjectManifest>,
    /// The workspace roots as registered by the LSP client
    workspace_roots: Vec<AbsPathBuf>,
    caps: lsp_types::ClientCapabilities,
    root_path: AbsPathBuf,
    detached_files: Vec<AbsPathBuf>,
    snippets: Vec<Snippet>,
    visual_studio_code_version: Option<Version>,

    default_config: DefaultConfigData,
    client_config: FullConfigInput,
    user_config: GlobalLocalConfigInput,
    #[allow(dead_code)]
    ratoml_files: FxHashMap<SourceRootId, RatomlNode>,
}

#[derive(Clone, Debug)]
struct RatomlNode {
    #[allow(dead_code)]
    node: GlobalLocalConfigInput,
    #[allow(dead_code)]
    parent: Option<SourceRootId>,
}

macro_rules! try_ {
    ($expr:expr) => {
        || -> _ { Some($expr) }()
    };
}
macro_rules! try_or {
    ($expr:expr, $or:expr) => {
        try_!($expr).unwrap_or($or)
    };
}

macro_rules! try_or_def {
    ($expr:expr) => {
        try_!($expr).unwrap_or_default()
    };
}

type ParallelCachePrimingNumThreads = u8;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LinkedProject {
    ProjectManifest(ProjectManifest),
    InlineJsonProject(ProjectJson),
}

impl From<ProjectManifest> for LinkedProject {
    fn from(v: ProjectManifest) -> Self {
        LinkedProject::ProjectManifest(v)
    }
}

impl From<ProjectJson> for LinkedProject {
    fn from(v: ProjectJson) -> Self {
        LinkedProject::InlineJsonProject(v)
    }
}

pub struct CallInfoConfig {
    pub params_only: bool,
    pub docs: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LensConfig {
    // runnables
    pub run: bool,
    pub debug: bool,
    pub interpret: bool,

    // implementations
    pub implementations: bool,

    // references
    pub method_refs: bool,
    pub refs_adt: bool,   // for Struct, Enum, Union and Trait
    pub refs_trait: bool, // for Struct, Enum, Union and Trait
    pub enum_variant_refs: bool,

    // annotations
    pub location: AnnotationLocation,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationLocation {
    AboveName,
    AboveWholeItem,
}

impl From<AnnotationLocation> for ide::AnnotationLocation {
    fn from(location: AnnotationLocation) -> Self {
        match location {
            AnnotationLocation::AboveName => ide::AnnotationLocation::AboveName,
            AnnotationLocation::AboveWholeItem => ide::AnnotationLocation::AboveWholeItem,
        }
    }
}

impl LensConfig {
    pub fn any(&self) -> bool {
        self.run
            || self.debug
            || self.implementations
            || self.method_refs
            || self.refs_adt
            || self.refs_trait
            || self.enum_variant_refs
    }

    pub fn none(&self) -> bool {
        !self.any()
    }

    pub fn runnable(&self) -> bool {
        self.run || self.debug
    }

    pub fn references(&self) -> bool {
        self.method_refs || self.refs_adt || self.refs_trait || self.enum_variant_refs
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverActionsConfig {
    pub implementations: bool,
    pub references: bool,
    pub run: bool,
    pub debug: bool,
    pub goto_type_def: bool,
}

impl HoverActionsConfig {
    pub const NO_ACTIONS: Self = Self {
        implementations: false,
        references: false,
        run: false,
        debug: false,
        goto_type_def: false,
    };

    pub fn any(&self) -> bool {
        self.implementations || self.references || self.runnable() || self.goto_type_def
    }

    pub fn none(&self) -> bool {
        !self.any()
    }

    pub fn runnable(&self) -> bool {
        self.run || self.debug
    }
}

#[derive(Debug, Clone)]
pub struct FilesConfig {
    pub watcher: FilesWatcher,
    pub exclude: Vec<AbsPathBuf>,
}

#[derive(Debug, Clone)]
pub enum FilesWatcher {
    Client,
    Server,
}

#[derive(Debug, Clone)]
pub struct NotificationsConfig {
    pub cargo_toml_not_found: bool,
    pub unindexed_project: bool,
}

#[derive(Debug, Clone)]
pub enum RustfmtConfig {
    Rustfmt { extra_args: Vec<String>, enable_range_formatting: bool },
    CustomCommand { command: String, args: Vec<String> },
}

/// Configuration for runnable items, such as `main` function or tests.
#[derive(Debug, Clone)]
pub struct RunnablesConfig {
    /// Custom command to be executed instead of `cargo` for runnables.
    pub override_cargo: Option<String>,
    /// Additional arguments for the `cargo`, e.g. `--release`.
    pub cargo_extra_args: Vec<String>,
    /// Additional arguments for the binary being run, if it is a test or benchmark.
    pub extra_test_binary_args: Vec<String>,
}

/// Configuration for workspace symbol search requests.
#[derive(Debug, Clone)]
pub struct WorkspaceSymbolConfig {
    /// In what scope should the symbol be searched in.
    pub search_scope: WorkspaceSymbolSearchScope,
    /// What kind of symbol is being searched for.
    pub search_kind: WorkspaceSymbolSearchKind,
    /// How many items are returned at most.
    pub search_limit: usize,
}

pub struct ClientCommandsConfig {
    pub run_single: bool,
    pub debug_single: bool,
    pub show_reference: bool,
    pub goto_location: bool,
    pub trigger_parameter_hints: bool,
}

#[derive(Debug)]
pub struct ConfigError {
    errors: Vec<(String, serde_json::Error)>,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errors = self.errors.iter().format_with("\n", |(key, e), f| {
            f(key)?;
            f(&": ")?;
            f(e)
        });
        write!(
            f,
            "invalid config value{}:\n{}",
            if self.errors.len() == 1 { "" } else { "s" },
            errors
        )
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn new(
        root_path: AbsPathBuf,
        caps: ClientCapabilities,
        workspace_roots: Vec<AbsPathBuf>,
        visual_studio_code_version: Option<Version>,
    ) -> Self {
        Config {
            caps,
            detached_files: Vec::new(),
            discovered_projects: Vec::new(),
            root_path,
            snippets: Default::default(),
            workspace_roots,
            visual_studio_code_version,
            client_config: FullConfigInput::default(),
            user_config: GlobalLocalConfigInput::default(),
            ratoml_files: FxHashMap::default(),
            default_config: DefaultConfigData::default(),
        }
    }

    pub fn rediscover_workspaces(&mut self) {
        let discovered = ProjectManifest::discover_all(&self.workspace_roots);
        tracing::info!("discovered projects: {:?}", discovered);
        if discovered.is_empty() {
            tracing::error!("failed to find any projects in {:?}", &self.workspace_roots);
        }
        self.discovered_projects = discovered;
    }

    pub fn remove_workspace(&mut self, path: &AbsPath) {
        if let Some(position) = self.workspace_roots.iter().position(|it| it == path) {
            self.workspace_roots.remove(position);
        }
    }

    pub fn add_workspaces(&mut self, paths: impl Iterator<Item = AbsPathBuf>) {
        self.workspace_roots.extend(paths);
    }

    pub fn update(&mut self, mut json: serde_json::Value) -> Result<(), ConfigError> {
        tracing::info!("updating config from JSON: {:#}", json);
        if json.is_null() || json.as_object().map_or(false, |it| it.is_empty()) {
            return Ok(());
        }
        let mut errors = Vec::new();
        self.detached_files =
            get_field::<Vec<Utf8PathBuf>>(&mut json, &mut errors, "detachedFiles", None)
                .unwrap_or_default()
                .into_iter()
                .map(AbsPathBuf::assert)
                .collect();
        patch_old_style::patch_json_for_outdated_configs(&mut json);
        self.client_config = FullConfigInput::from_json(json, &mut errors);
        tracing::debug!(?self.client_config, "deserialized config data");
        self.snippets.clear();

        let snips = self.completion_snippets_custom(None).to_owned();

        for (name, def) in snips.iter() {
            if def.prefix.is_empty() && def.postfix.is_empty() {
                continue;
            }
            let scope = match def.scope {
                SnippetScopeDef::Expr => SnippetScope::Expr,
                SnippetScopeDef::Type => SnippetScope::Type,
                SnippetScopeDef::Item => SnippetScope::Item,
            };
            match Snippet::new(
                &def.prefix,
                &def.postfix,
                &def.body,
                def.description.as_ref().unwrap_or(name),
                &def.requires,
                scope,
            ) {
                Some(snippet) => self.snippets.push(snippet),
                None => errors.push((
                    format!("snippet {name} is invalid"),
                    <serde_json::Error as serde::de::Error>::custom(
                        "snippet path is invalid or triggers are missing",
                    ),
                )),
            }
        }

        self.validate(&mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ConfigError { errors })
        }
    }

    fn validate(&self, error_sink: &mut Vec<(String, serde_json::Error)>) {
        use serde::de::Error;
        if self.check_command().is_empty() {
            error_sink.push((
                "/check/command".to_owned(),
                serde_json::Error::custom("expected a non-empty string"),
            ));
        }
    }

    pub fn json_schema() -> serde_json::Value {
        FullConfigInput::json_schema()
    }

    pub fn root_path(&self) -> &AbsPathBuf {
        &self.root_path
    }

    pub fn caps(&self) -> &lsp_types::ClientCapabilities {
        &self.caps
    }

    pub fn detached_files(&self) -> &[AbsPathBuf] {
        &self.detached_files
    }
}

impl Config {
    pub fn assist(&self, source_root: Option<SourceRootId>) -> AssistConfig {
        AssistConfig {
            snippet_cap: SnippetCap::new(self.experimental("snippetTextEdit")),
            allowed: None,
            insert_use: self.insert_use_config(source_root),
            prefer_no_std: self.imports_preferNoStd(source_root).to_owned(),
            assist_emit_must_use: self.assist_emitMustUse().to_owned(),
            prefer_prelude: self.imports_preferPrelude(source_root).to_owned(),
            term_search_fuel: self.assist_termSearch_fuel(source_root).to_owned() as u64,
        }
    }

    pub fn completion(&self, source_root: Option<SourceRootId>) -> CompletionConfig {
        CompletionConfig {
            enable_postfix_completions: self.completion_postfix_enable(source_root).to_owned(),
            enable_imports_on_the_fly: self.completion_autoimport_enable(source_root).to_owned()
                && completion_item_edit_resolve(&self.caps),
            enable_self_on_the_fly: self.completion_autoself_enable(source_root).to_owned(),
            enable_private_editable: self.completion_privateEditable_enable(source_root).to_owned(),
            enable_term_search: self.completion_termSearch_enable(source_root).to_owned(),
            term_search_fuel: self.completion_termSearch_fuel(source_root).to_owned() as u64,
            full_function_signatures: self
                .completion_fullFunctionSignatures_enable(source_root)
                .to_owned(),
            callable: match self.completion_callable_snippets(source_root) {
                CallableCompletionDef::FillArguments => Some(CallableSnippets::FillArguments),
                CallableCompletionDef::AddParentheses => Some(CallableSnippets::AddParentheses),
                CallableCompletionDef::None => None,
            },
            snippet_cap: SnippetCap::new(try_or_def!(
                self.caps
                    .text_document
                    .as_ref()?
                    .completion
                    .as_ref()?
                    .completion_item
                    .as_ref()?
                    .snippet_support?
            )),
            insert_use: self.insert_use_config(source_root),
            prefer_no_std: self.imports_preferNoStd(source_root).to_owned(),
            prefer_prelude: self.imports_preferPrelude(source_root).to_owned(),
            snippets: self.snippets.clone().to_vec(),
            limit: self.completion_limit(source_root).to_owned(),
        }
    }

    pub fn diagnostics(&self, source_root: Option<SourceRootId>) -> DiagnosticsConfig {
        DiagnosticsConfig {
            enabled: *self.diagnostics_enable(),
            proc_attr_macros_enabled: self.expand_proc_attr_macros(),
            proc_macros_enabled: *self.procMacro_enable(),
            disable_experimental: !self.diagnostics_experimental_enable(),
            disabled: self.diagnostics_disabled().clone(),
            expr_fill_default: match self.assist_expressionFillDefault() {
                ExprFillDefaultDef::Todo => ExprFillDefaultMode::Todo,
                ExprFillDefaultDef::Default => ExprFillDefaultMode::Default,
            },
            insert_use: self.insert_use_config(source_root),
            prefer_no_std: self.imports_preferNoStd(source_root).to_owned(),
            prefer_prelude: self.imports_preferPrelude(source_root).to_owned(),
            style_lints: self.diagnostics_styleLints_enable().to_owned(),
            term_search_fuel: self.assist_termSearch_fuel(source_root).to_owned() as u64,
        }
    }
    pub fn expand_proc_attr_macros(&self) -> bool {
        self.procMacro_enable().to_owned() && self.procMacro_attributes_enable().to_owned()
    }

    pub fn highlight_related(&self, source_root: Option<SourceRootId>) -> HighlightRelatedConfig {
        HighlightRelatedConfig {
            references: self.highlightRelated_references_enable(source_root).to_owned(),
            break_points: self.highlightRelated_breakPoints_enable(source_root).to_owned(),
            exit_points: self.highlightRelated_exitPoints_enable(source_root).to_owned(),
            yield_points: self.highlightRelated_yieldPoints_enable(source_root).to_owned(),
            closure_captures: self.highlightRelated_closureCaptures_enable(source_root).to_owned(),
        }
    }

    pub fn hover_actions(&self) -> HoverActionsConfig {
        let enable = self.experimental("hoverActions") && self.hover_actions_enable().to_owned();
        HoverActionsConfig {
            implementations: enable && self.hover_actions_implementations_enable().to_owned(),
            references: enable && self.hover_actions_references_enable().to_owned(),
            run: enable && self.hover_actions_run_enable().to_owned(),
            debug: enable && self.hover_actions_debug_enable().to_owned(),
            goto_type_def: enable && self.hover_actions_gotoTypeDef_enable().to_owned(),
        }
    }

    pub fn hover(&self) -> HoverConfig {
        let mem_kind = |kind| match kind {
            MemoryLayoutHoverRenderKindDef::Both => MemoryLayoutHoverRenderKind::Both,
            MemoryLayoutHoverRenderKindDef::Decimal => MemoryLayoutHoverRenderKind::Decimal,
            MemoryLayoutHoverRenderKindDef::Hexadecimal => MemoryLayoutHoverRenderKind::Hexadecimal,
        };
        HoverConfig {
            links_in_hover: self.hover_links_enable().to_owned(),
            memory_layout: self.hover_memoryLayout_enable().then_some(MemoryLayoutHoverConfig {
                size: self.hover_memoryLayout_size().map(mem_kind),
                offset: self.hover_memoryLayout_offset().map(mem_kind),
                alignment: self.hover_memoryLayout_alignment().map(mem_kind),
                niches: self.hover_memoryLayout_niches().unwrap_or_default(),
            }),
            documentation: self.hover_documentation_enable().to_owned(),
            format: {
                let is_markdown = try_or_def!(self
                    .caps
                    .text_document
                    .as_ref()?
                    .hover
                    .as_ref()?
                    .content_format
                    .as_ref()?
                    .as_slice())
                .contains(&MarkupKind::Markdown);
                if is_markdown {
                    HoverDocFormat::Markdown
                } else {
                    HoverDocFormat::PlainText
                }
            },
            keywords: self.hover_documentation_keywords_enable().to_owned(),
            max_trait_assoc_items_count: self.hover_show_traitAssocItems().to_owned(),
            max_fields_count: self.hover_show_fields().to_owned(),
            max_enum_variants_count: self.hover_show_enumVariants().to_owned(),
        }
    }

    pub fn inlay_hints(&self, source_root: Option<SourceRootId>) -> InlayHintsConfig {
        let client_capability_fields = self
            .caps
            .text_document
            .as_ref()
            .and_then(|text| text.inlay_hint.as_ref())
            .and_then(|inlay_hint_caps| inlay_hint_caps.resolve_support.as_ref())
            .map(|inlay_resolve| inlay_resolve.properties.iter())
            .into_iter()
            .flatten()
            .cloned()
            .collect::<FxHashSet<_>>();

        InlayHintsConfig {
            render_colons: self.inlayHints_renderColons(source_root).to_owned(),
            type_hints: self.inlayHints_typeHints_enable(source_root).to_owned(),
            parameter_hints: self.inlayHints_parameterHints_enable(source_root).to_owned(),
            chaining_hints: self.inlayHints_chainingHints_enable(source_root).to_owned(),
            discriminant_hints: match self.inlayHints_discriminantHints_enable(source_root) {
                DiscriminantHintsDef::Always => ide::DiscriminantHints::Always,
                DiscriminantHintsDef::Never => ide::DiscriminantHints::Never,
                DiscriminantHintsDef::Fieldless => ide::DiscriminantHints::Fieldless,
            },
            closure_return_type_hints: match self
                .inlayHints_closureReturnTypeHints_enable(source_root)
            {
                ClosureReturnTypeHintsDef::Always => ide::ClosureReturnTypeHints::Always,
                ClosureReturnTypeHintsDef::Never => ide::ClosureReturnTypeHints::Never,
                ClosureReturnTypeHintsDef::WithBlock => ide::ClosureReturnTypeHints::WithBlock,
            },
            lifetime_elision_hints: match self.inlayHints_lifetimeElisionHints_enable(source_root) {
                LifetimeElisionDef::Always => ide::LifetimeElisionHints::Always,
                LifetimeElisionDef::Never => ide::LifetimeElisionHints::Never,
                LifetimeElisionDef::SkipTrivial => ide::LifetimeElisionHints::SkipTrivial,
            },
            hide_named_constructor_hints: self
                .inlayHints_typeHints_hideNamedConstructor(source_root)
                .to_owned(),
            hide_closure_initialization_hints: self
                .inlayHints_typeHints_hideClosureInitialization(source_root)
                .to_owned(),
            closure_style: match self.inlayHints_closureStyle(source_root) {
                ClosureStyle::ImplFn => hir::ClosureStyle::ImplFn,
                ClosureStyle::RustAnalyzer => hir::ClosureStyle::RANotation,
                ClosureStyle::WithId => hir::ClosureStyle::ClosureWithId,
                ClosureStyle::Hide => hir::ClosureStyle::Hide,
            },
            closure_capture_hints: self
                .inlayHints_closureCaptureHints_enable(source_root)
                .to_owned(),
            adjustment_hints: match self.inlayHints_expressionAdjustmentHints_enable(source_root) {
                AdjustmentHintsDef::Always => ide::AdjustmentHints::Always,
                AdjustmentHintsDef::Never => {
                    match self.inlayHints_reborrowHints_enable(source_root) {
                        ReborrowHintsDef::Always | ReborrowHintsDef::Mutable => {
                            ide::AdjustmentHints::ReborrowOnly
                        }
                        ReborrowHintsDef::Never => ide::AdjustmentHints::Never,
                    }
                }
                AdjustmentHintsDef::Reborrow => ide::AdjustmentHints::ReborrowOnly,
            },
            adjustment_hints_mode: match self.inlayHints_expressionAdjustmentHints_mode(source_root)
            {
                AdjustmentHintsModeDef::Prefix => ide::AdjustmentHintsMode::Prefix,
                AdjustmentHintsModeDef::Postfix => ide::AdjustmentHintsMode::Postfix,
                AdjustmentHintsModeDef::PreferPrefix => ide::AdjustmentHintsMode::PreferPrefix,
                AdjustmentHintsModeDef::PreferPostfix => ide::AdjustmentHintsMode::PreferPostfix,
            },
            adjustment_hints_hide_outside_unsafe: self
                .inlayHints_expressionAdjustmentHints_hideOutsideUnsafe(source_root)
                .to_owned(),
            binding_mode_hints: self.inlayHints_bindingModeHints_enable(source_root).to_owned(),
            param_names_for_lifetime_elision_hints: self
                .inlayHints_lifetimeElisionHints_useParameterNames(source_root)
                .to_owned(),
            max_length: self.inlayHints_maxLength(source_root).to_owned(),
            closing_brace_hints_min_lines: if self
                .inlayHints_closingBraceHints_enable(source_root)
                .to_owned()
            {
                Some(self.inlayHints_closingBraceHints_minLines(source_root).to_owned())
            } else {
                None
            },
            fields_to_resolve: InlayFieldsToResolve {
                resolve_text_edits: client_capability_fields.contains("textEdits"),
                resolve_hint_tooltip: client_capability_fields.contains("tooltip"),
                resolve_label_tooltip: client_capability_fields.contains("label.tooltip"),
                resolve_label_location: client_capability_fields.contains("label.location"),
                resolve_label_command: client_capability_fields.contains("label.command"),
            },
            implicit_drop_hints: self.inlayHints_implicitDrops_enable(source_root).to_owned(),
            range_exclusive_hints: self
                .inlayHints_rangeExclusiveHints_enable(source_root)
                .to_owned(),
        }
    }

    fn insert_use_config(&self, source_root: Option<SourceRootId>) -> InsertUseConfig {
        InsertUseConfig {
            granularity: match self.imports_granularity_group(source_root) {
                ImportGranularityDef::Preserve => ImportGranularity::Preserve,
                ImportGranularityDef::Item => ImportGranularity::Item,
                ImportGranularityDef::Crate => ImportGranularity::Crate,
                ImportGranularityDef::Module => ImportGranularity::Module,
                ImportGranularityDef::One => ImportGranularity::One,
            },
            enforce_granularity: self.imports_granularity_enforce(source_root).to_owned(),
            prefix_kind: match self.imports_prefix(source_root) {
                ImportPrefixDef::Plain => PrefixKind::Plain,
                ImportPrefixDef::ByCrate => PrefixKind::ByCrate,
                ImportPrefixDef::BySelf => PrefixKind::BySelf,
            },
            group: self.imports_group_enable(source_root).to_owned(),
            skip_glob_imports: !self.imports_merge_glob(source_root),
        }
    }

    pub fn join_lines(&self, source_root: Option<SourceRootId>) -> JoinLinesConfig {
        JoinLinesConfig {
            join_else_if: self.joinLines_joinElseIf(source_root).to_owned(),
            remove_trailing_comma: self.joinLines_removeTrailingComma(source_root).to_owned(),
            unwrap_trivial_blocks: self.joinLines_unwrapTrivialBlock(source_root).to_owned(),
            join_assignments: self.joinLines_joinAssignments(source_root).to_owned(),
        }
    }

    pub fn highlighting_non_standard_tokens(&self, source_root: Option<SourceRootId>) -> bool {
        self.semanticHighlighting_nonStandardTokens(source_root).to_owned()
    }

    pub fn highlighting_config(&self, source_root: Option<SourceRootId>) -> HighlightConfig {
        HighlightConfig {
            strings: self.semanticHighlighting_strings_enable(source_root).to_owned(),
            punctuation: self.semanticHighlighting_punctuation_enable(source_root).to_owned(),
            specialize_punctuation: self
                .semanticHighlighting_punctuation_specialization_enable(source_root)
                .to_owned(),
            macro_bang: self
                .semanticHighlighting_punctuation_separate_macro_bang(source_root)
                .to_owned(),
            operator: self.semanticHighlighting_operator_enable(source_root).to_owned(),
            specialize_operator: self
                .semanticHighlighting_operator_specialization_enable(source_root)
                .to_owned(),
            inject_doc_comment: self
                .semanticHighlighting_doc_comment_inject_enable(source_root)
                .to_owned(),
            syntactic_name_ref_highlighting: false,
        }
    }

    pub fn has_linked_projects(&self) -> bool {
        !self.linkedProjects().is_empty()
    }
    pub fn linked_manifests(&self) -> impl Iterator<Item = &Utf8Path> + '_ {
        self.linkedProjects().iter().filter_map(|it| match it {
            ManifestOrProjectJson::Manifest(p) => Some(&**p),
            ManifestOrProjectJson::ProjectJson(_) => None,
        })
    }
    pub fn has_linked_project_jsons(&self) -> bool {
        self.linkedProjects().iter().any(|it| matches!(it, ManifestOrProjectJson::ProjectJson(_)))
    }
    pub fn linked_or_discovered_projects(&self) -> Vec<LinkedProject> {
        match self.linkedProjects().as_slice() {
            [] => {
                let exclude_dirs: Vec<_> =
                    self.files_excludeDirs().iter().map(|p| self.root_path.join(p)).collect();
                self.discovered_projects
                    .iter()
                    .filter(|project| {
                        !exclude_dirs.iter().any(|p| project.manifest_path().starts_with(p))
                    })
                    .cloned()
                    .map(LinkedProject::from)
                    .collect()
            }
            linked_projects => linked_projects
                .iter()
                .filter_map(|linked_project| match linked_project {
                    ManifestOrProjectJson::Manifest(it) => {
                        let path = self.root_path.join(it);
                        ProjectManifest::from_manifest_file(path)
                            .map_err(|e| tracing::error!("failed to load linked project: {}", e))
                            .ok()
                            .map(Into::into)
                    }
                    ManifestOrProjectJson::ProjectJson(it) => {
                        Some(ProjectJson::new(None, &self.root_path, it.clone()).into())
                    }
                })
                .collect(),
        }
    }

    pub fn did_save_text_document_dynamic_registration(&self) -> bool {
        let caps = try_or_def!(self.caps.text_document.as_ref()?.synchronization.clone()?);
        caps.did_save == Some(true) && caps.dynamic_registration == Some(true)
    }

    pub fn did_change_watched_files_dynamic_registration(&self) -> bool {
        try_or_def!(
            self.caps.workspace.as_ref()?.did_change_watched_files.as_ref()?.dynamic_registration?
        )
    }

    pub fn did_change_watched_files_relative_pattern_support(&self) -> bool {
        try_or_def!(
            self.caps
                .workspace
                .as_ref()?
                .did_change_watched_files
                .as_ref()?
                .relative_pattern_support?
        )
    }

    pub fn prefill_caches(&self) -> bool {
        self.cachePriming_enable().to_owned()
    }

    pub fn location_link(&self) -> bool {
        try_or_def!(self.caps.text_document.as_ref()?.definition?.link_support?)
    }

    pub fn line_folding_only(&self) -> bool {
        try_or_def!(self.caps.text_document.as_ref()?.folding_range.as_ref()?.line_folding_only?)
    }

    pub fn hierarchical_symbols(&self) -> bool {
        try_or_def!(
            self.caps
                .text_document
                .as_ref()?
                .document_symbol
                .as_ref()?
                .hierarchical_document_symbol_support?
        )
    }

    pub fn code_action_literals(&self) -> bool {
        try_!(self
            .caps
            .text_document
            .as_ref()?
            .code_action
            .as_ref()?
            .code_action_literal_support
            .as_ref()?)
        .is_some()
    }

    pub fn work_done_progress(&self) -> bool {
        try_or_def!(self.caps.window.as_ref()?.work_done_progress?)
    }

    pub fn will_rename(&self) -> bool {
        try_or_def!(self.caps.workspace.as_ref()?.file_operations.as_ref()?.will_rename?)
    }

    pub fn change_annotation_support(&self) -> bool {
        try_!(self
            .caps
            .workspace
            .as_ref()?
            .workspace_edit
            .as_ref()?
            .change_annotation_support
            .as_ref()?)
        .is_some()
    }

    pub fn code_action_resolve(&self) -> bool {
        try_or_def!(self
            .caps
            .text_document
            .as_ref()?
            .code_action
            .as_ref()?
            .resolve_support
            .as_ref()?
            .properties
            .as_slice())
        .iter()
        .any(|it| it == "edit")
    }

    pub fn signature_help_label_offsets(&self) -> bool {
        try_or_def!(
            self.caps
                .text_document
                .as_ref()?
                .signature_help
                .as_ref()?
                .signature_information
                .as_ref()?
                .parameter_information
                .as_ref()?
                .label_offset_support?
        )
    }

    pub fn completion_label_details_support(&self) -> bool {
        try_!(self
            .caps
            .text_document
            .as_ref()?
            .completion
            .as_ref()?
            .completion_item
            .as_ref()?
            .label_details_support
            .as_ref()?)
        .is_some()
    }

    pub fn semantics_tokens_augments_syntax_tokens(&self) -> bool {
        try_!(self.caps.text_document.as_ref()?.semantic_tokens.as_ref()?.augments_syntax_tokens?)
            .unwrap_or(false)
    }

    pub fn position_encoding(&self) -> PositionEncoding {
        negotiated_encoding(&self.caps)
    }

    fn experimental(&self, index: &'static str) -> bool {
        try_or_def!(self.caps.experimental.as_ref()?.get(index)?.as_bool()?)
    }

    pub fn code_action_group(&self) -> bool {
        self.experimental("codeActionGroup")
    }

    pub fn local_docs(&self) -> bool {
        self.experimental("localDocs")
    }

    pub fn open_server_logs(&self) -> bool {
        self.experimental("openServerLogs")
    }

    pub fn server_status_notification(&self) -> bool {
        self.experimental("serverStatusNotification")
    }

    /// Whether the client supports colored output for full diagnostics from `checkOnSave`.
    pub fn color_diagnostic_output(&self) -> bool {
        self.experimental("colorDiagnosticOutput")
    }

    pub fn test_explorer(&self) -> bool {
        self.experimental("testExplorer")
    }

    pub fn publish_diagnostics(&self) -> bool {
        self.diagnostics_enable().to_owned()
    }

    pub fn diagnostics_map(&self) -> DiagnosticsMapConfig {
        DiagnosticsMapConfig {
            remap_prefix: self.diagnostics_remapPrefix().clone(),
            warnings_as_info: self.diagnostics_warningsAsInfo().clone(),
            warnings_as_hint: self.diagnostics_warningsAsHint().clone(),
            check_ignore: self.check_ignore().clone(),
        }
    }

    pub fn extra_args(&self) -> &Vec<String> {
        self.cargo_extraArgs()
    }

    pub fn extra_env(&self) -> &FxHashMap<String, String> {
        self.cargo_extraEnv()
    }

    pub fn check_extra_args(&self) -> Vec<String> {
        let mut extra_args = self.extra_args().clone();
        extra_args.extend_from_slice(self.check_extraArgs());
        extra_args
    }

    pub fn check_extra_env(&self) -> FxHashMap<String, String> {
        let mut extra_env = self.cargo_extraEnv().clone();
        extra_env.extend(self.check_extraEnv().clone());
        extra_env
    }

    pub fn lru_parse_query_capacity(&self) -> Option<usize> {
        self.lru_capacity().to_owned()
    }

    pub fn lru_query_capacities_config(&self) -> Option<&FxHashMap<Box<str>, usize>> {
        self.lru_query_capacities().is_empty().not().then(|| self.lru_query_capacities())
    }

    pub fn proc_macro_srv(&self) -> Option<AbsPathBuf> {
        let path = self.procMacro_server().clone()?;
        Some(AbsPathBuf::try_from(path).unwrap_or_else(|path| self.root_path.join(path)))
    }

    pub fn ignored_proc_macros(&self) -> &FxHashMap<Box<str>, Box<[Box<str>]>> {
        self.procMacro_ignored()
    }

    pub fn expand_proc_macros(&self) -> bool {
        self.procMacro_enable().to_owned()
    }

    pub fn files(&self) -> FilesConfig {
        FilesConfig {
            watcher: match self.files_watcher() {
                FilesWatcherDef::Client if self.did_change_watched_files_dynamic_registration() => {
                    FilesWatcher::Client
                }
                _ => FilesWatcher::Server,
            },
            exclude: self.files_excludeDirs().iter().map(|it| self.root_path.join(it)).collect(),
        }
    }

    pub fn notifications(&self) -> NotificationsConfig {
        NotificationsConfig {
            cargo_toml_not_found: self.notifications_cargoTomlNotFound().to_owned(),
            unindexed_project: self.notifications_unindexedProject().to_owned(),
        }
    }

    pub fn cargo_autoreload_config(&self) -> bool {
        self.cargo_autoreload().to_owned()
    }

    pub fn run_build_scripts(&self) -> bool {
        self.cargo_buildScripts_enable().to_owned() || self.procMacro_enable().to_owned()
    }

    pub fn cargo(&self) -> CargoConfig {
        let rustc_source = self.rustc_source().as_ref().map(|rustc_src| {
            if rustc_src == "discover" {
                RustLibSource::Discover
            } else {
                RustLibSource::Path(self.root_path.join(rustc_src))
            }
        });
        let sysroot = self.cargo_sysroot().as_ref().map(|sysroot| {
            if sysroot == "discover" {
                RustLibSource::Discover
            } else {
                RustLibSource::Path(self.root_path.join(sysroot))
            }
        });
        let sysroot_src =
            self.cargo_sysrootSrc().as_ref().map(|sysroot| self.root_path.join(sysroot));
        let sysroot_query_metadata = self.cargo_sysrootQueryMetadata();

        CargoConfig {
            all_targets: *self.cargo_allTargets(),
            features: match &self.cargo_features() {
                CargoFeaturesDef::All => CargoFeatures::All,
                CargoFeaturesDef::Selected(features) => CargoFeatures::Selected {
                    features: features.clone(),
                    no_default_features: self.cargo_noDefaultFeatures().to_owned(),
                },
            },
            target: self.cargo_target().clone(),
            sysroot,
            sysroot_query_metadata: *sysroot_query_metadata,
            sysroot_src,
            rustc_source,
            cfg_overrides: project_model::CfgOverrides {
                global: CfgDiff::new(
                    self.cargo_cfgs()
                        .iter()
                        .map(|(key, val)| match val {
                            Some(val) => CfgAtom::KeyValue { key: key.into(), value: val.into() },
                            None => CfgAtom::Flag(key.into()),
                        })
                        .collect(),
                    vec![],
                )
                .unwrap(),
                selective: Default::default(),
            },
            wrap_rustc_in_build_scripts: *self.cargo_buildScripts_useRustcWrapper(),
            invocation_strategy: match self.cargo_buildScripts_invocationStrategy() {
                InvocationStrategy::Once => project_model::InvocationStrategy::Once,
                InvocationStrategy::PerWorkspace => project_model::InvocationStrategy::PerWorkspace,
            },
            invocation_location: match self.cargo_buildScripts_invocationLocation() {
                InvocationLocation::Root => {
                    project_model::InvocationLocation::Root(self.root_path.clone())
                }
                InvocationLocation::Workspace => project_model::InvocationLocation::Workspace,
            },
            run_build_script_command: self.cargo_buildScripts_overrideCommand().clone(),
            extra_args: self.cargo_extraArgs().clone(),
            extra_env: self.cargo_extraEnv().clone(),
            target_dir: self.target_dir_from_config(),
        }
    }

    pub fn rustfmt(&self) -> RustfmtConfig {
        match &self.rustfmt_overrideCommand() {
            Some(args) if !args.is_empty() => {
                let mut args = args.clone();
                let command = args.remove(0);
                RustfmtConfig::CustomCommand { command, args }
            }
            Some(_) | None => RustfmtConfig::Rustfmt {
                extra_args: self.rustfmt_extraArgs().clone(),
                enable_range_formatting: *self.rustfmt_rangeFormatting_enable(),
            },
        }
    }

    pub fn flycheck_workspace(&self) -> bool {
        *self.check_workspace()
    }

    pub fn cargo_test_options(&self) -> CargoOptions {
        CargoOptions {
            target_triples: self.cargo_target().clone().into_iter().collect(),
            all_targets: false,
            no_default_features: *self.cargo_noDefaultFeatures(),
            all_features: matches!(self.cargo_features(), CargoFeaturesDef::All),
            features: match self.cargo_features().clone() {
                CargoFeaturesDef::All => vec![],
                CargoFeaturesDef::Selected(it) => it,
            },
            extra_args: self.extra_args().clone(),
            extra_env: self.extra_env().clone(),
            target_dir: self.target_dir_from_config(),
        }
    }

    pub fn flycheck(&self) -> FlycheckConfig {
        match &self.check_overrideCommand() {
            Some(args) if !args.is_empty() => {
                let mut args = args.clone();
                let command = args.remove(0);
                FlycheckConfig::CustomCommand {
                    command,
                    args,
                    extra_env: self.check_extra_env(),
                    invocation_strategy: match self.check_invocationStrategy() {
                        InvocationStrategy::Once => flycheck::InvocationStrategy::Once,
                        InvocationStrategy::PerWorkspace => {
                            flycheck::InvocationStrategy::PerWorkspace
                        }
                    },
                    invocation_location: match self.check_invocationLocation() {
                        InvocationLocation::Root => {
                            flycheck::InvocationLocation::Root(self.root_path.clone())
                        }
                        InvocationLocation::Workspace => flycheck::InvocationLocation::Workspace,
                    },
                }
            }
            Some(_) | None => FlycheckConfig::CargoCommand {
                command: self.check_command().clone(),
                options: CargoOptions {
                    target_triples: self
                        .check_targets()
                        .clone()
                        .and_then(|targets| match &targets.0[..] {
                            [] => None,
                            targets => Some(targets.into()),
                        })
                        .unwrap_or_else(|| self.cargo_target().clone().into_iter().collect()),
                    all_targets: self.check_allTargets().unwrap_or(*self.cargo_allTargets()),
                    no_default_features: self
                        .check_noDefaultFeatures()
                        .unwrap_or(*self.cargo_noDefaultFeatures()),
                    all_features: matches!(
                        self.check_features().as_ref().unwrap_or(self.cargo_features()),
                        CargoFeaturesDef::All
                    ),
                    features: match self
                        .check_features()
                        .clone()
                        .unwrap_or_else(|| self.cargo_features().clone())
                    {
                        CargoFeaturesDef::All => vec![],
                        CargoFeaturesDef::Selected(it) => it,
                    },
                    extra_args: self.check_extra_args(),
                    extra_env: self.check_extra_env(),
                    target_dir: self.target_dir_from_config(),
                },
                ansi_color_output: self.color_diagnostic_output(),
            },
        }
    }

    fn target_dir_from_config(&self) -> Option<Utf8PathBuf> {
        self.cargo_targetDir().as_ref().and_then(|target_dir| match target_dir {
            TargetDirectory::UseSubdirectory(true) => {
                Some(Utf8PathBuf::from("target/rust-analyzer"))
            }
            TargetDirectory::UseSubdirectory(false) => None,
            TargetDirectory::Directory(dir) if dir.is_relative() => Some(dir.clone()),
            TargetDirectory::Directory(_) => None,
        })
    }

    pub fn check_on_save(&self) -> bool {
        *self.checkOnSave()
    }

    pub fn script_rebuild_on_save(&self) -> bool {
        *self.cargo_buildScripts_rebuildOnSave()
    }

    pub fn runnables(&self) -> RunnablesConfig {
        RunnablesConfig {
            override_cargo: self.runnables_command().clone(),
            cargo_extra_args: self.runnables_extraArgs().clone(),
            extra_test_binary_args: self.runnables_extraTestBinaryArgs().clone(),
        }
    }

    pub fn find_all_refs_exclude_imports(&self) -> bool {
        *self.references_excludeImports()
    }

    pub fn find_all_refs_exclude_tests(&self) -> bool {
        *self.references_excludeTests()
    }

    pub fn snippet_cap(&self) -> bool {
        self.experimental("snippetTextEdit")
    }

    pub fn call_info(&self) -> CallInfoConfig {
        CallInfoConfig {
            params_only: matches!(self.signatureInfo_detail(), SignatureDetail::Parameters),
            docs: *self.signatureInfo_documentation_enable(),
        }
    }

    pub fn lens(&self) -> LensConfig {
        LensConfig {
            run: *self.lens_enable() && *self.lens_run_enable(),
            debug: *self.lens_enable() && *self.lens_debug_enable(),
            interpret: *self.lens_enable() && *self.lens_run_enable() && *self.interpret_tests(),
            implementations: *self.lens_enable() && *self.lens_implementations_enable(),
            method_refs: *self.lens_enable() && *self.lens_references_method_enable(),
            refs_adt: *self.lens_enable() && *self.lens_references_adt_enable(),
            refs_trait: *self.lens_enable() && *self.lens_references_trait_enable(),
            enum_variant_refs: *self.lens_enable() && *self.lens_references_enumVariant_enable(),
            location: *self.lens_location(),
        }
    }

    pub fn workspace_symbol(&self) -> WorkspaceSymbolConfig {
        WorkspaceSymbolConfig {
            search_scope: match self.workspace_symbol_search_scope() {
                WorkspaceSymbolSearchScopeDef::Workspace => WorkspaceSymbolSearchScope::Workspace,
                WorkspaceSymbolSearchScopeDef::WorkspaceAndDependencies => {
                    WorkspaceSymbolSearchScope::WorkspaceAndDependencies
                }
            },
            search_kind: match self.workspace_symbol_search_kind() {
                WorkspaceSymbolSearchKindDef::OnlyTypes => WorkspaceSymbolSearchKind::OnlyTypes,
                WorkspaceSymbolSearchKindDef::AllSymbols => WorkspaceSymbolSearchKind::AllSymbols,
            },
            search_limit: *self.workspace_symbol_search_limit(),
        }
    }

    pub fn semantic_tokens_refresh(&self) -> bool {
        try_or_def!(self.caps.workspace.as_ref()?.semantic_tokens.as_ref()?.refresh_support?)
    }

    pub fn code_lens_refresh(&self) -> bool {
        try_or_def!(self.caps.workspace.as_ref()?.code_lens.as_ref()?.refresh_support?)
    }

    pub fn inlay_hints_refresh(&self) -> bool {
        try_or_def!(self.caps.workspace.as_ref()?.inlay_hint.as_ref()?.refresh_support?)
    }

    pub fn insert_replace_support(&self) -> bool {
        try_or_def!(
            self.caps
                .text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .insert_replace_support?
        )
    }

    pub fn client_commands(&self) -> ClientCommandsConfig {
        let commands =
            try_or!(self.caps.experimental.as_ref()?.get("commands")?, &serde_json::Value::Null);
        let commands: Option<lsp_ext::ClientCommandOptions> =
            serde_json::from_value(commands.clone()).ok();
        let force = commands.is_none() && *self.lens_forceCustomCommands();
        let commands = commands.map(|it| it.commands).unwrap_or_default();

        let get = |name: &str| commands.iter().any(|it| it == name) || force;

        ClientCommandsConfig {
            run_single: get("rust-analyzer.runSingle"),
            debug_single: get("rust-analyzer.debugSingle"),
            show_reference: get("rust-analyzer.showReferences"),
            goto_location: get("rust-analyzer.gotoLocation"),
            trigger_parameter_hints: get("editor.action.triggerParameterHints"),
        }
    }

    pub fn prime_caches_num_threads(&self) -> u8 {
        match *self.cachePriming_numThreads() {
            0 => num_cpus::get_physical().try_into().unwrap_or(u8::MAX),
            n => n,
        }
    }

    pub fn main_loop_num_threads(&self) -> usize {
        self.numThreads().unwrap_or(num_cpus::get_physical())
    }

    pub fn typing_autoclose_angle(&self) -> bool {
        *self.typing_autoClosingAngleBrackets_enable()
    }

    // VSCode is our reference implementation, so we allow ourselves to work around issues by
    // special casing certain versions
    pub fn visual_studio_code_version(&self) -> Option<&Version> {
        self.visual_studio_code_version.as_ref()
    }
}
// Deserialization definitions

macro_rules! create_bool_or_string_serde {
    ($ident:ident<$bool:literal, $string:literal>) => {
        mod $ident {
            pub(super) fn deserialize<'de, D>(d: D) -> Result<(), D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V;
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = ();

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str(concat!(
                            stringify!($bool),
                            " or \"",
                            stringify!($string),
                            "\""
                        ))
                    }

                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            $bool => Ok(()),
                            _ => Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Bool(v),
                                &self,
                            )),
                        }
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            $string => Ok(()),
                            _ => Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(v),
                                &self,
                            )),
                        }
                    }

                    fn visit_enum<A>(self, a: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::EnumAccess<'de>,
                    {
                        use serde::de::VariantAccess;
                        let (variant, va) = a.variant::<&'de str>()?;
                        va.unit_variant()?;
                        match variant {
                            $string => Ok(()),
                            _ => Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(variant),
                                &self,
                            )),
                        }
                    }
                }
                d.deserialize_any(V)
            }

            pub(super) fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str($string)
            }
        }
    };
}
create_bool_or_string_serde!(true_or_always<true, "always">);
create_bool_or_string_serde!(false_or_never<false, "never">);

macro_rules! named_unit_variant {
    ($variant:ident) => {
        pub(super) mod $variant {
            pub(in super::super) fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V;
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = ();
                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(concat!("\"", stringify!($variant), "\""))
                    }
                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if value == stringify!($variant) {
                            Ok(())
                        } else {
                            Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
                        }
                    }
                }
                deserializer.deserialize_str(V)
            }
            pub(in super::super) fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(stringify!($variant))
            }
        }
    };
}

mod unit_v {
    named_unit_variant!(all);
    named_unit_variant!(skip_trivial);
    named_unit_variant!(mutable);
    named_unit_variant!(reborrow);
    named_unit_variant!(fieldless);
    named_unit_variant!(with_block);
    named_unit_variant!(decimal);
    named_unit_variant!(hexadecimal);
    named_unit_variant!(both);
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
enum SnippetScopeDef {
    #[default]
    Expr,
    Item,
    Type,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
struct SnippetDef {
    #[serde(with = "single_or_array")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    prefix: Vec<String>,

    #[serde(with = "single_or_array")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    postfix: Vec<String>,

    #[serde(with = "single_or_array")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    body: Vec<String>,

    #[serde(with = "single_or_array")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    requires: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    scope: SnippetScopeDef,
}

mod single_or_array {
    use serde::{Deserialize, Serialize};

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SingleOrVec;

        impl<'de> serde::de::Visitor<'de> for SingleOrVec {
            type Value = Vec<String>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("string or array of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(vec![value.to_owned()])
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
            }
        }

        deserializer.deserialize_any(SingleOrVec)
    }

    pub(super) fn serialize<S>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match vec {
            // []  case is handled by skip_serializing_if
            [single] => serializer.serialize_str(single),
            slice => slice.serialize(serializer),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum ManifestOrProjectJson {
    Manifest(Utf8PathBuf),
    ProjectJson(ProjectJsonData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ExprFillDefaultDef {
    Todo,
    Default,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ImportGranularityDef {
    Preserve,
    Item,
    Crate,
    Module,
    One,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
enum CallableCompletionDef {
    FillArguments,
    AddParentheses,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum CargoFeaturesDef {
    #[serde(with = "unit_v::all")]
    All,
    Selected(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) enum InvocationStrategy {
    Once,
    PerWorkspace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CheckOnSaveTargets(#[serde(with = "single_or_array")] Vec<String>);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum InvocationLocation {
    Root,
    Workspace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum LifetimeElisionDef {
    #[serde(with = "true_or_always")]
    Always,
    #[serde(with = "false_or_never")]
    Never,
    #[serde(with = "unit_v::skip_trivial")]
    SkipTrivial,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum ClosureReturnTypeHintsDef {
    #[serde(with = "true_or_always")]
    Always,
    #[serde(with = "false_or_never")]
    Never,
    #[serde(with = "unit_v::with_block")]
    WithBlock,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ClosureStyle {
    ImplFn,
    RustAnalyzer,
    WithId,
    Hide,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum ReborrowHintsDef {
    #[serde(with = "true_or_always")]
    Always,
    #[serde(with = "false_or_never")]
    Never,
    #[serde(with = "unit_v::mutable")]
    Mutable,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum AdjustmentHintsDef {
    #[serde(with = "true_or_always")]
    Always,
    #[serde(with = "false_or_never")]
    Never,
    #[serde(with = "unit_v::reborrow")]
    Reborrow,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum DiscriminantHintsDef {
    #[serde(with = "true_or_always")]
    Always,
    #[serde(with = "false_or_never")]
    Never,
    #[serde(with = "unit_v::fieldless")]
    Fieldless,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum AdjustmentHintsModeDef {
    Prefix,
    Postfix,
    PreferPrefix,
    PreferPostfix,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum FilesWatcherDef {
    Client,
    Notify,
    Server,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ImportPrefixDef {
    Plain,
    #[serde(alias = "self")]
    BySelf,
    #[serde(alias = "crate")]
    ByCrate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum WorkspaceSymbolSearchScopeDef {
    Workspace,
    WorkspaceAndDependencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum SignatureDetail {
    Full,
    Parameters,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum WorkspaceSymbolSearchKindDef {
    OnlyTypes,
    AllSymbols,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
enum MemoryLayoutHoverRenderKindDef {
    #[serde(with = "unit_v::decimal")]
    Decimal,
    #[serde(with = "unit_v::hexadecimal")]
    Hexadecimal,
    #[serde(with = "unit_v::both")]
    Both,
}

#[test]
fn untagged_option_hover_render_kind() {
    let hex = MemoryLayoutHoverRenderKindDef::Hexadecimal;

    let ser = serde_json::to_string(&Some(hex)).unwrap();
    assert_eq!(&ser, "\"hexadecimal\"");

    let opt: Option<_> = serde_json::from_str("\"hexadecimal\"").unwrap();
    assert_eq!(opt, Some(hex));
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum TargetDirectory {
    UseSubdirectory(bool),
    Directory(Utf8PathBuf),
}

macro_rules! _default_val {
    (@verbatim: $s:literal, $ty:ty) => {{
        let default_: $ty = serde_json::from_str(&$s).unwrap();
        default_
    }};
    ($default:expr, $ty:ty) => {{
        let default_: $ty = $default;
        default_
    }};
}
use _default_val as default_val;

macro_rules! _default_str {
    (@verbatim: $s:literal, $_ty:ty) => {
        $s.to_owned()
    };
    ($default:expr, $ty:ty) => {{
        let val = default_val!($default, $ty);
        serde_json::to_string_pretty(&val).unwrap()
    }};
}
use _default_str as default_str;

macro_rules! _impl_for_config_data {
    (local, $(
            $(#[doc=$doc:literal])*
            $vis:vis $field:ident : $ty:ty = $default:expr,
        )*
    ) => {
        impl Config {
            $(
                $($doc)*
                #[allow(non_snake_case)]
                $vis fn $field(&self, _source_root: Option<SourceRootId>) -> &$ty {
                    if let Some(v) = self.client_config.local.$field.as_ref() {
                        return &v;
                    }

                    if let Some(v) = self.user_config.local.$field.as_ref() {
                        return &v;
                    }

                    &self.default_config.local.$field
                }
            )*
        }
    };
    (global, $(
                $(#[doc=$doc:literal])*
                $vis:vis $field:ident : $ty:ty = $default:expr,
            )*
        ) => {
        impl Config {
            $(
                $($doc)*
                #[allow(non_snake_case)]
                $vis fn $field(&self) -> &$ty {
                    if let Some(v) = self.client_config.global.$field.as_ref() {
                        return &v;
                    }

                    if let Some(v) = self.user_config.global.$field.as_ref() {
                        return &v;
                    }

                    &self.default_config.global.$field
                }
            )*
        }
    };
    (client, $(
        $(#[doc=$doc:literal])*
        $vis:vis $field:ident : $ty:ty = $default:expr,
    )*
    ) => {
        impl Config {
            $(
                $($doc)*
                #[allow(non_snake_case)]
                $vis fn $field(&self) -> &$ty {
                    if let Some(v) = self.client_config.global.$field.as_ref() {
                        return &v;
                    }

                    &self.default_config.client.$field
                }
            )*
        }
    };
}
use _impl_for_config_data as impl_for_config_data;

macro_rules! _config_data {
    // modname is for the tests
    ($(#[doc=$dox:literal])* $modname:ident: struct $name:ident <- $input:ident -> {
        $(
            $(#[doc=$doc:literal])*
            $vis:vis $field:ident $(| $alias:ident)*: $ty:ty = $(@$marker:ident: )? $default:expr,
        )*
    }) => {
        /// Default config values for this grouping.
        #[allow(non_snake_case)]
        #[derive(Debug, Clone, Serialize)]
        struct $name { $($field: $ty,)* }

        impl_for_config_data!{
            $modname,
            $(
                $vis $field : $ty = $default,
            )*
        }

        /// All fields `Option<T>`, `None` representing fields not set in a particular JSON/TOML blob.
        #[allow(non_snake_case)]
        #[derive(Clone, Serialize, Default)]
        struct $input { $(
            #[serde(skip_serializing_if = "Option::is_none")]
            $field: Option<$ty>,
        )* }

        impl std::fmt::Debug for $input {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut s = f.debug_struct(stringify!($input));
                $(
                    if let Some(val) = self.$field.as_ref() {
                        s.field(stringify!($field), val);
                    }
                )*
                s.finish()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name {$(
                    $field: default_val!($(@$marker:)? $default, $ty),
                )*}
            }
        }

        #[allow(unused)]
        impl $name {
            /// Applies overrides from some more local config blob, to self.
            fn apply_input(&mut self, input: $input) {
                $(
                    if let Some(value) = input.$field {
                        self.$field = value;
                    }
                )*
            }

            fn clone_with_overrides(&self, input: $input) -> Self {
                Self {$(
                    $field: input.$field.unwrap_or_else(|| self.$field.clone()),
                )*}
            }
        }

        #[allow(unused, clippy::ptr_arg)]
        impl $input {
            fn from_json(json: &mut serde_json::Value, error_sink: &mut Vec<(String, serde_json::Error)>) -> Self {
                Self {$(
                    $field: get_field(
                        json,
                        error_sink,
                        stringify!($field),
                        None$(.or(Some(stringify!($alias))))*,
                    ),
                )*}
            }

            fn from_toml(toml: &mut toml::Table, error_sink: &mut Vec<(String, toml::de::Error)>) -> Self {
                Self {$(
                    $field: get_field_toml::<$ty>(
                        toml,
                        error_sink,
                        stringify!($field),
                        None$(.or(Some(stringify!($alias))))*,
                    ),
                )*}
            }

            fn schema_fields(sink: &mut Vec<SchemaField>) {
                sink.extend_from_slice(&[
                    $({
                        let field = stringify!($field);
                        let ty = stringify!($ty);
                        let default = default_str!($(@$marker:)? $default, $ty);

                        (field, ty, &[$($doc),*], default)
                    },)*
                ])
            }
        }

        mod $modname {
            #[test]
            fn fields_are_sorted() {
                let field_names: &'static [&'static str] = &[$(stringify!($field)),*];
                field_names.windows(2).for_each(|w| assert!(w[0] <= w[1], "{} <= {} does not hold", w[0], w[1]));
            }
        }
    };
}
use _config_data as config_data;

#[derive(Default, Debug, Clone)]
struct DefaultConfigData {
    global: GlobalDefaultConfigData,
    local: LocalDefaultConfigData,
    #[allow(dead_code)]
    client: ClientDefaultConfigData,
}

/// All of the config levels, all fields `Option<T>`, to describe fields that are actually set by
/// some rust-analyzer.toml file or JSON blob. An empty rust-analyzer.toml corresponds to
/// all fields being None.
#[derive(Debug, Clone, Default)]
struct FullConfigInput {
    global: GlobalConfigInput,
    local: LocalConfigInput,
    #[allow(dead_code)]
    client: ClientConfigInput,
}

impl FullConfigInput {
    fn from_json(
        mut json: serde_json::Value,
        error_sink: &mut Vec<(String, serde_json::Error)>,
    ) -> FullConfigInput {
        FullConfigInput {
            global: GlobalConfigInput::from_json(&mut json, error_sink),
            local: LocalConfigInput::from_json(&mut json, error_sink),
            client: ClientConfigInput::from_json(&mut json, error_sink),
        }
    }

    fn schema_fields() -> Vec<SchemaField> {
        let mut fields = Vec::new();
        GlobalConfigInput::schema_fields(&mut fields);
        LocalConfigInput::schema_fields(&mut fields);
        ClientConfigInput::schema_fields(&mut fields);
        // HACK: sort the fields, so the diffs on the generated docs/schema are smaller
        fields.sort_by_key(|&(x, ..)| x);
        fields
    }

    fn json_schema() -> serde_json::Value {
        schema(&Self::schema_fields())
    }

    #[cfg(test)]
    fn manual() -> String {
        manual(&Self::schema_fields())
    }
}

/// All of the config levels, all fields `Option<T>`, to describe fields that are actually set by
/// some rust-analyzer.toml file or JSON blob. An empty rust-analyzer.toml corresponds to
/// all fields being None.
#[derive(Debug, Clone, Default)]
struct GlobalLocalConfigInput {
    global: GlobalConfigInput,
    local: LocalConfigInput,
}

impl GlobalLocalConfigInput {
    #[allow(dead_code)]
    fn from_toml(
        mut toml: toml::Table,
        error_sink: &mut Vec<(String, toml::de::Error)>,
    ) -> GlobalLocalConfigInput {
        GlobalLocalConfigInput {
            global: GlobalConfigInput::from_toml(&mut toml, error_sink),
            local: LocalConfigInput::from_toml(&mut toml, error_sink),
        }
    }
}

fn get_field_toml<T: DeserializeOwned>(
    val: &toml::Table,
    error_sink: &mut Vec<(String, toml::de::Error)>,
    field: &'static str,
    alias: Option<&'static str>,
) -> Option<T> {
    alias
        .into_iter()
        .chain(iter::once(field))
        .filter_map(move |field| {
            let subkeys = field.split('_');
            let mut v = val;
            for subkey in subkeys {
                if let Some(val) = v.get(subkey) {
                    if let Some(map) = val.as_table() {
                        v = map;
                    } else {
                        return Some(toml::Value::try_into(val.clone()).map_err(|e| (e, v)));
                    }
                } else {
                    return None;
                }
            }
            None
        })
        .find(Result::is_ok)
        .and_then(|res| match res {
            Ok(it) => Some(it),
            Err((e, pointer)) => {
                error_sink.push((pointer.to_string(), e));
                None
            }
        })
}

fn get_field<T: DeserializeOwned>(
    json: &mut serde_json::Value,
    error_sink: &mut Vec<(String, serde_json::Error)>,
    field: &'static str,
    alias: Option<&'static str>,
) -> Option<T> {
    // XXX: check alias first, to work around the VS Code where it pre-fills the
    // defaults instead of sending an empty object.
    alias
        .into_iter()
        .chain(iter::once(field))
        .filter_map(move |field| {
            let mut pointer = field.replace('_', "/");
            pointer.insert(0, '/');
            json.pointer_mut(&pointer)
                .map(|it| serde_json::from_value(it.take()).map_err(|e| (e, pointer)))
        })
        .find(Result::is_ok)
        .and_then(|res| match res {
            Ok(it) => Some(it),
            Err((e, pointer)) => {
                tracing::warn!("Failed to deserialize config field at {}: {:?}", pointer, e);
                error_sink.push((pointer, e));
                None
            }
        })
}

type SchemaField = (&'static str, &'static str, &'static [&'static str], String);

fn schema(fields: &[SchemaField]) -> serde_json::Value {
    let map = fields
        .iter()
        .map(|(field, ty, doc, default)| {
            let name = field.replace('_', ".");
            let name = format!("rust-analyzer.{name}");
            let props = field_props(field, ty, doc, default);
            (name, props)
        })
        .collect::<serde_json::Map<_, _>>();
    map.into()
}

fn field_props(field: &str, ty: &str, doc: &[&str], default: &str) -> serde_json::Value {
    let doc = doc_comment_to_string(doc);
    let doc = doc.trim_end_matches('\n');
    assert!(
        doc.ends_with('.') && doc.starts_with(char::is_uppercase),
        "bad docs for {field}: {doc:?}"
    );
    let default = default.parse::<serde_json::Value>().unwrap();

    let mut map = serde_json::Map::default();
    macro_rules! set {
        ($($key:literal: $value:tt),*$(,)?) => {{$(
            map.insert($key.into(), serde_json::json!($value));
        )*}};
    }
    set!("markdownDescription": doc);
    set!("default": default);

    match ty {
        "bool" => set!("type": "boolean"),
        "usize" => set!("type": "integer", "minimum": 0),
        "String" => set!("type": "string"),
        "Vec<String>" => set! {
            "type": "array",
            "items": { "type": "string" },
        },
        "Vec<Utf8PathBuf>" => set! {
            "type": "array",
            "items": { "type": "string" },
        },
        "FxHashSet<String>" => set! {
            "type": "array",
            "items": { "type": "string" },
            "uniqueItems": true,
        },
        "FxHashMap<Box<str>, Box<[Box<str>]>>" => set! {
            "type": "object",
        },
        "IndexMap<String, SnippetDef>" => set! {
            "type": "object",
        },
        "FxHashMap<String, String>" => set! {
            "type": "object",
        },
        "FxHashMap<Box<str>, usize>" => set! {
            "type": "object",
        },
        "FxHashMap<String, Option<String>>" => set! {
            "type": "object",
        },
        "Option<usize>" => set! {
            "type": ["null", "integer"],
            "minimum": 0,
        },
        "Option<String>" => set! {
            "type": ["null", "string"],
        },
        "Option<Utf8PathBuf>" => set! {
            "type": ["null", "string"],
        },
        "Option<bool>" => set! {
            "type": ["null", "boolean"],
        },
        "Option<Vec<String>>" => set! {
            "type": ["null", "array"],
            "items": { "type": "string" },
        },
        "ExprFillDefaultDef" => set! {
            "type": "string",
            "enum": ["todo", "default"],
            "enumDescriptions": [
                "Fill missing expressions with the `todo` macro",
                "Fill missing expressions with reasonable defaults, `new` or `default` constructors."
            ],
        },
        "ImportGranularityDef" => set! {
            "type": "string",
            "enum": ["preserve", "crate", "module", "item", "one"],
            "enumDescriptions": [
                "Do not change the granularity of any imports and preserve the original structure written by the developer.",
                "Merge imports from the same crate into a single use statement. Conversely, imports from different crates are split into separate statements.",
                "Merge imports from the same module into a single use statement. Conversely, imports from different modules are split into separate statements.",
                "Flatten imports so that each has its own use statement.",
                "Merge all imports into a single use statement as long as they have the same visibility and attributes."
            ],
        },
        "ImportPrefixDef" => set! {
            "type": "string",
            "enum": [
                "plain",
                "self",
                "crate"
            ],
            "enumDescriptions": [
                "Insert import paths relative to the current module, using up to one `super` prefix if the parent module contains the requested item.",
                "Insert import paths relative to the current module, using up to one `super` prefix if the parent module contains the requested item. Prefixes `self` in front of the path if it starts with a module.",
                "Force import paths to be absolute by always starting them with `crate` or the extern crate name they come from."
            ],
        },
        "Vec<ManifestOrProjectJson>" => set! {
            "type": "array",
            "items": { "type": ["string", "object"] },
        },
        "WorkspaceSymbolSearchScopeDef" => set! {
            "type": "string",
            "enum": ["workspace", "workspace_and_dependencies"],
            "enumDescriptions": [
                "Search in current workspace only.",
                "Search in current workspace and dependencies."
            ],
        },
        "WorkspaceSymbolSearchKindDef" => set! {
            "type": "string",
            "enum": ["only_types", "all_symbols"],
            "enumDescriptions": [
                "Search for types only.",
                "Search for all symbols kinds."
            ],
        },
        "ParallelCachePrimingNumThreads" => set! {
            "type": "number",
            "minimum": 0,
            "maximum": 255
        },
        "LifetimeElisionDef" => set! {
            "type": "string",
            "enum": [
                "always",
                "never",
                "skip_trivial"
            ],
            "enumDescriptions": [
                "Always show lifetime elision hints.",
                "Never show lifetime elision hints.",
                "Only show lifetime elision hints if a return type is involved."
            ]
        },
        "ClosureReturnTypeHintsDef" => set! {
            "type": "string",
            "enum": [
                "always",
                "never",
                "with_block"
            ],
            "enumDescriptions": [
                "Always show type hints for return types of closures.",
                "Never show type hints for return types of closures.",
                "Only show type hints for return types of closures with blocks."
            ]
        },
        "ReborrowHintsDef" => set! {
            "type": "string",
            "enum": [
                "always",
                "never",
                "mutable"
            ],
            "enumDescriptions": [
                "Always show reborrow hints.",
                "Never show reborrow hints.",
                "Only show mutable reborrow hints."
            ]
        },
        "AdjustmentHintsDef" => set! {
            "type": "string",
            "enum": [
                "always",
                "never",
                "reborrow"
            ],
            "enumDescriptions": [
                "Always show all adjustment hints.",
                "Never show adjustment hints.",
                "Only show auto borrow and dereference adjustment hints."
            ]
        },
        "DiscriminantHintsDef" => set! {
            "type": "string",
            "enum": [
                "always",
                "never",
                "fieldless"
            ],
            "enumDescriptions": [
                "Always show all discriminant hints.",
                "Never show discriminant hints.",
                "Only show discriminant hints on fieldless enum variants."
            ]
        },
        "AdjustmentHintsModeDef" => set! {
            "type": "string",
            "enum": [
                "prefix",
                "postfix",
                "prefer_prefix",
                "prefer_postfix",
            ],
            "enumDescriptions": [
                "Always show adjustment hints as prefix (`*expr`).",
                "Always show adjustment hints as postfix (`expr.*`).",
                "Show prefix or postfix depending on which uses less parenthesis, preferring prefix.",
                "Show prefix or postfix depending on which uses less parenthesis, preferring postfix.",
            ]
        },
        "CargoFeaturesDef" => set! {
            "anyOf": [
                {
                    "type": "string",
                    "enum": [
                        "all"
                    ],
                    "enumDescriptions": [
                        "Pass `--all-features` to cargo",
                    ]
                },
                {
                    "type": "array",
                    "items": { "type": "string" }
                }
            ],
        },
        "Option<CargoFeaturesDef>" => set! {
            "anyOf": [
                {
                    "type": "string",
                    "enum": [
                        "all"
                    ],
                    "enumDescriptions": [
                        "Pass `--all-features` to cargo",
                    ]
                },
                {
                    "type": "array",
                    "items": { "type": "string" }
                },
                { "type": "null" }
            ],
        },
        "CallableCompletionDef" => set! {
            "type": "string",
            "enum": [
                "fill_arguments",
                "add_parentheses",
                "none",
            ],
            "enumDescriptions": [
                "Add call parentheses and pre-fill arguments.",
                "Add call parentheses.",
                "Do no snippet completions for callables."
            ]
        },
        "SignatureDetail" => set! {
            "type": "string",
            "enum": ["full", "parameters"],
            "enumDescriptions": [
                "Show the entire signature.",
                "Show only the parameters."
            ],
        },
        "FilesWatcherDef" => set! {
            "type": "string",
            "enum": ["client", "server"],
            "enumDescriptions": [
                "Use the client (editor) to watch files for changes",
                "Use server-side file watching",
            ],
        },
        "AnnotationLocation" => set! {
            "type": "string",
            "enum": ["above_name", "above_whole_item"],
            "enumDescriptions": [
                "Render annotations above the name of the item.",
                "Render annotations above the whole item, including documentation comments and attributes."
            ],
        },
        "InvocationStrategy" => set! {
            "type": "string",
            "enum": ["per_workspace", "once"],
            "enumDescriptions": [
                "The command will be executed for each workspace.",
                "The command will be executed once."
            ],
        },
        "InvocationLocation" => set! {
            "type": "string",
            "enum": ["workspace", "root"],
            "enumDescriptions": [
                "The command will be executed in the corresponding workspace root.",
                "The command will be executed in the project root."
            ],
        },
        "Option<CheckOnSaveTargets>" => set! {
            "anyOf": [
                {
                    "type": "null"
                },
                {
                    "type": "string",
                },
                {
                    "type": "array",
                    "items": { "type": "string" }
                },
            ],
        },
        "ClosureStyle" => set! {
            "type": "string",
            "enum": ["impl_fn", "rust_analyzer", "with_id", "hide"],
            "enumDescriptions": [
                "`impl_fn`: `impl FnMut(i32, u64) -> i8`",
                "`rust_analyzer`: `|i32, u64| -> i8`",
                "`with_id`: `{closure#14352}`, where that id is the unique number of the closure in r-a internals",
                "`hide`: Shows `...` for every closure type",
            ],
        },
        "Option<MemoryLayoutHoverRenderKindDef>" => set! {
            "anyOf": [
                {
                    "type": "null"
                },
                {
                    "type": "string",
                    "enum": ["both", "decimal", "hexadecimal", ],
                    "enumDescriptions": [
                        "Render as 12 (0xC)",
                        "Render as 12",
                        "Render as 0xC"
                    ],
                },
            ],
        },
        "Option<TargetDirectory>" => set! {
            "anyOf": [
                {
                    "type": "null"
                },
                {
                    "type": "boolean"
                },
                {
                    "type": "string"
                },
            ],
        },
        _ => panic!("missing entry for {ty}: {default}"),
    }

    map.into()
}

#[cfg(test)]
fn manual(fields: &[SchemaField]) -> String {
    fields.iter().fold(String::new(), |mut acc, (field, _ty, doc, default)| {
        let name = format!("rust-analyzer.{}", field.replace('_', "."));
        let doc = doc_comment_to_string(doc);
        if default.contains('\n') {
            format_to_acc!(
                acc,
                r#"[[{name}]]{name}::
+
--
Default:
----
{default}
----
{doc}
--
"#
            )
        } else {
            format_to_acc!(acc, "[[{name}]]{name} (default: `{default}`)::\n+\n--\n{doc}--\n")
        }
    })
}

fn doc_comment_to_string(doc: &[&str]) -> String {
    doc.iter()
        .map(|it| it.strip_prefix(' ').unwrap_or(it))
        .fold(String::new(), |mut acc, it| format_to_acc!(acc, "{it}\n"))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use test_utils::{ensure_file_contents, project_root};

    use super::*;

    #[test]
    fn generate_package_json_config() {
        let s = Config::json_schema();
        let schema = format!("{s:#}");
        let mut schema = schema
            .trim_start_matches('{')
            .trim_end_matches('}')
            .replace("  ", "    ")
            .replace('\n', "\n            ")
            .trim_start_matches('\n')
            .trim_end()
            .to_owned();
        schema.push_str(",\n");

        // Transform the asciidoc form link to markdown style.
        //
        // https://link[text] => [text](https://link)
        let url_matches = schema.match_indices("https://");
        let mut url_offsets = url_matches.map(|(idx, _)| idx).collect::<Vec<usize>>();
        url_offsets.reverse();
        for idx in url_offsets {
            let link = &schema[idx..];
            // matching on whitespace to ignore normal links
            if let Some(link_end) = link.find(|c| c == ' ' || c == '[') {
                if link.chars().nth(link_end) == Some('[') {
                    if let Some(link_text_end) = link.find(']') {
                        let link_text = link[link_end..(link_text_end + 1)].to_string();

                        schema.replace_range((idx + link_end)..(idx + link_text_end + 1), "");
                        schema.insert(idx, '(');
                        schema.insert(idx + link_end + 1, ')');
                        schema.insert_str(idx, &link_text);
                    }
                }
            }
        }

        let package_json_path = project_root().join("editors/code/package.json");
        let mut package_json = fs::read_to_string(&package_json_path).unwrap();

        let start_marker = "                \"$generated-start\": {},\n";
        let end_marker = "                \"$generated-end\": {}\n";

        let start = package_json.find(start_marker).unwrap() + start_marker.len();
        let end = package_json.find(end_marker).unwrap();

        let p = remove_ws(&package_json[start..end]);
        let s = remove_ws(&schema);
        if !p.contains(&s) {
            package_json.replace_range(start..end, &schema);
            ensure_file_contents(&package_json_path, &package_json)
        }
    }

    #[test]
    fn generate_config_documentation() {
        let docs_path = project_root().join("docs/user/generated_config.adoc");
        let expected = FullConfigInput::manual();
        ensure_file_contents(&docs_path, &expected);
    }

    fn remove_ws(text: &str) -> String {
        text.replace(char::is_whitespace, "")
    }

    #[test]
    fn proc_macro_srv_null() {
        let mut config = Config::new(
            AbsPathBuf::try_from(project_root()).unwrap(),
            Default::default(),
            vec![],
            None,
        );
        config
            .update(serde_json::json!({
                "procMacro_server": null,
            }))
            .unwrap();
        assert_eq!(config.proc_macro_srv(), None);
    }

    #[test]
    fn proc_macro_srv_abs() {
        let mut config = Config::new(
            AbsPathBuf::try_from(project_root()).unwrap(),
            Default::default(),
            vec![],
            None,
        );
        config
            .update(serde_json::json!({
                "procMacro": {"server": project_root().display().to_string()}
            }))
            .unwrap();
        assert_eq!(config.proc_macro_srv(), Some(AbsPathBuf::try_from(project_root()).unwrap()));
    }

    #[test]
    fn proc_macro_srv_rel() {
        let mut config = Config::new(
            AbsPathBuf::try_from(project_root()).unwrap(),
            Default::default(),
            vec![],
            None,
        );
        config
            .update(serde_json::json!({
                "procMacro": {"server": "./server"}
            }))
            .unwrap();
        assert_eq!(
            config.proc_macro_srv(),
            Some(AbsPathBuf::try_from(project_root().join("./server")).unwrap())
        );
    }

    #[test]
    fn cargo_target_dir_unset() {
        let mut config = Config::new(
            AbsPathBuf::try_from(project_root()).unwrap(),
            Default::default(),
            vec![],
            None,
        );
        config
            .update(serde_json::json!({
                "rust": { "analyzerTargetDir": null }
            }))
            .unwrap();
        assert_eq!(config.cargo_targetDir(), &None);
        assert!(
            matches!(config.flycheck(), FlycheckConfig::CargoCommand { options, .. } if options.target_dir.is_none())
        );
    }

    #[test]
    fn cargo_target_dir_subdir() {
        let mut config = Config::new(
            AbsPathBuf::try_from(project_root()).unwrap(),
            Default::default(),
            vec![],
            None,
        );
        config
            .update(serde_json::json!({
                "rust": { "analyzerTargetDir": true }
            }))
            .unwrap();
        assert_eq!(config.cargo_targetDir(), &Some(TargetDirectory::UseSubdirectory(true)));
        assert!(
            matches!(config.flycheck(), FlycheckConfig::CargoCommand { options, .. } if options.target_dir == Some(Utf8PathBuf::from("target/rust-analyzer")))
        );
    }

    #[test]
    fn cargo_target_dir_relative_dir() {
        let mut config = Config::new(
            AbsPathBuf::try_from(project_root()).unwrap(),
            Default::default(),
            vec![],
            None,
        );
        config
            .update(serde_json::json!({
                "rust": { "analyzerTargetDir": "other_folder" }
            }))
            .unwrap();
        assert_eq!(
            config.cargo_targetDir(),
            &Some(TargetDirectory::Directory(Utf8PathBuf::from("other_folder")))
        );
        assert!(
            matches!(config.flycheck(), FlycheckConfig::CargoCommand { options, .. } if options.target_dir == Some(Utf8PathBuf::from("other_folder")))
        );
    }
}
