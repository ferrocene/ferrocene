fn main() {
    fn f(a: [u8; u32::DOESNOTEXIST]) {}
    //~^ ERROR no associated item named `DOESNOTEXIST` found for type `u32`
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
