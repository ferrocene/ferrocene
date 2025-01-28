extern "C" {
    fn test() -> f32;
}

fn main() {
    assert_eq!(unsafe { test() }, 114.0);
}
