fn main() {
    static BUG: fn(_) -> u8 = |_| 8;
    //~^ ERROR the placeholder `_` is not allowed within types on item signatures for statics
}

// ferrocene-annotations: fls_xztr1kebz8bo
// Function Pointer Types
