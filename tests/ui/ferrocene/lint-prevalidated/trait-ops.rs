// Don't use anything from core here.
// If you do, it will break once we start annotating more of core.
// You can still use traits as long as the impls use custom types.
// In particular do NOT use numerics anywhere here,
// nor any syntax in core::ops unless it's with an ADT.

//@ revisions: dedup no-dedup
//@[dedup] compile-flags: -Z deduplicate-diagnostics=yes

#![crate_type = "lib"]
#![no_std]
#![deny(ferrocene::uncertified)]
#![feature(try_trait_v2)]

use core::ops::*;

#[derive(Copy, Clone)]
struct Unvalidated;

macro_rules! impl_traits {
    ($type: path => [
        $(|)?
        $( $trait: ident $(< $generic:ident >)? =>
            $( type $assoc:ident; )*
            $(
                fn $name:ident($(& $($none1:lifetime)?)?
                               $(mut $($none2:lifetime)?)?
                               self $(,)?
                                $($ty:ty),* )
                    $( -> $ret:ty )?
            );*
            $(;)?
        )|*
    ]) => {
        $(
        impl $(<$generic>)? $trait $(<$generic>)? for $type {
            $( type $assoc = Self; )?
            $(
                fn $name ( $(& $($none1)?)? $(mut $($none2)?)? self,
                           $(_: $ty),* )
                    $(-> $ret)? {
                    loop {}
                }
            )*
        }
        )*
    }
}

impl_traits! {
    Unvalidated => [
        | Add => type Output; fn add(self, Self) -> Self;
        | AddAssign => fn add_assign(&mut self, Self);
        | BitAnd => type Output; fn bitand(self, Self) -> Self;
        | BitAndAssign => fn bitand_assign(&mut self, Self);
        | BitOr => type Output; fn bitor(self, Self) -> Self;
        | BitOrAssign => fn bitor_assign(&mut self, Self);
        | BitXor => type Output; fn bitxor(self, Self) -> Self;
        | BitXorAssign => fn bitxor_assign(&mut self, Self);
        | Deref => type Target; fn deref(&self) -> &Self;
        | DerefMut => fn deref_mut(&mut self) -> &mut Self;
        | Div => type Output; fn div(self, Self) -> Self;
        | DivAssign => fn div_assign(&mut self, Self);
        | Index<Idx> => type Output; fn index(&self, Idx) -> &Self;
        | IndexMut<Idx> => fn index_mut(&mut self, Idx) -> &mut Self;
        | Mul => type Output; fn mul(self, Self) -> Self;
        | MulAssign => fn mul_assign(&mut self, Self);
        | Neg => type Output; fn neg(self) -> Self;
        | Not => type Output; fn not(self) -> Self;
        | PartialEq => fn eq(&self, &Self) -> bool; fn ne(&self, &Self) -> bool;
        | Rem => type Output; fn rem(self, Self) -> Self;
        | RemAssign => fn rem_assign(&mut self, Self);
        | Shl => type Output; fn shl(self, Self) -> Self;
        | ShlAssign => fn shl_assign(&mut self, Self);
        | Shr => type Output; fn shr(self, Self) -> Self;
        | ShrAssign => fn shr_assign(&mut self, Self);
        | Sub => type Output; fn sub(self, Self) -> Self;
        | SubAssign => fn sub_assign(&mut self, Self);
    ]
}

#[ferrocene::prevalidated]
fn ops() {
    let mut x = Unvalidated;
    x + x; //~ ERROR unvalidated
    x += x; //~ ERROR unvalidated
    x - x; //~ ERROR unvalidated
    x * x; //~ ERROR unvalidated
    x / x; //~ ERROR unvalidated
    x | x; //~ ERROR unvalidated
    x & x; //~ ERROR unvalidated
    x ^ x; //~ ERROR unvalidated
    x % x; //~ ERROR unvalidated
    x << x; //~ ERROR unvalidated
    x >> x; //~ ERROR unvalidated
    -x; //~ ERROR unvalidated
    !x; //~ ERROR unvalidated
    x == x; //~ ERROR unvalidated
    *x; //~ ERROR unvalidated
    *x = Unvalidated; //~ ERROR unvalidated

    x != x; //~ ERROR unvalidated

    // A `x..x` range does not actually call `start_bound`, so we can't test it here.
}

impl Try for Unvalidated {
    type Output = ();
    type Residual = ();
    fn from_output(x: Self::Output) -> Self { Unvalidated }
    fn branch(self) -> ControlFlow<(), Self::Output> { ControlFlow::Continue(()) }
}

impl FromResidual<()> for Unvalidated {
    fn from_residual(_: ()) -> Self { Unvalidated }
}

#[ferrocene::prevalidated]
fn ops2() -> Unvalidated {
    let x = Unvalidated; // ok: ctor
    x?;
    //~^ ERROR unvalidated
    //~^^ ERROR unvalidated
    loop {}
}
