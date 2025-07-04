//! Various traits used to restrict intrinsics to not-completely-wrong types.

/// Types with a built-in dereference operator in runtime MIR,
/// aka references and raw pointers.
///
/// # Safety
/// Must actually *be* such a type.
pub unsafe trait BuiltinDeref: Sized {
    type Pointee: ?Sized;
}

unsafe impl<T: ?Sized> BuiltinDeref for &mut T {
    type Pointee = T;
}
unsafe impl<T: ?Sized> BuiltinDeref for &T {
    type Pointee = T;
}
unsafe impl<T: ?Sized> BuiltinDeref for *mut T {
    type Pointee = T;
}
unsafe impl<T: ?Sized> BuiltinDeref for *const T {
    type Pointee = T;
}

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
    type Output = *const U;
}
