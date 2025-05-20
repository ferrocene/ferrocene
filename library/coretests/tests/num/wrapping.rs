use core::num::Wrapping;

macro_rules! wrapping_operation {
    ($result:expr, $lhs:ident $op:tt $rhs:expr) => {
        assert_eq!($result, $lhs $op $rhs);
        assert_eq!($result, &$lhs $op $rhs);
        assert_eq!($result, $lhs $op &$rhs);
        assert_eq!($result, &$lhs $op &$rhs);
    };
    ($result:expr, $op:tt $expr:expr) => {
        assert_eq!($result, $op $expr);
        assert_eq!($result, $op &$expr);
    };
}

macro_rules! wrapping_assignment {
    ($result:expr, $lhs:ident $op:tt $rhs:expr) => {
        let mut lhs1 = $lhs;
        lhs1 $op $rhs;
        assert_eq!($result, lhs1);

        let mut lhs2 = $lhs;
        lhs2 $op &$rhs;
        assert_eq!($result, lhs2);
    };
}

macro_rules! wrapping_test {
    ($fn_name:ident, $type:ty, $min:expr, $max:expr) => {
        #[test]
        fn $fn_name() {
            let zero: Wrapping<$type> = Wrapping(0);
            let one: Wrapping<$type> = Wrapping(1);
            let min: Wrapping<$type> = Wrapping($min);
            let max: Wrapping<$type> = Wrapping($max);

            wrapping_operation!(min, max + one);
            wrapping_assignment!(min, max += one);
            wrapping_operation!(max, min - one);
            wrapping_assignment!(max, min -= one);
            wrapping_operation!(max, max * one);
            wrapping_assignment!(max, max *= one);
            wrapping_operation!(max, max / one);
            wrapping_assignment!(max, max /= one);
            wrapping_operation!(zero, max % one);
            wrapping_assignment!(zero, max %= one);
            wrapping_operation!(zero, zero & max);
            wrapping_assignment!(zero, zero &= max);
            wrapping_operation!(max, zero | max);
            wrapping_assignment!(max, zero |= max);
            wrapping_operation!(zero, max ^ max);
            wrapping_assignment!(zero, max ^= max);
            wrapping_operation!(zero, zero << 1usize);
            wrapping_assignment!(zero, zero <<= 1usize);
            wrapping_operation!(zero, zero >> 1usize);
            wrapping_assignment!(zero, zero >>= 1usize);
            wrapping_operation!(zero, -zero);
            wrapping_operation!(max, !min);
        }
    };
}

wrapping_test!(test_wrapping_i8, i8, i8::MIN, i8::MAX);
wrapping_test!(test_wrapping_i16, i16, i16::MIN, i16::MAX);
wrapping_test!(test_wrapping_i32, i32, i32::MIN, i32::MAX);
wrapping_test!(test_wrapping_i64, i64, i64::MIN, i64::MAX);
wrapping_test!(test_wrapping_i128, i128, i128::MIN, i128::MAX);
wrapping_test!(test_wrapping_isize, isize, isize::MIN, isize::MAX);
wrapping_test!(test_wrapping_u8, u8, u8::MIN, u8::MAX);
wrapping_test!(test_wrapping_u16, u16, u16::MIN, u16::MAX);
wrapping_test!(test_wrapping_u32, u32, u32::MIN, u32::MAX);
wrapping_test!(test_wrapping_u64, u64, u64::MIN, u64::MAX);
wrapping_test!(test_wrapping_u128, u128, u128::MIN, u128::MAX);
wrapping_test!(test_wrapping_usize, usize, usize::MIN, usize::MAX);

#[test]
fn wrapping_int_api() {
    assert_eq!(i8::MAX.wrapping_add(1), i8::MIN);
    assert_eq!(i16::MAX.wrapping_add(1), i16::MIN);
    assert_eq!(i32::MAX.wrapping_add(1), i32::MIN);
    assert_eq!(i64::MAX.wrapping_add(1), i64::MIN);
    assert_eq!(isize::MAX.wrapping_add(1), isize::MIN);

    assert_eq!(i8::MIN.wrapping_sub(1), i8::MAX);
    assert_eq!(i16::MIN.wrapping_sub(1), i16::MAX);
    assert_eq!(i32::MIN.wrapping_sub(1), i32::MAX);
    assert_eq!(i64::MIN.wrapping_sub(1), i64::MAX);
    assert_eq!(isize::MIN.wrapping_sub(1), isize::MAX);

    assert_eq!(u8::MAX.wrapping_add(1), u8::MIN);
    assert_eq!(u16::MAX.wrapping_add(1), u16::MIN);
    assert_eq!(u32::MAX.wrapping_add(1), u32::MIN);
    assert_eq!(u64::MAX.wrapping_add(1), u64::MIN);
    assert_eq!(usize::MAX.wrapping_add(1), usize::MIN);

    assert_eq!(u8::MIN.wrapping_sub(1), u8::MAX);
    assert_eq!(u16::MIN.wrapping_sub(1), u16::MAX);
    assert_eq!(u32::MIN.wrapping_sub(1), u32::MAX);
    assert_eq!(u64::MIN.wrapping_sub(1), u64::MAX);
    assert_eq!(usize::MIN.wrapping_sub(1), usize::MAX);

    assert_eq!((0xfe_u8 as i8).wrapping_mul(16), (0xe0_u8 as i8));
    assert_eq!((0xfedc_u16 as i16).wrapping_mul(16), (0xedc0_u16 as i16));
    assert_eq!((0xfedc_ba98_u32 as i32).wrapping_mul(16), (0xedcb_a980_u32 as i32));
    assert_eq!(
        (0xfedc_ba98_7654_3217_u64 as i64).wrapping_mul(16),
        (0xedcb_a987_6543_2170_u64 as i64)
    );

    match () {
        #[cfg(target_pointer_width = "32")]
        () => {
            assert_eq!((0xfedc_ba98_u32 as isize).wrapping_mul(16), (0xedcb_a980_u32 as isize));
        }
        #[cfg(target_pointer_width = "64")]
        () => {
            assert_eq!(
                (0xfedc_ba98_7654_3217_u64 as isize).wrapping_mul(16),
                (0xedcb_a987_6543_2170_u64 as isize)
            );
        }
    }

    assert_eq!((0xfe as u8).wrapping_mul(16), (0xe0 as u8));
    assert_eq!((0xfedc as u16).wrapping_mul(16), (0xedc0 as u16));
    assert_eq!((0xfedc_ba98 as u32).wrapping_mul(16), (0xedcb_a980 as u32));
    assert_eq!((0xfedc_ba98_7654_3217 as u64).wrapping_mul(16), (0xedcb_a987_6543_2170 as u64));

    match () {
        #[cfg(target_pointer_width = "32")]
        () => {
            assert_eq!((0xfedc_ba98 as usize).wrapping_mul(16), (0xedcb_a980 as usize));
        }
        #[cfg(target_pointer_width = "64")]
        () => {
            assert_eq!(
                (0xfedc_ba98_7654_3217 as usize).wrapping_mul(16),
                (0xedcb_a987_6543_2170 as usize)
            );
        }
    }

    macro_rules! check_mul_no_wrap {
        ($e:expr, $f:expr) => {
            assert_eq!(($e).wrapping_mul($f), ($e) * $f);
        };
    }
    macro_rules! check_mul_wraps {
        ($e:expr, $f:expr) => {
            assert_eq!(($e).wrapping_mul($f), $e);
        };
    }

    check_mul_no_wrap!(0xfe_u8 as i8, -1);
    check_mul_no_wrap!(0xfedc_u16 as i16, -1);
    check_mul_no_wrap!(0xfedc_ba98_u32 as i32, -1);
    check_mul_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, -1);
    check_mul_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -1);

    check_mul_no_wrap!(0xfe_u8 as i8, -2);
    check_mul_no_wrap!(0xfedc_u16 as i16, -2);
    check_mul_no_wrap!(0xfedc_ba98_u32 as i32, -2);
    check_mul_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, -2);
    check_mul_no_wrap!(0xfedc_ba98_fedc_ba98_u64 as u64 as isize, -2);

    check_mul_no_wrap!(0xfe_u8 as i8, 2);
    check_mul_no_wrap!(0xfedc_u16 as i16, 2);
    check_mul_no_wrap!(0xfedc_ba98_u32 as i32, 2);
    check_mul_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, 2);
    check_mul_no_wrap!(0xfedc_ba98_fedc_ba98_u64 as u64 as isize, 2);

    check_mul_wraps!(0x80_u8 as i8, -1);
    check_mul_wraps!(0x8000_u16 as i16, -1);
    check_mul_wraps!(0x8000_0000_u32 as i32, -1);
    check_mul_wraps!(0x8000_0000_0000_0000_u64 as i64, -1);
    match () {
        #[cfg(target_pointer_width = "32")]
        () => {
            check_mul_wraps!(0x8000_0000_u32 as isize, -1);
        }
        #[cfg(target_pointer_width = "64")]
        () => {
            check_mul_wraps!(0x8000_0000_0000_0000_u64 as isize, -1);
        }
    }

    macro_rules! check_div_no_wrap {
        ($e:expr, $f:expr) => {
            assert_eq!(($e).wrapping_div($f), ($e) / $f);
        };
    }
    macro_rules! check_div_wraps {
        ($e:expr, $f:expr) => {
            assert_eq!(($e).wrapping_div($f), $e);
        };
    }

    check_div_no_wrap!(0xfe_u8 as i8, -1);
    check_div_no_wrap!(0xfedc_u16 as i16, -1);
    check_div_no_wrap!(0xfedc_ba98_u32 as i32, -1);
    check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, -1);
    check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -1);

    check_div_no_wrap!(0xfe_u8 as i8, -2);
    check_div_no_wrap!(0xfedc_u16 as i16, -2);
    check_div_no_wrap!(0xfedc_ba98_u32 as i32, -2);
    check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, -2);
    check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -2);

    check_div_no_wrap!(0xfe_u8 as i8, 2);
    check_div_no_wrap!(0xfedc_u16 as i16, 2);
    check_div_no_wrap!(0xfedc_ba98_u32 as i32, 2);
    check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, 2);
    check_div_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, 2);

    check_div_wraps!(-128 as i8, -1);
    check_div_wraps!(0x8000_u16 as i16, -1);
    check_div_wraps!(0x8000_0000_u32 as i32, -1);
    check_div_wraps!(0x8000_0000_0000_0000_u64 as i64, -1);
    match () {
        #[cfg(target_pointer_width = "32")]
        () => {
            check_div_wraps!(0x8000_0000_u32 as isize, -1);
        }
        #[cfg(target_pointer_width = "64")]
        () => {
            check_div_wraps!(0x8000_0000_0000_0000_u64 as isize, -1);
        }
    }

    macro_rules! check_rem_no_wrap {
        ($e:expr, $f:expr) => {
            assert_eq!(($e).wrapping_rem($f), ($e) % $f);
        };
    }
    macro_rules! check_rem_wraps {
        ($e:expr, $f:expr) => {
            assert_eq!(($e).wrapping_rem($f), 0);
        };
    }

    check_rem_no_wrap!(0xfe_u8 as i8, -1);
    check_rem_no_wrap!(0xfedc_u16 as i16, -1);
    check_rem_no_wrap!(0xfedc_ba98_u32 as i32, -1);
    check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, -1);
    check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -1);

    check_rem_no_wrap!(0xfe_u8 as i8, -2);
    check_rem_no_wrap!(0xfedc_u16 as i16, -2);
    check_rem_no_wrap!(0xfedc_ba98_u32 as i32, -2);
    check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, -2);
    check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, -2);

    check_rem_no_wrap!(0xfe_u8 as i8, 2);
    check_rem_no_wrap!(0xfedc_u16 as i16, 2);
    check_rem_no_wrap!(0xfedc_ba98_u32 as i32, 2);
    check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64, 2);
    check_rem_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize, 2);

    check_rem_wraps!(0x80_u8 as i8, -1);
    check_rem_wraps!(0x8000_u16 as i16, -1);
    check_rem_wraps!(0x8000_0000_u32 as i32, -1);
    check_rem_wraps!(0x8000_0000_0000_0000_u64 as i64, -1);
    match () {
        #[cfg(target_pointer_width = "32")]
        () => {
            check_rem_wraps!(0x8000_0000_u32 as isize, -1);
        }
        #[cfg(target_pointer_width = "64")]
        () => {
            check_rem_wraps!(0x8000_0000_0000_0000_u64 as isize, -1);
        }
    }

    macro_rules! check_neg_no_wrap {
        ($e:expr) => {
            assert_eq!(($e).wrapping_neg(), -($e));
        };
    }
    macro_rules! check_neg_wraps {
        ($e:expr) => {
            assert_eq!(($e).wrapping_neg(), ($e));
        };
    }

    check_neg_no_wrap!(0xfe_u8 as i8);
    check_neg_no_wrap!(0xfedc_u16 as i16);
    check_neg_no_wrap!(0xfedc_ba98_u32 as i32);
    check_neg_no_wrap!(0xfedc_ba98_7654_3217_u64 as i64);
    check_neg_no_wrap!(0xfedc_ba98_7654_3217_u64 as u64 as isize);

    check_neg_wraps!(0x80_u8 as i8);
    check_neg_wraps!(0x8000_u16 as i16);
    check_neg_wraps!(0x8000_0000_u32 as i32);
    check_neg_wraps!(0x8000_0000_0000_0000_u64 as i64);
    match () {
        #[cfg(target_pointer_width = "32")]
        () => {
            check_neg_wraps!(0x8000_0000_u32 as isize);
        }
        #[cfg(target_pointer_width = "64")]
        () => {
            check_neg_wraps!(0x8000_0000_0000_0000_u64 as isize);
        }
    }
}

#[test]
fn wrapping_const() {
    // Specifically the wrapping behavior of division and remainder is subtle,
    // see https://github.com/rust-lang/rust/pull/94512.
    const _: () = {
        assert!(i32::MIN.wrapping_div(-1) == i32::MIN);
        assert!(i32::MIN.wrapping_rem(-1) == 0);
    };
}

#[test]
fn test_wrapping_fmt() {
    use core::fmt;

    for i in 0..100 {
        let a: Wrapping<i32> = Wrapping(i);

        let b = d(a);
        let c = d(i);

        assert_eq!(b, c);
    }

    fn d<T>(e: T) -> [String; 6]
    where
        T: fmt::Debug + fmt::Display + fmt::Binary + fmt::Octal + fmt::LowerHex + fmt::UpperHex,
    {
        [
            format!("{}", e),
            format!("{:?}", e),
            format!("{:b}", e),
            format!("{:o}", e),
            format!("{:x}", e),
            format!("{:X}", e),
        ]
    }
}

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

#[test]
fn wrapping_sh_impl_unsigned() {
    let mut a = Wrapping(10_i32);

    let _b = a.shl(1);
    let _c = a.shl_assign(1);
    let _d = a.shr(1);
    let _e = a.shr_assign(1);

    // use core::hint;

    // let f_shl: fn(Wrapping<i32>, usize) -> Wrapping<i32> = Shl::shl;
    // let f_shl = hint::black_box(f_shl);
    // let b = f_shl(a, 1);
    // assert_eq!(b, Wrapping(20));

    // let f_shl_assign: fn(&mut Wrapping<i32>, usize) = ShlAssign::shl_assign;
    // let f_shl_assign = hint::black_box(f_shl_assign);
    // f_shl_assign(&mut a, 2);
    // assert_eq!(a, Wrapping(40));

    // let f_shr: fn(Wrapping<i32>, usize) -> Wrapping<i32> = Shr::shr;
    // let f_shr = hint::black_box(f_shr);
    // let c = f_shr(a, 3);
    // assert_eq!(c, Wrapping(5));

    // let f_shr_assign: fn(&mut Wrapping<i32>, usize) = ShrAssign::shr_assign;
    // let f_shr_assign = hint::black_box(f_shr_assign);
    // f_shr_assign(&mut a, 4);
    // assert_eq!(a, Wrapping(2));
}

#[test]
fn test_ops() {
    let a: Wrapping<i32> = Wrapping(10);
    let b: Wrapping<i32> = Wrapping(1);
    let c: i32 = 2;

    assert_eq!(
        test_op(a, b, c, Wrapping::add, Wrapping::add_assign, Wrapping::add_assign),
        [Wrapping(11), Wrapping(11), Wrapping(13)]
    );
    assert_eq!(
        test_op(a, b, c, Wrapping::sub, Wrapping::sub_assign, Wrapping::sub_assign),
        [Wrapping(9), Wrapping(9), Wrapping(7)]
    );
    assert_eq!(
        test_op(a, b, c, Wrapping::mul, Wrapping::mul_assign, Wrapping::mul_assign),
        [Wrapping(10), Wrapping(10), Wrapping(20)]
    );
    assert_eq!(
        test_op(a, b, c, Wrapping::div, Wrapping::div_assign, Wrapping::div_assign),
        [Wrapping(10), Wrapping(10), Wrapping(5)]
    );
    assert_eq!(
        test_op(a, b, c, Wrapping::rem, Wrapping::rem_assign, Wrapping::rem_assign),
        [Wrapping(0), Wrapping(0), Wrapping(0)]
    );
    assert_eq!(
        test_op(a, b, c, Wrapping::bitxor, Wrapping::bitxor_assign, Wrapping::bitxor_assign),
        [Wrapping(11), Wrapping(11), Wrapping(9)]
    );
    assert_eq!(
        test_op(a, b, c, Wrapping::bitor, Wrapping::bitor_assign, Wrapping::bitor_assign),
        [Wrapping(11), Wrapping(11), Wrapping(11)]
    );
    assert_eq!(
        test_op(a, b, c, Wrapping::bitand, Wrapping::bitand_assign, Wrapping::bitand_assign),
        [Wrapping(0), Wrapping(0), Wrapping(0)]
    );

    assert_eq!(Wrapping::neg(a), Wrapping(-10));
    assert_eq!(Wrapping::not(a), Wrapping(-11));
}

fn test_op<F1, F2, F3>(
    mut a: Wrapping<i32>,
    b: Wrapping<i32>,
    c: i32,
    op: F1,
    op_assign: F2,
    op_assign_other: F3,
) -> [Wrapping<i32>; 3]
where
    F1: Fn(Wrapping<i32>, Wrapping<i32>) -> Wrapping<i32>,
    F2: Fn(&mut Wrapping<i32>, Wrapping<i32>),
    F3: Fn(&mut Wrapping<i32>, i32),
{
    let x = op(a, b);
    op_assign(&mut a, b);
    let y = a;
    op_assign_other(&mut a, c);
    let z = a;
    [x, y, z]
}

#[test]
fn test_wrapping_int_impl() {
    let a: Wrapping<u32> = Wrapping(5);

    assert_eq!(a.count_ones(), 2);
    assert_eq!(a.count_zeros(), 30);
    assert_eq!(a.trailing_zeros(), 0);
    assert_eq!(a.rotate_left(5), Wrapping(160));
    assert_eq!(a.rotate_right(5), Wrapping(671088640));
    assert_eq!(a.swap_bytes(), Wrapping(83886080));
    assert_eq!(a.reverse_bits(), Wrapping(2684354560));
    assert_eq!(Wrapping::<u32>::from_be(a), Wrapping(83886080));
    assert_eq!(Wrapping::<u32>::from_le(a), Wrapping(5));
    assert_eq!(a.to_be(), Wrapping(83886080));
    assert_eq!(a.to_le(), Wrapping(5));
    assert_eq!(a.pow(5), Wrapping(3125));
}

#[test]
fn test_wrapping_int_impl_signed() {
    let a: Wrapping<i32> = Wrapping(5);
    assert_eq!(a.leading_zeros(), 29);
    assert_eq!(a.abs(), Wrapping(5));
    assert_eq!(a.signum(), Wrapping(1));
    assert_eq!(a.is_positive(), true);
    assert_eq!(a.is_negative(), false);

    let a: Wrapping<i32> = Wrapping(-5);
    assert_eq!(a.leading_zeros(), 0);
    assert_eq!(a.abs(), Wrapping(5));
    assert_eq!(a.signum(), Wrapping(-1));
    assert_eq!(a.is_positive(), false);
    assert_eq!(a.is_negative(), true);
}

#[test]
fn test_wrapping_int_impl_unsigned() {

    let a: Wrapping<u32> = Wrapping(5);
    assert_eq!(a.leading_zeros(), 29);
    assert_eq!(a.is_power_of_two(), false);
    assert_eq!(a.next_power_of_two(), Wrapping(8));
}