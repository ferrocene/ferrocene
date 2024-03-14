use rustc_apfloat::ieee::Double;
use rustc_middle::ty::layout::LayoutOf as _;
use rustc_middle::ty::Ty;
use rustc_span::Symbol;
use rustc_target::spec::abi::Abi;

use super::{bin_op_simd_float_all, bin_op_simd_float_first, convert_float_to_int, FloatBinOp};
use crate::*;
use shims::foreign_items::EmulateForeignItemResult;

impl<'mir, 'tcx: 'mir> EvalContextExt<'mir, 'tcx> for crate::MiriInterpCx<'mir, 'tcx> {}
pub(super) trait EvalContextExt<'mir, 'tcx: 'mir>:
    crate::MiriInterpCxExt<'mir, 'tcx>
{
    fn emulate_x86_sse2_intrinsic(
        &mut self,
        link_name: Symbol,
        abi: Abi,
        args: &[OpTy<'tcx, Provenance>],
        dest: &MPlaceTy<'tcx, Provenance>,
    ) -> InterpResult<'tcx, EmulateForeignItemResult> {
        let this = self.eval_context_mut();
        this.expect_target_feature_for_intrinsic(link_name, "sse2")?;
        // Prefix should have already been checked.
        let unprefixed_name = link_name.as_str().strip_prefix("llvm.x86.sse2.").unwrap();

        // These intrinsics operate on 128-bit (f32x4, f64x2, i8x16, i16x8, i32x4, i64x2) SIMD
        // vectors unless stated otherwise.
        // Many intrinsic names are sufixed with "ps" (packed single), "ss" (scalar signle),
        // "pd" (packed double) or "sd" (scalar double), where single means single precision
        // floating point (f32) and double means double precision floating point (f64). "ps"
        // and "pd" means thet the operation is performed on each element of the vector, while
        // "ss" and "sd" means that the operation is performed only on the first element, copying
        // the remaining elements from the input vector (for binary operations, from the left-hand
        // side).
        // Intrinsincs sufixed with "epiX" or "epuX" operate with X-bit signed or unsigned
        // vectors.
        match unprefixed_name {
            // Used to implement the _mm_madd_epi16 function.
            // Multiplies packed signed 16-bit integers in `left` and `right`, producing
            // intermediate signed 32-bit integers. Horizontally add adjacent pairs of
            // intermediate 32-bit integers, and pack the results in `dest`.
            "pmadd.wd" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                assert_eq!(left_len, right_len);
                assert_eq!(dest_len.checked_mul(2).unwrap(), left_len);

                for i in 0..dest_len {
                    let j1 = i.checked_mul(2).unwrap();
                    let left1 = this.read_scalar(&this.project_index(&left, j1)?)?.to_i16()?;
                    let right1 = this.read_scalar(&this.project_index(&right, j1)?)?.to_i16()?;

                    let j2 = j1.checked_add(1).unwrap();
                    let left2 = this.read_scalar(&this.project_index(&left, j2)?)?.to_i16()?;
                    let right2 = this.read_scalar(&this.project_index(&right, j2)?)?.to_i16()?;

                    let dest = this.project_index(&dest, i)?;

                    // Multiplications are i16*i16->i32, which will not overflow.
                    let mul1 = i32::from(left1).checked_mul(right1.into()).unwrap();
                    let mul2 = i32::from(left2).checked_mul(right2.into()).unwrap();
                    // However, this addition can overflow in the most extreme case
                    // (-0x8000)*(-0x8000)+(-0x8000)*(-0x8000) = 0x80000000
                    let res = mul1.wrapping_add(mul2);

                    this.write_scalar(Scalar::from_i32(res), &dest)?;
                }
            }
            // Used to implement the _mm_sad_epu8 function.
            // Computes the absolute differences of packed unsigned 8-bit integers in `a`
            // and `b`, then horizontally sum each consecutive 8 differences to produce
            // two unsigned 16-bit integers, and pack these unsigned 16-bit integers in
            // the low 16 bits of 64-bit elements returned.
            //
            // https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sad_epu8
            "psad.bw" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                // left and right are u8x16, dest is u64x2
                assert_eq!(left_len, right_len);
                assert_eq!(left_len, 16);
                assert_eq!(dest_len, 2);

                for i in 0..dest_len {
                    let dest = this.project_index(&dest, i)?;

                    let mut res: u16 = 0;
                    let n = left_len.checked_div(dest_len).unwrap();
                    for j in 0..n {
                        let op_i = j.checked_add(i.checked_mul(n).unwrap()).unwrap();
                        let left = this.read_scalar(&this.project_index(&left, op_i)?)?.to_u8()?;
                        let right =
                            this.read_scalar(&this.project_index(&right, op_i)?)?.to_u8()?;

                        res = res.checked_add(left.abs_diff(right).into()).unwrap();
                    }

                    this.write_scalar(Scalar::from_u64(res.into()), &dest)?;
                }
            }
            // Used to implement the _mm_{sll,srl,sra}_epi16 functions.
            // Shifts 16-bit packed integers in left by the amount in right.
            // Both operands are vectors of 16-bit integers. However, right is
            // interpreted as a single 64-bit integer (remaining bits are ignored).
            // For logic shifts, when right is larger than 15, zero is produced.
            // For arithmetic shifts, when right is larger than 15, the sign bit
            // is copied to remaining bits.
            "psll.w" | "psrl.w" | "psra.w" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                assert_eq!(dest_len, left_len);
                assert_eq!(dest_len, right_len);

                enum ShiftOp {
                    Sll,
                    Srl,
                    Sra,
                }
                let which = match unprefixed_name {
                    "psll.w" => ShiftOp::Sll,
                    "psrl.w" => ShiftOp::Srl,
                    "psra.w" => ShiftOp::Sra,
                    _ => unreachable!(),
                };

                // Get the 64-bit shift operand and convert it to the type expected
                // by checked_{shl,shr} (u32).
                // It is ok to saturate the value to u32::MAX because any value
                // above 15 will produce the same result.
                let shift = extract_first_u64(this, &right)?.try_into().unwrap_or(u32::MAX);

                for i in 0..dest_len {
                    let left = this.read_scalar(&this.project_index(&left, i)?)?.to_u16()?;
                    let dest = this.project_index(&dest, i)?;

                    let res = match which {
                        ShiftOp::Sll => left.checked_shl(shift).unwrap_or(0),
                        ShiftOp::Srl => left.checked_shr(shift).unwrap_or(0),
                        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                        ShiftOp::Sra => {
                            // Convert u16 to i16 to use arithmetic shift
                            let left = left as i16;
                            // Copy the sign bit to the remaining bits
                            left.checked_shr(shift).unwrap_or(left >> 15) as u16
                        }
                    };

                    this.write_scalar(Scalar::from_u16(res), &dest)?;
                }
            }
            // Used to implement the _mm_{sll,srl,sra}_epi32 functions.
            // 32-bit equivalent to the shift functions above.
            "psll.d" | "psrl.d" | "psra.d" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                assert_eq!(dest_len, left_len);
                assert_eq!(dest_len, right_len);

                enum ShiftOp {
                    Sll,
                    Srl,
                    Sra,
                }
                let which = match unprefixed_name {
                    "psll.d" => ShiftOp::Sll,
                    "psrl.d" => ShiftOp::Srl,
                    "psra.d" => ShiftOp::Sra,
                    _ => unreachable!(),
                };

                // Get the 64-bit shift operand and convert it to the type expected
                // by checked_{shl,shr} (u32).
                // It is ok to saturate the value to u32::MAX because any value
                // above 31 will produce the same result.
                let shift = extract_first_u64(this, &right)?.try_into().unwrap_or(u32::MAX);

                for i in 0..dest_len {
                    let left = this.read_scalar(&this.project_index(&left, i)?)?.to_u32()?;
                    let dest = this.project_index(&dest, i)?;

                    let res = match which {
                        ShiftOp::Sll => left.checked_shl(shift).unwrap_or(0),
                        ShiftOp::Srl => left.checked_shr(shift).unwrap_or(0),
                        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                        ShiftOp::Sra => {
                            // Convert u32 to i32 to use arithmetic shift
                            let left = left as i32;
                            // Copy the sign bit to the remaining bits
                            left.checked_shr(shift).unwrap_or(left >> 31) as u32
                        }
                    };

                    this.write_scalar(Scalar::from_u32(res), &dest)?;
                }
            }
            // Used to implement the _mm_{sll,srl}_epi64 functions.
            // 64-bit equivalent to the shift functions above, except _mm_sra_epi64,
            // which is not available in SSE2.
            "psll.q" | "psrl.q" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                assert_eq!(dest_len, left_len);
                assert_eq!(dest_len, right_len);

                enum ShiftOp {
                    Sll,
                    Srl,
                }
                let which = match unprefixed_name {
                    "psll.q" => ShiftOp::Sll,
                    "psrl.q" => ShiftOp::Srl,
                    _ => unreachable!(),
                };

                // Get the 64-bit shift operand and convert it to the type expected
                // by checked_{shl,shr} (u32).
                // It is ok to saturate the value to u32::MAX because any value
                // above 63 will produce the same result.
                let shift = this
                    .read_scalar(&this.project_index(&right, 0)?)?
                    .to_u64()?
                    .try_into()
                    .unwrap_or(u32::MAX);

                for i in 0..dest_len {
                    let left = this.read_scalar(&this.project_index(&left, i)?)?.to_u64()?;
                    let dest = this.project_index(&dest, i)?;

                    let res = match which {
                        ShiftOp::Sll => left.checked_shl(shift).unwrap_or(0),
                        ShiftOp::Srl => left.checked_shr(shift).unwrap_or(0),
                    };

                    this.write_scalar(Scalar::from_u64(res), &dest)?;
                }
            }
            // Used to implement the _mm_cvtps_epi32, _mm_cvttps_epi32, _mm_cvtpd_epi32
            // and _mm_cvttpd_epi32 functions.
            // Converts packed f32/f64 to packed i32.
            "cvtps2dq" | "cvttps2dq" | "cvtpd2dq" | "cvttpd2dq" => {
                let [op] = this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (op_len, _) = op.layout.ty.simd_size_and_type(*this.tcx);
                let (dest_len, _) = dest.layout.ty.simd_size_and_type(*this.tcx);
                match unprefixed_name {
                    "cvtps2dq" | "cvttps2dq" => {
                        // f32x4 to i32x4 conversion
                        assert_eq!(op_len, 4);
                        assert_eq!(dest_len, op_len);
                    }
                    "cvtpd2dq" | "cvttpd2dq" => {
                        // f64x2 to i32x4 conversion
                        // the last two values are filled with zeros
                        assert_eq!(op_len, 2);
                        assert_eq!(dest_len, 4);
                    }
                    _ => unreachable!(),
                }

                let rnd = match unprefixed_name {
                    // "current SSE rounding mode", assume nearest
                    // https://www.felixcloutier.com/x86/cvtps2dq
                    // https://www.felixcloutier.com/x86/cvtpd2dq
                    "cvtps2dq" | "cvtpd2dq" => rustc_apfloat::Round::NearestTiesToEven,
                    // always truncate
                    // https://www.felixcloutier.com/x86/cvttps2dq
                    // https://www.felixcloutier.com/x86/cvttpd2dq
                    "cvttps2dq" | "cvttpd2dq" => rustc_apfloat::Round::TowardZero,
                    _ => unreachable!(),
                };

                convert_float_to_int(this, op, rnd, dest)?;
            }
            // Used to implement the _mm_packs_epi16 function.
            // Converts two 16-bit integer vectors to a single 8-bit integer
            // vector with signed saturation.
            "packsswb.128" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                // left and right are i16x8, dest is i8x16
                assert_eq!(left_len, 8);
                assert_eq!(right_len, 8);
                assert_eq!(dest_len, 16);

                for i in 0..left_len {
                    let left = this.read_scalar(&this.project_index(&left, i)?)?.to_i16()?;
                    let right = this.read_scalar(&this.project_index(&right, i)?)?.to_i16()?;
                    let left_dest = this.project_index(&dest, i)?;
                    let right_dest = this.project_index(&dest, i.checked_add(left_len).unwrap())?;

                    let left_res =
                        i8::try_from(left).unwrap_or(if left < 0 { i8::MIN } else { i8::MAX });
                    let right_res =
                        i8::try_from(right).unwrap_or(if right < 0 { i8::MIN } else { i8::MAX });

                    this.write_scalar(Scalar::from_i8(left_res), &left_dest)?;
                    this.write_scalar(Scalar::from_i8(right_res), &right_dest)?;
                }
            }
            // Used to implement the _mm_packus_epi16 function.
            // Converts two 16-bit signed integer vectors to a single 8-bit
            // unsigned integer vector with saturation.
            "packuswb.128" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                // left and right are i16x8, dest is u8x16
                assert_eq!(left_len, 8);
                assert_eq!(right_len, 8);
                assert_eq!(dest_len, 16);

                for i in 0..left_len {
                    let left = this.read_scalar(&this.project_index(&left, i)?)?.to_i16()?;
                    let right = this.read_scalar(&this.project_index(&right, i)?)?.to_i16()?;
                    let left_dest = this.project_index(&dest, i)?;
                    let right_dest = this.project_index(&dest, i.checked_add(left_len).unwrap())?;

                    let left_res = u8::try_from(left).unwrap_or(if left < 0 { 0 } else { u8::MAX });
                    let right_res =
                        u8::try_from(right).unwrap_or(if right < 0 { 0 } else { u8::MAX });

                    this.write_scalar(Scalar::from_u8(left_res), &left_dest)?;
                    this.write_scalar(Scalar::from_u8(right_res), &right_dest)?;
                }
            }
            // Used to implement the _mm_packs_epi32 function.
            // Converts two 32-bit integer vectors to a single 16-bit integer
            // vector with signed saturation.
            "packssdw.128" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                // left and right are i32x4, dest is i16x8
                assert_eq!(left_len, 4);
                assert_eq!(right_len, 4);
                assert_eq!(dest_len, 8);

                for i in 0..left_len {
                    let left = this.read_scalar(&this.project_index(&left, i)?)?.to_i32()?;
                    let right = this.read_scalar(&this.project_index(&right, i)?)?.to_i32()?;
                    let left_dest = this.project_index(&dest, i)?;
                    let right_dest = this.project_index(&dest, i.checked_add(left_len).unwrap())?;

                    let left_res =
                        i16::try_from(left).unwrap_or(if left < 0 { i16::MIN } else { i16::MAX });
                    let right_res =
                        i16::try_from(right).unwrap_or(if right < 0 { i16::MIN } else { i16::MAX });

                    this.write_scalar(Scalar::from_i16(left_res), &left_dest)?;
                    this.write_scalar(Scalar::from_i16(right_res), &right_dest)?;
                }
            }
            // Used to implement _mm_min_sd and _mm_max_sd functions.
            // Note that the semantics are a bit different from Rust simd_min
            // and simd_max intrinsics regarding handling of NaN and -0.0: Rust
            // matches the IEEE min/max operations, while x86 has different
            // semantics.
            "min.sd" | "max.sd" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let which = match unprefixed_name {
                    "min.sd" => FloatBinOp::Min,
                    "max.sd" => FloatBinOp::Max,
                    _ => unreachable!(),
                };

                bin_op_simd_float_first::<Double>(this, which, left, right, dest)?;
            }
            // Used to implement _mm_min_pd and _mm_max_pd functions.
            // Note that the semantics are a bit different from Rust simd_min
            // and simd_max intrinsics regarding handling of NaN and -0.0: Rust
            // matches the IEEE min/max operations, while x86 has different
            // semantics.
            "min.pd" | "max.pd" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let which = match unprefixed_name {
                    "min.pd" => FloatBinOp::Min,
                    "max.pd" => FloatBinOp::Max,
                    _ => unreachable!(),
                };

                bin_op_simd_float_all::<Double>(this, which, left, right, dest)?;
            }
            // Used to implement _mm_sqrt_sd functions.
            // Performs the operations on the first component of `op` and
            // copies the remaining components from `op`.
            "sqrt.sd" => {
                let [op] = this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (op, op_len) = this.operand_to_simd(op)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                assert_eq!(dest_len, op_len);

                let op0 = this.read_scalar(&this.project_index(&op, 0)?)?.to_u64()?;
                // FIXME using host floats
                let res0 = Scalar::from_u64(f64::from_bits(op0).sqrt().to_bits());
                this.write_scalar(res0, &this.project_index(&dest, 0)?)?;

                for i in 1..dest_len {
                    this.copy_op(&this.project_index(&op, i)?, &this.project_index(&dest, i)?)?;
                }
            }
            // Used to implement _mm_sqrt_pd functions.
            // Performs the operations on all components of `op`.
            "sqrt.pd" => {
                let [op] = this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (op, op_len) = this.operand_to_simd(op)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                assert_eq!(dest_len, op_len);

                for i in 0..dest_len {
                    let op = this.read_scalar(&this.project_index(&op, i)?)?.to_u64()?;
                    let dest = this.project_index(&dest, i)?;

                    // FIXME using host floats
                    let res = Scalar::from_u64(f64::from_bits(op).sqrt().to_bits());

                    this.write_scalar(res, &dest)?;
                }
            }
            // Used to implement the _mm_cmp*_sd functions.
            // Performs a comparison operation on the first component of `left`
            // and `right`, returning 0 if false or `u64::MAX` if true. The remaining
            // components are copied from `left`.
            // _mm_cmp_sd is actually an AVX function where the operation is specified
            // by a const parameter.
            // _mm_cmp{eq,lt,le,gt,ge,neq,nlt,nle,ngt,nge,ord,unord}_sd are SSE2 functions
            // with hard-coded operations.
            "cmp.sd" => {
                let [left, right, imm] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let which =
                    FloatBinOp::cmp_from_imm(this, this.read_scalar(imm)?.to_i8()?, link_name)?;

                bin_op_simd_float_first::<Double>(this, which, left, right, dest)?;
            }
            // Used to implement the _mm_cmp*_pd functions.
            // Performs a comparison operation on each component of `left`
            // and `right`. For each component, returns 0 if false or `u64::MAX`
            // if true.
            // _mm_cmp_pd is actually an AVX function where the operation is specified
            // by a const parameter.
            // _mm_cmp{eq,lt,le,gt,ge,neq,nlt,nle,ngt,nge,ord,unord}_pd are SSE2 functions
            // with hard-coded operations.
            "cmp.pd" => {
                let [left, right, imm] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let which =
                    FloatBinOp::cmp_from_imm(this, this.read_scalar(imm)?.to_i8()?, link_name)?;

                bin_op_simd_float_all::<Double>(this, which, left, right, dest)?;
            }
            // Used to implement _mm_{,u}comi{eq,lt,le,gt,ge,neq}_sd functions.
            // Compares the first component of `left` and `right` and returns
            // a scalar value (0 or 1).
            "comieq.sd" | "comilt.sd" | "comile.sd" | "comigt.sd" | "comige.sd" | "comineq.sd"
            | "ucomieq.sd" | "ucomilt.sd" | "ucomile.sd" | "ucomigt.sd" | "ucomige.sd"
            | "ucomineq.sd" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, right_len) = this.operand_to_simd(right)?;

                assert_eq!(left_len, right_len);

                let left = this.read_scalar(&this.project_index(&left, 0)?)?.to_f64()?;
                let right = this.read_scalar(&this.project_index(&right, 0)?)?.to_f64()?;
                // The difference between the com* and ucom* variants is signaling
                // of exceptions when either argument is a quiet NaN. We do not
                // support accessing the SSE status register from miri (or from Rust,
                // for that matter), so we treat both variants equally.
                let res = match unprefixed_name {
                    "comieq.sd" | "ucomieq.sd" => left == right,
                    "comilt.sd" | "ucomilt.sd" => left < right,
                    "comile.sd" | "ucomile.sd" => left <= right,
                    "comigt.sd" | "ucomigt.sd" => left > right,
                    "comige.sd" | "ucomige.sd" => left >= right,
                    "comineq.sd" | "ucomineq.sd" => left != right,
                    _ => unreachable!(),
                };
                this.write_scalar(Scalar::from_i32(i32::from(res)), dest)?;
            }
            // Use to implement the _mm_cvtsd_si32, _mm_cvttsd_si32,
            // _mm_cvtsd_si64 and _mm_cvttsd_si64 functions.
            // Converts the first component of `op` from f64 to i32/i64.
            "cvtsd2si" | "cvttsd2si" | "cvtsd2si64" | "cvttsd2si64" => {
                let [op] = this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;
                let (op, _) = this.operand_to_simd(op)?;

                let op = this.read_immediate(&this.project_index(&op, 0)?)?;

                let rnd = match unprefixed_name {
                    // "current SSE rounding mode", assume nearest
                    // https://www.felixcloutier.com/x86/cvtsd2si
                    "cvtsd2si" | "cvtsd2si64" => rustc_apfloat::Round::NearestTiesToEven,
                    // always truncate
                    // https://www.felixcloutier.com/x86/cvttsd2si
                    "cvttsd2si" | "cvttsd2si64" => rustc_apfloat::Round::TowardZero,
                    _ => unreachable!(),
                };

                let res = this.float_to_int_checked(&op, dest.layout, rnd)?.unwrap_or_else(|| {
                    // Fallback to minimum acording to SSE semantics.
                    ImmTy::from_int(dest.layout.size.signed_int_min(), dest.layout)
                });

                this.write_immediate(*res, dest)?;
            }
            // Used to implement the _mm_cvtsd_ss and _mm_cvtss_sd functions.
            // Converts the first f64/f32 from `right` to f32/f64 and copies
            // the remaining elements from `left`
            "cvtsd2ss" | "cvtss2sd" => {
                let [left, right] =
                    this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;

                let (left, left_len) = this.operand_to_simd(left)?;
                let (right, _) = this.operand_to_simd(right)?;
                let (dest, dest_len) = this.mplace_to_simd(dest)?;

                assert_eq!(dest_len, left_len);

                // Convert first element of `right`
                let right0 = this.read_immediate(&this.project_index(&right, 0)?)?;
                let dest0 = this.project_index(&dest, 0)?;
                // `float_to_float_or_int` here will convert from f64 to f32 (cvtsd2ss) or
                // from f32 to f64 (cvtss2sd).
                let res0 = this.float_to_float_or_int(&right0, dest0.layout)?;
                this.write_immediate(*res0, &dest0)?;

                // Copy remianing from `left`
                for i in 1..dest_len {
                    this.copy_op(&this.project_index(&left, i)?, &this.project_index(&dest, i)?)?;
                }
            }
            // Used to implement the `_mm_pause` function.
            // The intrinsic is used to hint the processor that the code is in a spin-loop.
            "pause" => {
                let [] = this.check_shim(abi, Abi::C { unwind: false }, link_name, args)?;
                this.yield_active_thread();
            }
            _ => return Ok(EmulateForeignItemResult::NotSupported),
        }
        Ok(EmulateForeignItemResult::NeedsJumping)
    }
}

/// Takes a 128-bit vector, transmutes it to `[u64; 2]` and extracts
/// the first value.
fn extract_first_u64<'tcx>(
    this: &crate::MiriInterpCx<'_, 'tcx>,
    op: &MPlaceTy<'tcx, Provenance>,
) -> InterpResult<'tcx, u64> {
    // Transmute vector to `[u64; 2]`
    let u64_array_layout = this.layout_of(Ty::new_array(this.tcx.tcx, this.tcx.types.u64, 2))?;
    let op = op.transmute(u64_array_layout, this)?;

    // Get the first u64 from the array
    this.read_scalar(&this.project_index(&op, 0)?)?.to_u64()
}
