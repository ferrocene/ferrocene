//@ edition:2018

async fn main() -> Result<i32, ()> {
//~^ ERROR `main` function is not allowed to be `async`
    Ok(1)
}

// ferrocene-annotations: fls_8jb3sjqamdpu
// Program Entry Point
