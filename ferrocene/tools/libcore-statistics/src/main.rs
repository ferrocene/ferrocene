mod render;
mod stats;
mod visitor;

use crate::stats::FunctionsCollector;
use crate::visitor::Visitor;
use anyhow::Error;
use rustdoc_types::Crate;

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let [_, path] = args.as_slice() else {
        eprintln!("one argument required: path to json");
        std::process::exit(1);
    };

    let contents = std::fs::read(&path)?;
    let root: Crate = serde_json::from_slice(&contents)?;

    let mut collector = FunctionsCollector::new();
    collector.visit_crate(&root);

    println!("Module\tPath\tKind\tImpl\tVisibility");
    for item in &collector.found {
        println!(
            "{}\t{}\t{}\t{}\t{}",
            item.module,
            item.name,
            item.kind,
            item.impl_.as_deref().unwrap_or(""),
            if item.public { "public" } else { "private" },
        );
    }

    Ok(())
}
