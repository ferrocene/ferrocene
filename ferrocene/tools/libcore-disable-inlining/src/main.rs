use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const INLINE: &str = "#[inline]";
const INLINE_ALWAYS: &str = "#[inline(always)]";
const INLINE_NEVER: &str = "#[inline(never)]";
const USAGE: &str = "usage: stop-inlining [DIR]\n\nerror: ";

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let dir_path = get_dir_path(&args);
    handle_dir(dir_path);
}

fn get_dir_path(args: &[String]) -> PathBuf {
    let dir_path: PathBuf = match args.len() {
        0 => unreachable!(),
        1 => panic!("{USAGE}missing argument DIR"),
        2 => args[1].clone().into(),
        _ => panic!("{USAGE}too many arguments"),
    };

    if !dir_path.is_dir() {
        panic!("{USAGE}DIR is not a directory")
    }

    dir_path
}

fn handle_dir(dir_path: impl AsRef<Path>) {
    let dir = fs::read_dir(&dir_path).unwrap_or_else(|_| panic!("{:?}", dir_path.as_ref()));

    for file in dir {
        let path = file.unwrap().path();

        if path.is_dir() {
            handle_dir(&path); // recurse into subdirectories
        } else if !is_rust_file(&path) {
            continue; // skip non Rust files
        } else {
            handle_file(&path);
        }
    }
}

fn handle_file(file_path: &Path) {
    eprintln!("{file_path:?}");
    let file_content = fs::read_to_string(file_path).unwrap();

    for old_attr in [INLINE, INLINE_ALWAYS] {
        replace_attributes(file_path, &file_content, old_attr, INLINE_NEVER)
    }
}

fn replace_attributes(file_path: &Path, file_content: &str, old_attr: &str, new_attr: &str) {
    let attr_locations = find_all(&file_content, old_attr);
    if attr_locations.is_empty() {
        return; // the file contains no attributes
    }

    if replacing_attributes_all_together_succeeds(file_path, old_attr, new_attr, &file_content) {
        return;
    } else {
        replace_attributes_one_by_one(file_path, old_attr, new_attr, file_content, attr_locations);
    }
}

fn replacing_attributes_all_together_succeeds(
    file_path: &Path,
    old_attr: &str,
    new_attr: &str,
    file_content: &str,
) -> bool {
    let new_file_content = file_content.replace(old_attr, new_attr);
    fs::write(file_path, &new_file_content).unwrap();

    if it_still_compiles() {
        true
    } else {
        // revert change
        fs::write(file_path, file_content).unwrap();
        false
    }
}

fn replace_attributes_one_by_one(
    file_path: &Path,
    old_attr: &str,
    new_attr: &str,
    file_content: &str,
    attr_locations: Vec<usize>,
) {
    let mut file_content = file_content.to_string();

    // due to difference in length between the old and new attribute, we need
    // to carry an offset
    let mut accumulated_offset: i32 = 0;
    let len_diff: i32 = to_i32(new_attr.len()) - to_i32(old_attr.len());

    for attr_start in attr_locations {
        let attr_start = to_usize(to_i32(attr_start) + accumulated_offset);

        // apply change
        replace_attr(&mut file_content, old_attr, new_attr, attr_start);
        fs::write(file_path, &file_content).unwrap();

        if it_still_compiles() {
            accumulated_offset += len_diff;
        } else {
            // revert change
            replace_attr(&mut file_content, new_attr, old_attr, attr_start);
            fs::write(file_path, &file_content).unwrap();
        }
    }
}

fn is_rust_file(file_path: &Path) -> bool {
    file_path.extension().unwrap() == "rs"
}

// Note: We want to build compiler_builtins, but there is no `./x build
// compiler_builtins`. Therefore we build library/alloc because it is the
// fastest target that includes compiler_builtins.
fn it_still_compiles() -> bool {
    Command::new("./x")
        .args(["build", "library/alloc", "--stage", "0"])
        .output()
        .unwrap()
        .status
        .success()
}

/// Returns the byte indicies of all occurences of the pattern in this string
/// slice.
///
/// Returns an empty Vec if the pattern doesn't match.
fn find_all(s: &str, pat: &str) -> Vec<usize> {
    let mut indicies = Vec::new();
    let mut remaining_s = s;
    let mut total_index = 0;

    while let Some(index_in_remaining_s) = remaining_s.find(pat) {
        let index_in_s = total_index + index_in_remaining_s;
        indicies.push(index_in_s);

        let index_after_pattern = index_in_remaining_s + pat.len();
        remaining_s = &remaining_s[index_after_pattern..];

        total_index += index_after_pattern;
    }
    indicies
}

fn replace_attr(s: &mut String, old_attr: &str, new_attr: &str, attr_start: usize) {
    let attr_range = attr_start..attr_start + old_attr.len();
    s.replace_range(attr_range, new_attr);
}

fn to_i32(n: usize) -> i32 {
    n.try_into().unwrap_or_else(|_| panic!("{n}"))
}

fn to_usize(n: i32) -> usize {
    n.try_into().unwrap_or_else(|_| panic!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_1() {
        // Arrange
        let s = "aaaabaa";
        for (pat, correct) in [
            ("a", [0, 1, 2, 3, 5, 6].as_slice()),
            ("aa", [0, 2, 5].as_slice()),
            ("aaa", [0].as_slice()),
            ("b", [4].as_slice()),
            ("c", [].as_slice()),
        ] {
            // Act
            let result = find_all(s, pat);

            // Assert
            assert_eq!(result, correct);
        }
    }

    #[test]
    fn test_find_all_2() {
        // Arrange
        let s = TEST_FILE;
        for (pat, correct) in [
            (INLINE, [5, 151].as_slice()),
            (INLINE_ALWAYS, &[38]),
            (INLINE_NEVER, &[79]),
            ("fn", &[19, 60, 100, 136, 183]),
        ] {
            // Act
            let result = find_all(s, pat);

            // Assert
            assert_eq!(result, correct);
        }
    }

    #[test]
    fn test_replace_attr() {
        // Arrange
        let mut s = TEST_FILE.to_string();
        let old_attr = INLINE;
        let new_attr = INLINE_NEVER;

        // Act
        replace_attr(&mut s, old_attr, new_attr, 5);

        // Assert
        let correct = TEST_FILE.replacen(old_attr, new_attr, 1);
        assert_eq!(s, correct);
    }

    const TEST_FILE: &str = "
    #[inline]
    fn a() {}
    
    #[inline(always)]
    fn b() {}
    
    #[inline(never)]
    fn c() {}
    
    #[no_mangle]
    fn d() {}

    #[inline] // with a comment
    fn e() {}
    ";
}
