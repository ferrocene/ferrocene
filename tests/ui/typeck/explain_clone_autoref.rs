struct NotClone;

fn main() {
    clone_thing(&NotClone);
}

fn clone_thing(nc: &NotClone) -> NotClone {
    //~^ NOTE expected `NotClone` because of return type
    nc.clone()
    //~^ ERROR mismatched type
    //~| NOTE `NotClone` does not implement `Clone`, so `&NotClone` was cloned instead
    //~| NOTE expected struct `NotClone`, found `&NotClone`
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
