use std::path::PathBuf;

pub fn env_path(var: &str) -> PathBuf {
    let var = format!("COVERAGE_REPORT_{var}");
    if let Some(content) = std::env::var_os(&var) {
        content.into()
    } else {
        panic!("missing enviroment variable {var}");
    }
}
