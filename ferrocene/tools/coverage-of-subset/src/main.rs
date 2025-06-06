use std::{fs, path::Path, sync::LazyLock};

/// Uncertified code is marked with this attribute
const MARKER: &str = "#[cfg(not(feature = \"ferrocene_certified\"))]";
/// Mark code with this to stop rustc from instrumenting it
const TO_BE_INSERTED: &str = "#[cfg_attr(not(bootstrap), coverage(off))]";
// Note: Has to be a lock to be used in a static. Will only ever be called from
// one thread, so it should not be blocking in reality.
static MARKER_PLUS_TO_BE_INSERTED: LazyLock<String> =
    LazyLock::new(|| format!("{MARKER}\n{TO_BE_INSERTED}"));

fn main() {
    // The path to inject the coverage(off) attributes.
    // Note: hardcoded because it is only used for libcore atm. Can be made
    // into a CLI argument if necessary.
    let path = "library/core";
    inject_into_dir(path);
}

fn inject_into_dir(dir_path: impl AsRef<Path>) {
    let dir = fs::read_dir(&dir_path).expect(&format!("{:?}", dir_path.as_ref()));

    for file in dir {
        let file_path = file.unwrap().path();

        if file_path.is_dir() {
            inject_into_dir(&file_path); // recurse into subdirectories
        } else if !is_rust_file(&file_path) {
            continue; // skip non Rust files
        } else {
            let mut file_content = fs::read_to_string(&file_path).unwrap();
            file_content = inject_coverage_attribute(&file_content);
            fs::write(file_path, file_content).unwrap();
        }
    }
}

fn inject_coverage_attribute(file_content: &str) -> String {
    file_content.replace(MARKER, &MARKER_PLUS_TO_BE_INSERTED)
}

fn is_rust_file(file_path: &Path) -> bool {
    file_path.extension().unwrap() == "rs"
}
