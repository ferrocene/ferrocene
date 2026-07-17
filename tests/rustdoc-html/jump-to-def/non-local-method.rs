//@ compile-flags: -Zunstable-options --generate-link-to-definition

#![crate_name = "foo"]

//@ has 'src/foo/non-local-method.rs.html'

//@ has - '//a[@href="{{channel}}/alloc/boxed/struct.Box.html"]' 'std::boxed::Box'
use std::boxed::Box;
//@ has - '//a[@href="{{channel}}/std/io/trait.Read.html"]' 'std::io::Read'
use std::io::Read;
//@ has - '//a[@href="{{channel}}/std/io/index.html"]' 'std::io'
use std::io;
//@ has - '//a[@href="{{channel}}/std/process/fn.exit.html"]' 'std::process::exit'
use std::process::exit;
use std::cmp::Ordering;
use std::marker::PhantomData;

pub fn bar2<T: Read>(readable: T) {
    //@ has - '//a[@href="{{channel}}/std/io/trait.Read.html#tymethod.read"]' 'read'
    let _ = readable.read(&mut []);
}

pub fn bar() {
    //@ has - '//a[@href="{{channel}}/alloc/boxed/struct.Box.html"]' 'Box'
    //@ has - '//a[@href="{{channel}}/alloc/boxed/struct.Box.html#method.new"]' 'new'
    let _ = Box::new(0);
    //@ has - '//a[@href="#48"]' 'local_private'
    local_private();
}

pub fn extern_call() {
    //@ has - '//a[@href="{{channel}}/std/process/fn.exit.html"]' 'exit'
    exit(0);
}

pub fn macro_call() {
    //@ has - '//a[@href="{{channel}}/core/macro.todo.html"]' 'todo!'
    todo!("anything at all");
}

pub fn variant() {
    //@ has - '//a[@href="{{channel}}/core/cmp/enum.Ordering.html#variant.Less"]' 'Less'
    let _ = Ordering::Less;
    //@ has - '//a[@href="{{channel}}/core/marker/struct.PhantomData.html"]' 'PhantomData'
    let _: PhantomData::<usize> = PhantomData;
}

fn local_private() {}
