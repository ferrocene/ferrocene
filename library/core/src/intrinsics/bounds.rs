//! Various traits used to restrict intrinsics to not-completely-wrong types.

use crate::marker::PointeeSized;

/// Types with a built-in dereference operator in runtime MIR,
/// aka references and raw pointers.
///
/// # Safety
/// Must actually *be* such a type.
pub unsafe trait BuiltinDeref: Sized {
    type Pointee: PointeeSized;
}

unsafe impl<T: PointeeSized> BuiltinDeref for &mut T {
    type Pointee = T;
}
unsafe impl<T: PointeeSized> BuiltinDeref for &T {
    type Pointee = T;
}
unsafe impl<T: PointeeSized> BuiltinDeref for *mut T {
    type Pointee = T;
}
unsafe impl<T: PointeeSized> BuiltinDeref for *const T {
    type Pointee = T;
}

<<<<<<< HEAD
#[cfg(not(feature = "ferrocene_certified"))]
pub trait ChangePointee<U: ?Sized>: BuiltinDeref {
    type Output;
}
#[cfg(not(feature = "ferrocene_certified"))]
impl<'a, T: ?Sized + 'a, U: ?Sized + 'a> ChangePointee<U> for &'a mut T {
    type Output = &'a mut U;
}
#[cfg(not(feature = "ferrocene_certified"))]
impl<'a, T: ?Sized + 'a, U: ?Sized + 'a> ChangePointee<U> for &'a T {
    type Output = &'a U;
}
#[cfg(not(feature = "ferrocene_certified"))]
impl<T: ?Sized, U: ?Sized> ChangePointee<U> for *mut T {
    type Output = *mut U;
}
#[cfg(not(feature = "ferrocene_certified"))]
impl<T: ?Sized, U: ?Sized> ChangePointee<U> for *const T {
=======
pub trait ChangePointee<U: PointeeSized>: BuiltinDeref {
    type Output;
}
impl<'a, T: PointeeSized + 'a, U: PointeeSized + 'a> ChangePointee<U> for &'a mut T {
    type Output = &'a mut U;
}
impl<'a, T: PointeeSized + 'a, U: PointeeSized + 'a> ChangePointee<U> for &'a T {
    type Output = &'a U;
}
impl<T: PointeeSized, U: PointeeSized> ChangePointee<U> for *mut T {
    type Output = *mut U;
}
impl<T: PointeeSized, U: PointeeSized> ChangePointee<U> for *const T {
>>>>>>> main
    type Output = *const U;
}
