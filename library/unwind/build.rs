// ferrocene addition
fn main() {
    let target = std::env::var("TARGET").expect("Cargo did not set the TARGET env var");

    if !target.contains("-ferrocenecoretest") {
        return;
    }

    match target.as_str() {
        "aarch64-unknown-ferrocenecoretest" => {}
        "thumbv7em-ferrocenecoretest-eabi" | "thumbv7em-ferrocenecoretest-eabihf" => {
            println!("cargo::rustc-cfg=ferrocenecoretest")
        }
        _ => unimplemented!("extend this `match`"),
    };
}
