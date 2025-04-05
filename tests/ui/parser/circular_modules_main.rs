#[path = "circular_modules_hello.rs"]
mod circular_modules_hello;

pub fn hi_str() -> String {
    "Hi!".to_string()
}

fn main() {
    circular_modules_hello::say_hello();
}

<<<<<<< HEAD
// ferrocene-annotations: fls_1zbaajz5prpn
// Attribute path
=======
//~? ERROR circular modules: $DIR/circular_modules_main.rs -> $DIR/circular_modules_hello.rs -> $DIR/circular_modules_main.rs
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
