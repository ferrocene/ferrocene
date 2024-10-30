// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::HashSet;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use crate::env;
use crate::error::Error;
use crate::report::Reporter;
use crate::targets::Target;
use crate::utils::run_command;

static SAMPLE_PROGRAMS: &[SampleProgram] = &[
    SampleProgram {
        name: "addition.rs",
        contents: include_bytes!("../sample-programs/addition.rs"),
        rustflags: &["--crate-type", "lib", "--edition", "2021"],
        expected_executables: &[],
        expected_libraries: &[],
        expected_rlibs: &["addition"],
        executable_output: None,
    },
    SampleProgram {
        name: "subtraction.rs",
        contents: include_bytes!("../sample-programs/subtraction.rs"),
        rustflags: &["--crate-type", "staticlib", "--edition", "2021"],
        expected_executables: &[],
        expected_libraries: &["subtraction"],
        expected_rlibs: &[],
        executable_output: None,
    },
    SampleProgram {
        name: "subtraction-sys.rs",
        contents: include_bytes!("../sample-programs/subtraction-sys.rs"),
        rustflags: &["--crate-type", "lib", "--edition", "2021", "-l", "subtraction"],
        expected_executables: &[],
        expected_libraries: &[],
        expected_rlibs: &["subtraction_sys"],
        executable_output: None,
    },
    SampleProgram {
        name: "assertion.rs",
        contents: include_bytes!("../sample-programs/assertion.rs"),
        rustflags: &[
            "--crate-type",
            "bin",
            "--edition",
            "2021",
            "--extern",
            "addition",
            "--extern",
            "subtraction_sys",
        ],
        expected_executables: &["assertion"],
        expected_libraries: &[],
        expected_rlibs: &[],
        // See assertion.rs
        executable_output: Some(b"123456789"),
    },
];

/// Compile the [`SAMPLE_PROGRAMS`] for all `targets`.
///
/// Also execute the sample programs for the platform specified by the
/// `SELFTEST_TARGET` compile-time environment variable.
pub(crate) fn check(
    reporter: &dyn Reporter,
    sysroot: &Path,
    targets: &[Target],
) -> Result<(), Error> {
    targets.iter().try_for_each(|target| check_target(reporter, sysroot, target, SAMPLE_PROGRAMS))
}

fn check_target(
    reporter: &dyn Reporter,
    sysroot: &Path,
    target: &Target,
    programs: &[SampleProgram],
) -> Result<(), Error> {
    let temp = tempfile::Builder::new()
        .prefix("fst-")
        .tempdir()
        .map_err(|error| Error::TemporaryCompilationDirectoryCreationFailed { error })?;

    let ctx = Context::new(target, sysroot, &temp);
    create_tmp_compilation_dir(&ctx.source_dir)?;
    create_tmp_compilation_dir(&ctx.output_dir)?;
    let mut expected_artifacts = ExpectedFiles::new(&ctx.output_dir);

    for program in programs {
        let expected_binary_paths = program
            .expected_executables
            .iter()
            .flat_map(|expected| match target.spec.triple {
                windows if windows.ends_with("-pc-windows-msvc") => {
                    vec![format!("{expected}.exe"), format!("{expected}.pdb")]
                }
                _ => vec![expected.to_string()],
            })
            .collect::<Vec<_>>();
        expected_artifacts.add(expected_binary_paths);

        let expected_library_paths = program
            .expected_libraries
            .iter()
            .map(|expected| match target.spec.triple {
                windows if windows.ends_with("-pc-windows-msvc") => format!("{expected}.lib"),
                _ => format!("lib{expected}.a"),
            })
            .collect::<Vec<_>>();
        expected_artifacts.add(expected_library_paths);

        let expected_rlib_paths = program
            .expected_rlibs
            .iter()
            .map(|expected| format!("lib{expected}.rlib"))
            .collect::<Vec<_>>();
        expected_artifacts.add(expected_rlib_paths);

        compile(&ctx, program)?;
        expected_artifacts.check(program.name)?;

        let should_run = (ctx.target.triple == env::SELFTEST_TARGET)
            .then_some(program.executable_output)
            .flatten();
        if let Some(expected_output) = should_run {
            run(&ctx, program, expected_output)?
        }

        reporter.success(&format!(
            "compiled {}sample program `{}` for target {}",
            if should_run.is_some() { "and ran " } else { "" },
            program.name,
            target.triple
        ));
    }
    Ok(())
}

fn create_tmp_compilation_dir(path: &Path) -> Result<(), Error> {
    std::fs::create_dir_all(path)
        .map_err(|error| Error::TemporaryCompilationDirectoryCreationFailed { error })
}

fn compile(ctx: &Context<'_>, program: &SampleProgram) -> Result<(), Error> {
    let program_path = ctx.source_dir.join(program.name);
    std::fs::write(&program_path, program.contents).map_err(|error| {
        Error::WritingSampleProgramFailed {
            name: program.name.into(),
            dest: program_path.clone(),
            error,
        }
    })?;

    let mut remap_path_prefix = OsString::new();
    remap_path_prefix.push(&ctx.temp_dir);
    remap_path_prefix.push("=/self-test");

    let mut cmd = Command::new(&ctx.rustc);
    cmd.args(["--target", ctx.target.triple]);
    cmd.arg("-L").arg(&ctx.output_dir);
    cmd.arg("--out-dir").arg(&ctx.output_dir);
    cmd.arg("--remap-path-prefix").arg(&remap_path_prefix);
    if !ctx.target.std {
        cmd.args(["--cfg", "selftest_no_std"]);
    }
    cmd.args(program.rustflags);
    cmd.args(&ctx.target.rustflags);
    cmd.arg(&program_path);

    run_command(&mut cmd)
        .map_err(|error| Error::sample_program_compilation_failed(program.name, error))?;

    Ok(())
}

fn run(ctx: &Context<'_>, program: &SampleProgram, expected_output: &[u8]) -> Result<(), Error> {
    // where is it
    let bin_name = program.name.replace(".rs", "");
    let bin_path = ctx.output_dir.join(bin_name);
    // now try and execute it
    let mut cmd = Command::new(bin_path);
    let output = cmd
        .output()
        .map_err(|error| Error::RunningSampleProgramFailed { name: program.name.into(), error })?;
    if output.stdout != expected_output {
        Err(Error::SampleProgramOutputWrong {
            name: program.name.into(),
            expected: expected_output.to_vec(),
            found: output.stdout,
        })
    } else {
        Ok(())
    }
}

struct ExpectedFiles {
    path: PathBuf,
    expected: HashSet<String>,
}

impl ExpectedFiles {
    fn new(path: &Path) -> Self {
        Self { path: path.into(), expected: HashSet::new() }
    }

    fn add(&mut self, files: impl IntoIterator<Item = String>) {
        self.expected.extend(files);
    }

    /// Check that all, and those only, compilation artifacts are present.
    fn check(&self, after_compiling: &str) -> Result<(), Error> {
        let mut currently_expected = self.expected.clone();

        let map_err =
            |error| Error::CompilationArtifactsListingFailed { path: self.path.clone(), error };
        for entry in std::fs::read_dir(&self.path).map_err(map_err)? {
            let entry = entry.map_err(map_err)?;
            let file_name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| Error::NonUtf8Path { path: entry.path() })?
                .to_string();

            if !currently_expected.remove(file_name.as_str()) {
                return Err(Error::UnexpectedCompilationArtifact {
                    name: file_name,
                    after_compiling: after_compiling.into(),
                });
            }
        }

        let mut currently_expected = currently_expected.into_iter().collect::<Vec<_>>();
        currently_expected.sort();

        if let Some(missing_file) = currently_expected.first() {
            Err(Error::MissingCompilationArtifact {
                name: missing_file.to_string(),
                after_compiling: after_compiling.into(),
            })
        } else {
            Ok(())
        }
    }
}

struct Context<'a> {
    target: &'a Target,
    rustc: PathBuf,
    temp_dir: PathBuf,
    source_dir: PathBuf,
    output_dir: PathBuf,
}

impl<'a> Context<'a> {
    fn new(target: &'a Target, sysroot: &Path, temp: &TempDir) -> Self {
        Self {
            target,
            rustc: sysroot.join("bin").join("rustc"),
            temp_dir: temp.path().into(),
            source_dir: temp.path().join("src"),
            output_dir: temp.path().join("out"),
        }
    }
}

struct SampleProgram {
    name: &'static str,
    contents: &'static [u8],
    rustflags: &'static [&'static str],
    /// Executable names on Linux/Mac are `{name}`, on Windows they are `{name}.exe`
    expected_executables: &'static [&'static str],
    /// rlibs are always `{name}.rlib`
    expected_rlibs: &'static [&'static str],
    /// Libraries on Linux/Mac are `lib{name}.a`, on Windows they are `{name}.lib`
    expected_libraries: &'static [&'static str],
    executable_output: Option<&'static [u8]>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkers::Linker;
    use crate::targets::TargetSpec;
    use crate::test_utils::TestUtils;

    #[test]
    fn test_all_sample_program_source_files_are_registered() {
        // This test does not check the functionality of the module, but ensures we don't forget to
        // add new tests to the SAMPLE_PORGRAMS array.
        let mut actual = std::fs::read_dir("sample-programs")
            .expect("sample-programs directory not found")
            .map(|entry| entry.unwrap().path().file_name().unwrap().to_str().unwrap().to_string())
            .collect::<Vec<_>>();
        let mut registered = SAMPLE_PROGRAMS.iter().map(|p| p.name.to_string()).collect::<Vec<_>>();

        actual.sort();
        registered.sort();

        assert_eq!(
            registered, actual,
            "\n\nThe list of sample programs in src/compile.rs is different \
             than the list of files in the filesystem.\n\
             Did you register all the sample programs?\n\n"
        );
    }

    #[test]
    fn test_check_target() {
        // To test the check_target function, we create a fake rustc binary that verifies the
        // correct flags were provided, and if so creates empty files in place of the artifacts.
        const RUSTC_SOURCE: &str = r#"
            fn main() {
                let args = std::env::args().skip(1).collect::<Vec<_>>();
                let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
                match &args[..] {
                    // First invocation
                    [
                        "--target", "x86_64-unknown-linux-gnu",
                        "-L", _,
                        "--out-dir", out_dir,
                        "--remap-path-prefix", _,
                        "--crate-type", "lib",
                        "-C linker=rust-lld",
                        source
                    ] => {
                        assert_eq!("foo.rs", source.rsplit_once(std::path::MAIN_SEPARATOR).unwrap().1);
                        std::fs::write(format!("{out_dir}/libfoo.rlib"), b"").unwrap();
                    }
                    // Second invocation
                    [
                        "--target", "x86_64-unknown-linux-gnu",
                        "-L", _,
                        "--out-dir", out_dir,
                        "--remap-path-prefix", _,
                        "--crate-type", "bin",
                        "-C linker=rust-lld",
                        source
                    ] => {
                        assert_eq!("bar.rs", source.rsplit_once(std::path::MAIN_SEPARATOR).unwrap().1);
                        std::fs::write(format!("{out_dir}/bar"), b"").unwrap();
                    }
                    other => panic!("unexpected args: {other:?}"),
                }
            }
        "#;

        const TEST_PROGRAMS: &[SampleProgram] = &[
            SampleProgram {
                name: "foo.rs",
                contents: b"pub fn foo() {}",
                rustflags: &["--crate-type", "lib"],
                expected_rlibs: &["foo"],
                expected_executables: &[],
                expected_libraries: &[],
                executable_output: None,
            },
            SampleProgram {
                name: "bar.rs",
                contents: b"fn main() {}",
                rustflags: &["--crate-type", "bin"],
                expected_rlibs: &[],
                expected_executables: &["bar"],
                expected_libraries: &[],
                executable_output: None,
            },
        ];

        let target = Target {
            spec: &TargetSpec {
                triple: "x86_64-unknown-linux-gnu",
                std: true,
                linker: Linker::BundledLld,
            },
            rustflags: vec!["-C linker=rust-lld".into()],
        };

        let utils = TestUtils::new();
        utils.bin("rustc").program_source(RUSTC_SOURCE).create();

        check_target(utils.reporter(), utils.sysroot(), &target, TEST_PROGRAMS).unwrap();
    }

    #[test]
    fn test_expected_files() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();

        let create = |name| std::fs::write(dir.join(name), b"").unwrap();

        let mut expected = ExpectedFiles::new(dir);
        expected.add(vec!["foo".to_string(), "bar".to_string()]);
        match expected.check("binary") {
            Err(Error::MissingCompilationArtifact { name, after_compiling }) => {
                assert_eq!("bar", name);
                assert_eq!("binary", after_compiling);
            }
            other => panic!("unexpected result: {other:?}"),
        }

        create("foo");
        create("bar");
        expected.check("binary").unwrap();

        std::fs::remove_file(dir.join("foo")).unwrap();
        match expected.check("binary") {
            Err(Error::MissingCompilationArtifact { name, after_compiling }) => {
                assert_eq!("foo", name);
                assert_eq!("binary", after_compiling);
            }
            other => panic!("unexpected result: {other:?}"),
        }

        create("foo");
        create("baz");
        match expected.check("binary") {
            Err(Error::UnexpectedCompilationArtifact { name, after_compiling }) => {
                assert_eq!("baz", name);
                assert_eq!("binary", after_compiling);
            }
            other => panic!("unexpected result: {other:?}"),
        }

        expected.add(vec!["baz".to_string()]);
        expected.check("binary").unwrap();
    }

    #[test]
    fn test_compile_failed_to_write_program() {
        let tempdir = TempDir::new().unwrap();
        let output_dir = tempdir.path().join("out");

        let utils = TestUtils::new();
        let rustc = utils.bin("rustc").create();

        let context = Context {
            target: &Target {
                spec: &TargetSpec {
                    triple: "x86_64-unknown-linux-gnu",
                    std: true,
                    linker: Linker::BundledLld,
                },
                rustflags: Vec::new(),
            },
            rustc: rustc.clone(),
            temp_dir: tempdir.path().into(),
            source_dir: tempdir.path().join("missing"),
            output_dir,
        };
        let program = SampleProgram {
            name: "example.rs",
            contents: b"fn main() { println!(\"Hello world!\"); }\n",
            rustflags: &[],
            expected_executables: &[],
            expected_libraries: &[],
            expected_rlibs: &[],
            executable_output: None,
        };

        match compile(&context, &program) {
            Err(Error::WritingSampleProgramFailed { name, dest, error }) => {
                assert_eq!("example.rs", name);
                assert_eq!(tempdir.path().join("missing").join("example.rs"), dest);
                assert_eq!(std::io::ErrorKind::NotFound, error.kind());
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn test_compile_failed_invocation() {
        let tempdir = TempDir::new().unwrap();
        let source_dir = tempdir.path().join("src");
        let output_dir = tempdir.path().join("out");

        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::create_dir_all(&output_dir).unwrap();

        let utils = TestUtils::new();
        // Empty expected args will result in the compilation failing.
        let rustc = utils.bin("rustc").expected_args(&[]).create();

        let context = Context {
            target: &Target {
                spec: &TargetSpec {
                    triple: "x86_64-unknown-linux-gnu",
                    std: true,
                    linker: Linker::BundledLld,
                },
                rustflags: Vec::new(),
            },
            rustc: rustc.clone(),
            temp_dir: tempdir.path().into(),
            source_dir,
            output_dir,
        };
        let program = SampleProgram {
            name: "example.rs",
            contents: b"fn main() { println!(\"Hello world!\"); }\n",
            rustflags: &[],
            expected_executables: &[],
            expected_rlibs: &[],
            expected_libraries: &[],
            executable_output: None,
        };

        match compile(&context, &program) {
            Err(Error::SampleProgramCompilationFailed { name, error }) => {
                assert_eq!("example.rs", name);
                assert_eq!(rustc, error.path);
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn test_compile_success_no_std() {
        compile_success_inner(false, &["--cfg", "selftest_no_std"]);
    }

    #[test]
    fn test_compile_success_std() {
        compile_success_inner(true, &[]);
    }

    fn compile_success_inner(std: bool, extra_arg: &[&str]) {
        let tempdir = TempDir::new().unwrap();
        let source_dir = tempdir.path().join("src");
        let output_dir = tempdir.path().join("out");

        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::create_dir_all(&output_dir).unwrap();

        let source_file = source_dir.join("example.rs").to_str().unwrap().to_string();
        let remap_path_prefix = format!("{}=/self-test", tempdir.path().to_str().unwrap());
        let mut args = Vec::new();
        args.extend_from_slice(&[
            "--target",
            "x86_64-unknown-linux-gnu",
            "-L",
            output_dir.to_str().unwrap(),
            "--out-dir",
            output_dir.to_str().unwrap(),
            "--remap-path-prefix",
            &remap_path_prefix,
        ]);
        args.extend_from_slice(extra_arg);
        args.extend_from_slice(&["--extern", "foo", "-Clinker=rust-lld", &source_file]);

        let utils = TestUtils::new();
        let rustc = utils.bin("rustc").expected_args(&args).create();

        let context = Context {
            target: &Target {
                // An if statement is used here because spec must be &'static, which means we
                // cannot dynamically set std depending on the function parameter.
                spec: if std {
                    &TargetSpec {
                        triple: "x86_64-unknown-linux-gnu",
                        linker: Linker::BundledLld,
                        std: true,
                    }
                } else {
                    &TargetSpec {
                        triple: "x86_64-unknown-linux-gnu",
                        linker: Linker::BundledLld,
                        std: false,
                    }
                },
                rustflags: vec!["-Clinker=rust-lld".into()],
            },
            rustc,
            temp_dir: tempdir.path().into(),
            source_dir,
            output_dir,
        };

        let program = SampleProgram {
            name: "example.rs",
            contents: b"fn main() { println!(\"Hello world!\"); }\n",
            rustflags: &["--extern", "foo"],
            expected_executables: &["example"],
            expected_rlibs: &[],
            expected_libraries: &[],
            executable_output: None,
        };

        compile(&context, &program).unwrap();
    }
}
