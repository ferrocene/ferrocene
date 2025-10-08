use std::io::Read;

use llvm_profparser::instrumentation_profile;
use rustc_demangle::demangle;

fn main() -> anyhow::Result<()> {
    let mut bytes = vec![];
    std::io::stdin().read_to_end(&mut bytes)?;
    let profile = instrumentation_profile::parse_bytes(&bytes)?;
    assert!(!profile.is_byte_coverage());

    for record in profile.records() {
        println!("{}", demangle(record.name.as_ref().unwrap()));
    }

    println!("{profile:?}");
    todo!("everything else");
    Ok(())
}
