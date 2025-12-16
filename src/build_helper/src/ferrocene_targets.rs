use std::collections::BTreeMap;
use std::sync::LazyLock;

static CERTIFIED_RUNTIME_TO_REGULAR: LazyLock<BTreeMap<&str, &str>> = LazyLock::new(|| {
    BTreeMap::from([
        ("x86_64-ferrocene-none", "x86_64-unknown-none"),
        ("aarch64-ferrocene-none", "aarch64-unknown-none"),
        ("thumbv7em-ferrocene-none-eabi", "thumbv7em-none-eabi"),
        ("thumbv7em-ferrocene-none-eabihf", "thumbv7em-none-eabihf"),
    ])
});

pub fn has_certified_runtime<S: AsRef<str>>(target: S) -> bool {
    CERTIFIED_RUNTIME_TO_REGULAR.contains_key(target.as_ref())
}

pub fn replace_certified_runtime<S: AsRef<str>>(target: S) -> String {
    CERTIFIED_RUNTIME_TO_REGULAR
        .get(target.as_ref())
        .copied()
        .unwrap_or_else(|| target.as_ref())
        .to_owned()
}
