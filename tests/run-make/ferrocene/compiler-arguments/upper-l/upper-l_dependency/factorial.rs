extern crate operations;

pub fn factorial(value: i32) -> i32 {
    match value {
        i32::MIN ..= 1 => {
            1
        }
        _ => {
            operations::times(value, factorial(operations::minus(value, 1)))
        }
    }
}
