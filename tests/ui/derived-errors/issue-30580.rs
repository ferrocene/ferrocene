// Test that we do not see uninformative region-related errors
// when we get some basic type-checking failure. See #30580.

pub struct Foo { a: u32 }
pub struct Pass<'a, 'tcx: 'a>(&'a mut &'a (), &'a &'tcx ());

impl<'a, 'tcx> Pass<'a, 'tcx>
{
    pub fn tcx(&self) -> &'a &'tcx () { self.1 }
    fn lol(&mut self, b: &Foo)
    {
        b.c; //~ ERROR no field `c` on type `&Foo`
        self.tcx();
    }
}

fn main() {}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
