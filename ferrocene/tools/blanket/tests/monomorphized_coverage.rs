use std::path::{Path, PathBuf};
use std::process::Command;

use llvm_profparser::CoverageMapping;

const SAMPLE_SOURCE: &str = r#"
macro_rules! make_thing {
    ($name:ident, $t:ty) => {
        pub fn $name(flag: bool, x: $t) -> $t {
            if flag {
                x.wrapping_add(1)
            } else {
                x.wrapping_sub(1)
            }
        }
    };
}

make_thing!(thing_u8, u8);
make_thing!(thing_u32, u32);

fn main() {
    println!("{}", thing_u8(true, 5));
    println!("{}", thing_u32(false, 5));
}
"#;

const TRUE_BRANCH_LINE: usize = 5;
const FALSE_BRANCH_LINE: usize = 7;

struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn rustc_host_triple() -> String {
    let output = Command::new("rustc").arg("-vV").output().expect("failed to run `rustc -vV`");
    let stdout = String::from_utf8(output.stdout).expect("rustc -vV output is not valid utf-8");
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .expect("couldn't find `host:` line in `rustc -vV` output")
        .to_string()
}

fn rustc_sysroot() -> PathBuf {
    let output = Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .expect("failed to run `rustc --print sysroot`");
    PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
}

fn llvm_tool(sysroot: &Path, host: &str, name: &str) -> PathBuf {
    let path = sysroot.join("lib").join("rustlib").join(host).join("bin").join(name);
    assert!(
        path.is_file(),
        "couldn't find `{name}` at {path:?}; install it with `rustup component add llvm-tools`"
    );
    path
}

#[test]
fn monomorphized_functions_get_scoped_coverage() {
    let dir = TempDir(std::env::temp_dir().join(format!(
        "blanket-monomorphized-coverage-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    )));
    let temp_dir = &dir.0;
    std::fs::create_dir_all(temp_dir).unwrap();

    let src = temp_dir.join("sample.rs");
    let bin = temp_dir.join("prog");
    let profraw = temp_dir.join("prog.profraw");
    let profdata = temp_dir.join("prog.profdata");

    // drop the newline after `r#"` before writing to a file
    std::fs::write(&src, SAMPLE_SOURCE.trim_start_matches('\n')).unwrap();

    let host = rustc_host_triple();
    let sysroot = rustc_sysroot();
    let llvm_profdata = llvm_tool(&sysroot, &host, "llvm-profdata");

    let status = Command::new("rustc")
        .args(["--edition", "2024", "-C", "instrument-coverage", "-g", "-o"])
        .arg(&bin)
        .arg(&src)
        .status()
        .expect("failed to run rustc");
    assert!(status.success(), "rustc failed to compile the sample");

    let status = Command::new(&bin)
        .env("LLVM_PROFILE_FILE", &profraw)
        .status()
        .expect("failed to run the compiled sample");
    assert!(status.success(), "the compiled sample exited with a failure");

    let status = Command::new(&llvm_profdata)
        .args(["merge", "--sparse", "-o"])
        .arg(&profdata)
        .arg(&profraw)
        .status()
        .expect("failed to run llvm-profdata");
    assert!(status.success(), "llvm-profdata failed to merge the profile");

    let profile = llvm_profparser::parse(&profdata).expect("failed to parse the merged profdata");
    let objects = vec![bin.clone()];
    let mapping =
        CoverageMapping::new(&objects, &profile, false).expect("failed to read coverage mapping");

    let names: Vec<String> = mapping
        .mapping_info_iter()
        .next()
        .expect("expected coverage mapping info for the compiled object")
        .expect("failed to read coverage mapping info")
        .prof_names
        .into_values()
        .collect();
    let name_u8 = names
        .iter()
        .find(|n| n.contains("thing_u8"))
        .expect("couldn't find thing_u8's linkage name")
        .clone();
    let name_u32 = names
        .iter()
        .find(|n| n.contains("thing_u32"))
        .expect("couldn't find thing_u32's linkage name")
        .clone();
    assert_ne!(name_u8, name_u32, "thing_u8 and thing_u32 must be distinct compiled functions");

    let report = mapping.generate_report().expect("failed to generate coverage report");
    let (_, result) = report
        .files
        .iter()
        .find(|(path, _)| path.file_name().and_then(|f| f.to_str()) == Some("sample.rs"))
        .expect("no coverage entry for sample.rs");

    // Without per-function scoping, `hits` merges regions for both versions
    // of the function based on source location alone.
    //
    // `thing_u8` covers `true` branch while
    // `thing_u32` covers `false` branch
    //
    // But because both functions map to the same source location both branches
    // are assumed as covered
    let merged_hits_on_line = |line: usize| -> usize {
        result.hits.iter().filter(|(loc, _)| loc.line_start == line).map(|(_, &count)| count).sum()
    };

    assert!(
        merged_hits_on_line(TRUE_BRANCH_LINE) > 0,
        "sanity check: merged view should show the true-branch as hit (via thing_u8)"
    );
    assert!(
        merged_hits_on_line(FALSE_BRANCH_LINE) > 0,
        "sanity check: merged view should show the false-branch as hit (via thing_u32)"
    );

    // when we scope coverage counters per generated function the region
    // counters across variants do not get merged
    let scoped_hits_on_line = |function: &str, line: usize| -> usize {
        let scoped = result
            .hits_by_function
            .get(&llvm_profparser::strip_crate_hashes(function))
            .unwrap_or_else(|| panic!("no scoped coverage recorded for {function}"));
        scoped.iter().filter(|(loc, _)| loc.line_start == line).map(|(_, &count)| count).sum()
    };

    // Scoped to thing_u8, only the true-branch (line 5) should show hits.
    assert!(
        scoped_hits_on_line(&name_u8, TRUE_BRANCH_LINE) > 0,
        "thing_u8 should show its own true-branch as covered"
    );
    assert_eq!(
        scoped_hits_on_line(&name_u8, FALSE_BRANCH_LINE),
        0,
        "thing_u8 never executes the false-branch; scoping to it must not borrow \
         thing_u32's hits on that line"
    );

    // Scoped to thing_u32, only the false-branch (line 7) should show hits.
    assert!(
        scoped_hits_on_line(&name_u32, FALSE_BRANCH_LINE) > 0,
        "thing_u32 should show its own false-branch as covered"
    );
    assert_eq!(
        scoped_hits_on_line(&name_u32, TRUE_BRANCH_LINE),
        0,
        "thing_u32 never executes the true-branch; scoping to it must not borrow \
         thing_u8's hits on that line"
    );
}
