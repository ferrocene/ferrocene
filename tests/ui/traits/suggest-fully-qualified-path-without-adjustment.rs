use std::ops::{Deref, DerefMut};

struct Thing;

trait Method<T> {
    fn method(&self) -> T;
    fn mut_method(&mut self) -> T;
}

impl Method<i32> for Thing {
    fn method(&self) -> i32 { 0 }
    fn mut_method(&mut self) -> i32 { 0 }
}

impl Method<u32> for Thing {
    fn method(&self) -> u32 { 0 }
    fn mut_method(&mut self) -> u32 { 0 }
}

trait MethodRef<T> {
    fn by_self(self);
}
impl MethodRef<i32> for &Thing {
    fn by_self(self) {}
}
impl MethodRef<u32> for &Thing {
    fn by_self(self) {}
}

struct DerefsTo<T>(T);
impl<T> Deref for DerefsTo<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for DerefsTo<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut ref_thing = &Thing;
    ref_thing.method();
    //~^ ERROR type annotations needed
    //~| ERROR type annotations needed
    ref_thing.by_self(); //~ ERROR type annotations needed

    let mut mut_thing = &mut Thing;
    mut_thing.method(); //~ ERROR type annotations needed
    mut_thing.mut_method(); //~ ERROR type annotations needed
    mut_thing.by_self(); //~ ERROR type annotations needed

    let mut deref_to = &DerefsTo(Thing);
    deref_to.method(); //~ ERROR type annotations needed
    deref_to.mut_method(); //~ ERROR type annotations needed
    deref_to.by_self(); //~ ERROR type annotations needed

    let mut deref_deref_to = &DerefsTo(DerefsTo(Thing));
    deref_deref_to.method(); //~ ERROR type annotations needed
    deref_deref_to.mut_method(); //~ ERROR type annotations needed
    deref_deref_to.by_self(); //~ ERROR type annotations needed
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_151r19d7xbgz
// Entities
//
// ferrocene-annotations: fls_izl8iuhoz9e0
// Scopes
//
// ferrocene-annotations: fls_6ozthochxz1i
// Binding Scopes
//
// ferrocene-annotations: fls_ftphlagzd2te
// Generic Parameter Scope
//
// ferrocene-annotations: fls_m0z7omni9hp0
// Item Scope
//
// ferrocene-annotations: fls_769b4p8v3cwu
// Label Scope
//
// ferrocene-annotations: fls_kgbi26212eof
// Self Scope
//
// ferrocene-annotations: fls_octf6sf7yso
// Textual Macro Scope
//
// ferrocene-annotations: fls_lnpyb285qdiy
// Scope Hierarchy
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
//
// ferrocene-annotations: fls_ld0ize96cm6m
// Preludes
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_40xoego2thsp
// Resolution
//
// ferrocene-annotations: fls_i6qzga6dyaee
// Path Resolution
//
// ferrocene-annotations: fls_bbso3c45kr9z
// Simple Path Resolution
