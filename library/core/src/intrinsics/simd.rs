//! SIMD compiler intrinsics.
//!
//! In this module, a "vector" is any `repr(simd)` type.

extern "rust-intrinsic" {
    /// Insert an element into a vector, returning the updated vector.
    ///
    /// `T` must be a vector with element type `U`.
    ///
    /// # Safety
    ///
    /// `idx` must be in-bounds of the vector.
    #[rustc_nounwind]
    pub fn simd_insert<T, U>(x: T, idx: u32, val: U) -> T;

    /// Extract an element from a vector.
    ///
    /// `T` must be a vector with element type `U`.
    ///
    /// # Safety
    ///
    /// `idx` must be in-bounds of the vector.
    #[rustc_nounwind]
    pub fn simd_extract<T, U>(x: T, idx: u32) -> U;

    /// Add two simd vectors elementwise.
    ///
    /// `T` must be a vector of integer or floating point primitive types.
    #[rustc_nounwind]
    pub fn simd_add<T>(x: T, y: T) -> T;

    /// Subtract `rhs` from `lhs` elementwise.
    ///
    /// `T` must be a vector of integer or floating point primitive types.
    #[rustc_nounwind]
    pub fn simd_sub<T>(lhs: T, rhs: T) -> T;

    /// Multiply two simd vectors elementwise.
    ///
    /// `T` must be a vector of integer or floating point primitive types.
    #[rustc_nounwind]
    pub fn simd_mul<T>(x: T, y: T) -> T;

    /// Divide `lhs` by `rhs` elementwise.
    ///
    /// `T` must be a vector of integer or floating point primitive types.
    ///
    /// # Safety
    /// For integers, `rhs` must not contain any zero elements.
    /// Additionally for signed integers, `<int>::MIN / -1` is undefined behavior.
    #[rustc_nounwind]
    pub fn simd_div<T>(lhs: T, rhs: T) -> T;

    /// Remainder of two vectors elementwise
    ///
    /// `T` must be a vector of integer or floating point primitive types.
    ///
    /// # Safety
    /// For integers, `rhs` must not contain any zero elements.
    /// Additionally for signed integers, `<int>::MIN / -1` is undefined behavior.
    #[rustc_nounwind]
    pub fn simd_rem<T>(lhs: T, rhs: T) -> T;

    /// Elementwise vector left shift, with UB on overflow.
    ///
    /// Shift `lhs` left by `rhs`, shifting in sign bits for signed types.
    ///
    /// `T` must be a vector of integer primitive types.
    ///
    /// # Safety
    ///
    /// Each element of `rhs` must be less than `<int>::BITS`.
    #[rustc_nounwind]
    pub fn simd_shl<T>(lhs: T, rhs: T) -> T;

    /// Elementwise vector right shift, with UB on overflow.
    ///
    /// `T` must be a vector of integer primitive types.
    ///
    /// Shift `lhs` right by `rhs`, shifting in sign bits for signed types.
    ///
    /// # Safety
    ///
    /// Each element of `rhs` must be less than `<int>::BITS`.
    #[rustc_nounwind]
    pub fn simd_shr<T>(lhs: T, rhs: T) -> T;

    /// Elementwise vector "and".
    ///
    /// `T` must be a vector of integer primitive types.
    #[rustc_nounwind]
    pub fn simd_and<T>(x: T, y: T) -> T;

    /// Elementwise vector "or".
    ///
    /// `T` must be a vector of integer primitive types.
    #[rustc_nounwind]
    pub fn simd_or<T>(x: T, y: T) -> T;

    /// Elementwise vector "exclusive or".
    ///
    /// `T` must be a vector of integer primitive types.
    #[rustc_nounwind]
    pub fn simd_xor<T>(x: T, y: T) -> T;

    /// Numerically cast a vector, elementwise.
    ///
    /// `T` and `U` must be vectors of integer or floating point primitive types, and must have the
    /// same length.
    ///
    /// When casting floats to integers, the result is truncated. Out-of-bounds result lead to UB.
    /// When casting integers to floats, the result is rounded.
    /// Otherwise, truncates or extends the value, maintaining the sign for signed integers.
    ///
    /// # Safety
    /// Casting from integer types is always safe.
    /// Casting between two float types is also always safe.
    ///
    /// Casting floats to integers truncates, following the same rules as `to_int_unchecked`.
    /// Specifically, each element must:
    /// * Not be `NaN`
    /// * Not be infinite
    /// * Be representable in the return type, after truncating off its fractional part
    #[rustc_nounwind]
    pub fn simd_cast<T, U>(x: T) -> U;

    /// Numerically cast a vector, elementwise.
    ///
    /// `T` and `U` be a vectors of integer or floating point primitive types, and must have the
    /// same length.
    ///
    /// Like `simd_cast`, but saturates float-to-integer conversions (NaN becomes 0).
    /// This matches regular `as` and is always safe.
    ///
    /// When casting floats to integers, the result is truncated.
    /// When casting integers to floats, the result is rounded.
    /// Otherwise, truncates or extends the value, maintaining the sign for signed integers.
    #[rustc_nounwind]
    pub fn simd_as<T, U>(x: T) -> U;

    /// Elementwise negation of a vector.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// Rust panics for `-<int>::Min` due to overflow, but it is not UB with this intrinsic.
    #[rustc_nounwind]
    pub fn simd_neg<T>(x: T) -> T;

    /// Elementwise absolute value of a vector.
    ///
    /// `T` must be a vector of floating-point primitive types.
    #[rustc_nounwind]
    pub fn simd_fabs<T>(x: T) -> T;

    /// Elementwise minimum of two vectors.
    ///
    /// `T` must be a vector of floating-point primitive types.
    ///
    /// Follows IEEE-754 `minNum` semantics.
    #[rustc_nounwind]
    pub fn simd_fmin<T>(x: T, y: T) -> T;

    /// Elementwise maximum of two vectors.
    ///
    /// `T` must be a vector of floating-point primitive types.
    ///
    /// Follows IEEE-754 `maxNum` semantics.
    #[rustc_nounwind]
    pub fn simd_fmax<T>(x: T, y: T) -> T;

    /// Tests elementwise equality of two vectors.
    ///
    /// `T` must be a vector of floating-point primitive types.
    ///
    /// `U` must be a vector of integers with the same number of elements and element size as `T`.
    ///
    /// Returns `0` for false and `!0` for true.
    #[rustc_nounwind]
    pub fn simd_eq<T, U>(x: T, y: T) -> U;

    /// Tests elementwise inequality equality of two vectors.
    ///
    /// `T` must be a vector of floating-point primitive types.
    ///
    /// `U` must be a vector of integers with the same number of elements and element size as `T`.
    ///
    /// Returns `0` for false and `!0` for true.
    #[rustc_nounwind]
    pub fn simd_ne<T, U>(x: T, y: T) -> U;

    /// Tests if `x` is less than `y`, elementwise.
    ///
    /// `T` must be a vector of floating-point primitive types.
    ///
    /// `U` must be a vector of integers with the same number of elements and element size as `T`.
    ///
    /// Returns `0` for false and `!0` for true.
    #[rustc_nounwind]
    pub fn simd_lt<T, U>(x: T, y: T) -> U;

    /// Tests if `x` is less than or equal to `y`, elementwise.
    ///
    /// `T` must be a vector of floating-point primitive types.
    ///
    /// `U` must be a vector of integers with the same number of elements and element size as `T`.
    ///
    /// Returns `0` for false and `!0` for true.
    #[rustc_nounwind]
    pub fn simd_le<T, U>(x: T, y: T) -> U;

    /// Tests if `x` is greater than `y`, elementwise.
    ///
    /// `T` must be a vector of floating-point primitive types.
    ///
    /// `U` must be a vector of integers with the same number of elements and element size as `T`.
    ///
    /// Returns `0` for false and `!0` for true.
    #[rustc_nounwind]
    pub fn simd_gt<T, U>(x: T, y: T) -> U;

    /// Tests if `x` is greater than or equal to `y`, elementwise.
    ///
    /// `T` must be a vector of floating-point primitive types.
    ///
    /// `U` must be a vector of integers with the same number of elements and element size as `T`.
    ///
    /// Returns `0` for false and `!0` for true.
    #[rustc_nounwind]
    pub fn simd_ge<T, U>(x: T, y: T) -> U;

    /// Shuffle two vectors by const indices.
    ///
    /// `T` must be a vector.
    ///
    /// `U` must be a **const** array of `i32`s. This means it must either refer to a named
    /// const or be given as an inline const expression (`const { ... }`).
    ///
    /// `V` must be a vector with the same element type as `T` and the same length as `U`.
    ///
    /// Returns a new vector such that element `i` is selected from `xy[idx[i]]`, where `xy`
    /// is the concatenation of `x` and `y`. It is a compile-time error if `idx[i]` is out-of-bounds
    /// of `xy`.
    #[rustc_nounwind]
    pub fn simd_shuffle<T, U, V>(x: T, y: T, idx: U) -> V;

    /// Shuffle two vectors by const indices.
    ///
    /// `T` must be a vector.
    ///
    /// `U` must be a vector with the same element type as `T` and the same length as `IDX`.
    ///
    /// Returns a new vector such that element `i` is selected from `xy[IDX[i]]`, where `xy`
    /// is the concatenation of `x` and `y`. It is a compile-time error if `IDX[i]` is out-of-bounds
    /// of `xy`.
    #[rustc_nounwind]
    pub fn simd_shuffle_generic<T, U, const IDX: &'static [u32]>(x: T, y: T) -> U;

    /// Read a vector of pointers.
    ///
    /// `T` must be a vector.
    ///
    /// `U` must be a vector of pointers to the element type of `T`, with the same length as `T`.
    ///
    /// `V` must be a vector of integers with the same length as `T` (but any element size).
    ///
    /// `idx` must be a constant: either naming a constant item, or an inline
    /// `const {}` expression.
    ///
    /// For each pointer in `ptr`, if the corresponding value in `mask` is `!0`, read the pointer.
    /// Otherwise if the corresponding value in `mask` is `0`, return the corresponding value from
    /// `val`.
    ///
    /// # Safety
    /// Unmasked values in `T` must be readable as if by `<ptr>::read` (e.g. aligned to the element
    /// type).
    ///
    /// `mask` must only contain `0` or `!0` values.
    #[rustc_nounwind]
    pub fn simd_gather<T, U, V>(val: T, ptr: U, mask: V) -> T;

    /// Write to a vector of pointers.
    ///
    /// `T` must be a vector.
    ///
    /// `U` must be a vector of pointers to the element type of `T`, with the same length as `T`.
    ///
    /// `V` must be a vector of integers with the same length as `T` (but any element size).
    ///
    /// For each pointer in `ptr`, if the corresponding value in `mask` is `!0`, write the
    /// corresponding value in `val` to the pointer.
    /// Otherwise if the corresponding value in `mask` is `0`, do nothing.
    ///
    /// The stores happen in left-to-right order.
    /// (This is relevant in case two of the stores overlap.)
    ///
    /// # Safety
    /// Unmasked values in `T` must be writeable as if by `<ptr>::write` (e.g. aligned to the element
    /// type).
    ///
    /// `mask` must only contain `0` or `!0` values.
    #[rustc_nounwind]
    pub fn simd_scatter<T, U, V>(val: T, ptr: U, mask: V);

    /// Read a vector of pointers.
    ///
    /// `T` must be a vector.
    ///
    /// `U` must be a pointer to the element type of `T`
    ///
    /// `V` must be a vector of integers with the same length as `T` (but any element size).
    ///
    /// For each element, if the corresponding value in `mask` is `!0`, read the corresponding
    /// pointer offset from `ptr`.
    /// The first element is loaded from `ptr`, the second from `ptr.wrapping_offset(1)` and so on.
    /// Otherwise if the corresponding value in `mask` is `0`, return the corresponding value from
    /// `val`.
    ///
    /// # Safety
    /// Unmasked values in `T` must be readable as if by `<ptr>::read` (e.g. aligned to the element
    /// type).
    ///
    /// `mask` must only contain `0` or `!0` values.
    #[rustc_nounwind]
    pub fn simd_masked_load<V, U, T>(mask: V, ptr: U, val: T) -> T;

    /// Write to a vector of pointers.
    ///
    /// `T` must be a vector.
    ///
    /// `U` must be a pointer to the element type of `T`
    ///
    /// `V` must be a vector of integers with the same length as `T` (but any element size).
    ///
    /// For each element, if the corresponding value in `mask` is `!0`, write the corresponding
    /// value in `val` to the pointer offset from `ptr`.
    /// The first element is written to `ptr`, the second to `ptr.wrapping_offset(1)` and so on.
    /// Otherwise if the corresponding value in `mask` is `0`, do nothing.
    ///
    /// # Safety
    /// Unmasked values in `T` must be writeable as if by `<ptr>::write` (e.g. aligned to the element
    /// type).
    ///
    /// `mask` must only contain `0` or `!0` values.
    #[rustc_nounwind]
    pub fn simd_masked_store<V, U, T>(mask: V, ptr: U, val: T);

    /// Add two simd vectors elementwise, with saturation.
    ///
    /// `T` must be a vector of integer primitive types.
    #[rustc_nounwind]
    pub fn simd_saturating_add<T>(x: T, y: T) -> T;

    /// Subtract two simd vectors elementwise, with saturation.
    ///
    /// `T` must be a vector of integer primitive types.
    ///
    /// Subtract `rhs` from `lhs`.
    #[rustc_nounwind]
    pub fn simd_saturating_sub<T>(lhs: T, rhs: T) -> T;

    /// Add elements within a vector from left to right.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    ///
    /// Starting with the value `y`, add the elements of `x` and accumulate.
    #[rustc_nounwind]
    pub fn simd_reduce_add_ordered<T, U>(x: T, y: U) -> U;

    /// Add elements within a vector in arbitrary order. May also be re-associated with
    /// unordered additions on the inputs/outputs.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    #[rustc_nounwind]
    pub fn simd_reduce_add_unordered<T, U>(x: T) -> U;

    /// Multiply elements within a vector from left to right.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    ///
    /// Starting with the value `y`, multiply the elements of `x` and accumulate.
    #[rustc_nounwind]
    pub fn simd_reduce_mul_ordered<T, U>(x: T, y: U) -> U;

    /// Multiply elements within a vector in arbitrary order. May also be re-associated with
    /// unordered additions on the inputs/outputs.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    #[rustc_nounwind]
    pub fn simd_reduce_mul_unordered<T, U>(x: T) -> U;

    /// Check if all mask values are true.
    ///
    /// `T` must be a vector of integer primitive types.
    ///
    /// # Safety
    /// `x` must contain only `0` or `!0`.
    #[rustc_nounwind]
    pub fn simd_reduce_all<T>(x: T) -> bool;

    /// Check if any mask value is true.
    ///
    /// `T` must be a vector of integer primitive types.
    ///
    /// # Safety
    /// `x` must contain only `0` or `!0`.
    #[rustc_nounwind]
    pub fn simd_reduce_any<T>(x: T) -> bool;

    /// Return the maximum element of a vector.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    ///
    /// For floating-point values, uses IEEE-754 `maxNum`.
    #[rustc_nounwind]
    pub fn simd_reduce_max<T, U>(x: T) -> U;

    /// Return the minimum element of a vector.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    ///
    /// For floating-point values, uses IEEE-754 `minNum`.
    #[rustc_nounwind]
    pub fn simd_reduce_min<T, U>(x: T) -> U;

    /// Logical "and" all elements together.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    #[rustc_nounwind]
    pub fn simd_reduce_and<T, U>(x: T) -> U;

    /// Logical "or" all elements together.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    #[rustc_nounwind]
    pub fn simd_reduce_or<T, U>(x: T) -> U;

    /// Logical "exclusive or" all elements together.
    ///
    /// `T` must be a vector of integer or floating-point primitive types.
    ///
    /// `U` must be the element type of `T`.
    #[rustc_nounwind]
    pub fn simd_reduce_xor<T, U>(x: T) -> U;

    /// Truncate an integer vector to a bitmask.
    ///
    /// `T` must be an integer vector.
    ///
    /// `U` must be either the smallest unsigned integer with at least as many bits as the length
    /// of `T`, or the smallest array of `u8` with as many bits as the length of `T`.
    ///
    /// Each element is truncated to a single bit and packed into the result.
    ///
    /// No matter whether the output is an array or an unsigned integer, it is treated as a single
    /// contiguous list of bits. The bitmask is always packed on the least-significant side of the
    /// output, and padded with 0s in the most-significant bits. The order of the bits depends on
    /// endianness:
    ///
    /// * On little endian, the least significant bit corresponds to the first vector element.
    /// * On big endian, the least significant bit corresponds to the last vector element.
    ///
    /// For example, `[!0, 0, !0, !0]` packs to `0b1101` on little endian and `0b1011` on big
    /// endian.
    ///
    /// To consider a larger example, `[!0, 0, 0, 0, 0, 0, 0, 0, !0, !0, 0, 0, 0, 0, !0, 0]` packs
    /// to `[0b00000001, 0b01000011]` or `0b0100001100000001` on little endian, and `[0b10000000,
    /// 0b11000010]` or `0b1000000011000010` on big endian.
    ///
    /// # Safety
    /// `x` must contain only `0` and `!0`.
    #[rustc_nounwind]
    pub fn simd_bitmask<T, U>(x: T) -> U;

    /// Select elements from a mask.
    ///
    /// `M` must be an integer vector.
    ///
    /// `T` must be a vector with the same number of elements as `M`.
    ///
    /// For each element, if the corresponding value in `mask` is `!0`, select the element from
    /// `if_true`.  If the corresponding value in `mask` is `0`, select the element from
    /// `if_false`.
    ///
    /// # Safety
    /// `mask` must only contain `0` and `!0`.
    #[rustc_nounwind]
    pub fn simd_select<M, T>(mask: M, if_true: T, if_false: T) -> T;

    /// Select elements from a bitmask.
    ///
    /// `M` must be an unsigned integer or array of `u8`, matching `simd_bitmask`.
    ///
    /// `T` must be a vector.
    ///
    /// For each element, if the bit in `mask` is `1`, select the element from
    /// `if_true`.  If the corresponding bit in `mask` is `0`, select the element from
    /// `if_false`.
    ///
    /// The bitmask bit order matches `simd_bitmask`.
    ///
    /// # Safety
    /// Padding bits must be all zero.
    #[rustc_nounwind]
    pub fn simd_select_bitmask<M, T>(m: M, yes: T, no: T) -> T;

    /// Elementwise calculates the offset from a pointer vector, potentially wrapping.
    ///
    /// `T` must be a vector of pointers.
    ///
    /// `U` must be a vector of `isize` or `usize` with the same number of elements as `T`.
    ///
    /// Operates as if by `<ptr>::wrapping_offset`.
    #[rustc_nounwind]
    pub fn simd_arith_offset<T, U>(ptr: T, offset: U) -> T;

    /// Cast a vector of pointers.
    ///
    /// `T` and `U` must be vectors of pointers with the same number of elements.
    #[rustc_nounwind]
    pub fn simd_cast_ptr<T, U>(ptr: T) -> U;

    /// Expose a vector of pointers as a vector of addresses.
    ///
    /// `T` must be a vector of pointers.
    ///
    /// `U` must be a vector of `usize` with the same length as `T`.
    #[rustc_nounwind]
    pub fn simd_expose_provenance<T, U>(ptr: T) -> U;

    /// Create a vector of pointers from a vector of addresses.
    ///
    /// `T` must be a vector of `usize`.
    ///
    /// `U` must be a vector of pointers, with the same length as `T`.
    #[rustc_nounwind]
    pub fn simd_with_exposed_provenance<T, U>(addr: T) -> U;

    /// Swap bytes of each element.
    ///
    /// `T` must be a vector of integers.
    #[rustc_nounwind]
    pub fn simd_bswap<T>(x: T) -> T;

    /// Reverse bits of each element.
    ///
    /// `T` must be a vector of integers.
    #[rustc_nounwind]
    pub fn simd_bitreverse<T>(x: T) -> T;

    /// Count the leading zeros of each element.
    ///
    /// `T` must be a vector of integers.
    #[rustc_nounwind]
    pub fn simd_ctlz<T>(x: T) -> T;

    /// Count the number of ones in each element.
    ///
    /// `T` must be a vector of integers.
    #[rustc_nounwind]
    #[cfg(not(bootstrap))]
    pub fn simd_ctpop<T>(x: T) -> T;

    /// Count the trailing zeros of each element.
    ///
    /// `T` must be a vector of integers.
    #[rustc_nounwind]
    pub fn simd_cttz<T>(x: T) -> T;

    /// Round up each element to the next highest integer-valued float.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_ceil<T>(x: T) -> T;

    /// Round down each element to the next lowest integer-valued float.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_floor<T>(x: T) -> T;

    /// Round each element to the closest integer-valued float.
    /// Ties are resolved by rounding away from 0.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_round<T>(x: T) -> T;

    /// Return the integer part of each element as an integer-valued float.
    /// In other words, non-integer values are truncated towards zero.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_trunc<T>(x: T) -> T;

    /// Takes the square root of each element.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_fsqrt<T>(x: T) -> T;

    /// Computes `(x*y) + z` for each element, but without any intermediate rounding.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_fma<T>(x: T, y: T, z: T) -> T;

    // Computes the sine of each element.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_fsin<T>(a: T) -> T;

    // Computes the cosine of each element.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_fcos<T>(a: T) -> T;

    // Computes the exponential function of each element.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_fexp<T>(a: T) -> T;

    // Computes 2 raised to the power of each element.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_fexp2<T>(a: T) -> T;

    // Computes the base 10 logarithm of each element.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_flog10<T>(a: T) -> T;

    // Computes the base 2 logarithm of each element.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_flog2<T>(a: T) -> T;

    // Computes the natural logarithm of each element.
    ///
    /// `T` must be a vector of floats.
    #[rustc_nounwind]
    pub fn simd_flog<T>(a: T) -> T;
}
