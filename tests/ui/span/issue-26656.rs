// Issue #26656: Verify that trait objects cannot bypass dropck.

// Using this instead of Fn etc. to take HRTB out of the equation.
trait Trigger<B> { fn fire(&self, b: &mut B); }
impl<B: Button> Trigger<B> for () {
    fn fire(&self, b: &mut B) {
        b.push();
    }
}

// Still unsound Zook
trait Button { fn push(&self); }
struct Zook<B> { button: B, trigger: Box<dyn Trigger<B>+'static> }

impl<B> Drop for Zook<B> {
    fn drop(&mut self) {
        self.trigger.fire(&mut self.button);
    }
}

// AND
struct Bomb { usable: bool }
impl Drop for Bomb { fn drop(&mut self) { self.usable = false; } }
impl Bomb { fn activate(&self) { assert!(self.usable) } }

enum B<'a> { HarmlessButton, BigRedButton(&'a Bomb) }
impl<'a> Button for B<'a> {
    fn push(&self) {
        if let B::BigRedButton(borrowed) = *self {
            borrowed.activate();
        }
    }
}

fn main() {
    let (mut zook, ticking);
    zook = Zook { button: B::HarmlessButton,
                  trigger: Box::new(()) };
    ticking = Bomb { usable: true };
    zook.button = B::BigRedButton(&ticking);
}
//~^^ ERROR `ticking` does not live long enough

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
