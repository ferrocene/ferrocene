struct A;

fn main() {
    let a = A;

    a + a; //~ ERROR cannot add `A` to `A`

    a - a; //~ ERROR cannot subtract `A` from `A`

    a * a; //~ ERROR cannot multiply `A` by `A`

    a / a; //~ ERROR cannot divide `A` by `A`

    a % a; //~ ERROR cannot mod `A` by `A`

    a & a; //~ ERROR no implementation for `A & A`

    a | a; //~ ERROR no implementation for `A | A`

    a << a; //~ ERROR no implementation for `A << A`

    a >> a; //~ ERROR no implementation for `A >> A`

    a == a; //~ ERROR binary operation `==` cannot be applied to type `A`

    a != a; //~ ERROR binary operation `!=` cannot be applied to type `A`

    a < a; //~ ERROR binary operation `<` cannot be applied to type `A`

    a <= a; //~ ERROR binary operation `<=` cannot be applied to type `A`

    a > a; //~ ERROR binary operation `>` cannot be applied to type `A`

    a >= a; //~ ERROR binary operation `>=` cannot be applied to type `A`
}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
//
// ferrocene-annotations: fls_abp6tjbz8tpn
// Bit Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
