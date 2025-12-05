//@ build-fail
//@ normalize-stderr: "\[&usize; \d+\]" -> "[&usize; usize::MAX]"

#[cfg(target_pointer_width = "64")]
fn main() {
    let n = 0_usize;
    let a: Box<_> = Box::new([&n; 0xF000000000000000_usize]);
    println!("{}", a[0xFFFFFF_usize]);
}

#[cfg(target_pointer_width = "32")]
fn main() {
    let n = 0_usize;
    let a: Box<_> = Box::new([&n; 0xFFFFFFFF_usize]);
    println!("{}", a[0xFFFFFF_usize]);
}

//~? ERROR are too big for the target architecture
<<<<<<< HEAD
//~? ERROR are too big for the target architecture

// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
||||||| ac0aff2115f
//~? ERROR are too big for the target architecture
=======
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
