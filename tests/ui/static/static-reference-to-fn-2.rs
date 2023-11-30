fn id<T>(x: T) -> T { x }

struct StateMachineIter<'a> {
    statefn: &'a StateMachineFunc<'a>
}

type StateMachineFunc<'a> = fn(&mut StateMachineIter<'a>) -> Option<&'static str>;

impl<'a> Iterator for StateMachineIter<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<&'static str> {
        return  (*self.statefn)(self);
    }
}

fn state1(self_: &mut StateMachineIter) -> Option<&'static str> {
    self_.statefn = &id(state2 as StateMachineFunc);
    //~^ ERROR temporary value dropped while borrowed
    return Some("state1");
}

fn state2(self_: &mut StateMachineIter) -> Option<(&'static str)> {
    self_.statefn = &id(state3 as StateMachineFunc);
    //~^ ERROR temporary value dropped while borrowed
    return Some("state2");
}

fn state3(self_: &mut StateMachineIter) -> Option<(&'static str)> {
    self_.statefn = &id(finished as StateMachineFunc);
    //~^ ERROR temporary value dropped while borrowed
    return Some("state3");
}

fn finished(_: &mut StateMachineIter) -> Option<(&'static str)> {
    return None;
}

fn state_iter() -> StateMachineIter<'static> {
    StateMachineIter {
    //~^ ERROR cannot return value referencing temporary value
        statefn: &id(state1 as StateMachineFunc)
    }
}


fn main() {
    let mut it = state_iter();
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_g0uyl7qw4c7w
// Parenthesized Expressions
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
