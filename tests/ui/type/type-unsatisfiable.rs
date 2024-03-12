//@ revisions: lib usage
//@[lib] compile-flags: --crate-type=lib
//@[lib] build-pass

use std::ops::Sub;
trait Vector2 {
    type ScalarType;

    fn from_values(x: Self::ScalarType, y: Self::ScalarType) -> Self
    where
        Self: Sized;

    fn x(&self) -> Self::ScalarType;
    fn y(&self) -> Self::ScalarType;
}

impl<T> Sub for dyn Vector2<ScalarType = T>
where
    T: Sub<Output = T>,
    (dyn Vector2<ScalarType = T>): Sized,
{
    type Output = dyn Vector2<ScalarType = T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_values(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

struct Vec2 {
    x: i32,
    y: i32,
}

impl Vector2 for Vec2 {
    type ScalarType = i32;

    fn from_values(x: Self::ScalarType, y: Self::ScalarType) -> Self
    where
        Self: Sized,
    {
        Self { x, y }
    }

    fn x(&self) -> Self::ScalarType {
        self.x
    }
    fn y(&self) -> Self::ScalarType {
        self.y
    }
}

#[cfg(usage)]
fn main() {
    let hey: Box<dyn Vector2<ScalarType = i32>> = Box::new(Vec2 { x: 1, y: 2 });
    let word: Box<dyn Vector2<ScalarType = i32>> = Box::new(Vec2 { x: 1, y: 2 });

    let bar = *hey - *word;
    //[usage]~^ ERROR cannot subtract
}

// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
