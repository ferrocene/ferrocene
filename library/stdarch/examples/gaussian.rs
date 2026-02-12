//! Hexagon HVX Gaussian 3x3 blur example
//!
//! This example demonstrates the use of Hexagon HVX intrinsics to implement
//! a 3x3 Gaussian blur filter on unsigned 8-bit images.
//!
//! The 3x3 Gaussian kernel is:
//!     1 2 1
//!     2 4 2  / 16
//!     1 2 1
//!
//! This is a separable filter: `[1 2 1]^T * [1 2 1] / 16`.
//! Each 1D pass of `[1 2 1] / 4` is computed using byte averaging:
//!     avg(avg(a, c), b) ≈ (a + 2b + c) / 4
//!
//! This approach uses only `HvxVector` (single-vector) operations, avoiding
//! `HvxVectorPair` which currently has ABI limitations in the Rust/LLVM
//! Hexagon backend.
//!
//! To build:
//!
//!     RUSTFLAGS="-C target-feature=+hvxv60,+hvx-length128b \
//!         -C linker=hexagon-unknown-linux-musl-clang" \
//!         cargo +nightly build --bin gaussian -p stdarch_examples \
//!         --target hexagon-unknown-linux-musl \
//!         -Zbuild-std -Zbuild-std-features=llvm-libunwind
//!
//! To run under QEMU:
//!
//!     qemu-hexagon -L <sysroot>/target/hexagon-unknown-linux-musl \
//!         target/hexagon-unknown-linux-musl/debug/gaussian

#![cfg(target_arch = "hexagon")]
#![feature(stdarch_hexagon)]
#![feature(hexagon_target_feature)]
#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::missing_docs_in_private_items,
    clippy::cast_possible_wrap,
    clippy::cast_ptr_alignment,
    dead_code
)]

#[cfg(not(target_feature = "hvx-length128b"))]
use core_arch::arch::hexagon::v64::*;
#[cfg(target_feature = "hvx-length128b")]
use core_arch::arch::hexagon::v128::*;

/// Vector length in bytes for HVX 128-byte mode
#[cfg(target_feature = "hvx-length128b")]
const VLEN: usize = 128;

/// Vector length in bytes for HVX 64-byte mode
#[cfg(not(target_feature = "hvx-length128b"))]
const VLEN: usize = 64;

/// Image width - must be multiple of VLEN
const WIDTH: usize = 256;
const HEIGHT: usize = 16;

/// Vertical 1-2-1 filter pass using byte averaging
///
/// Computes: dst[x] = avg(avg(row_above[x], row_below[x]), center[x])
///                   ≈ (row_above[x] + 2*center[x] + row_below[x]) / 4
///
/// # Safety
///
/// - `src` must point to the center row with valid data at -stride and +stride
/// - `dst` must point to a valid output buffer for `width` bytes
/// - `width` must be a multiple of VLEN
/// - All pointers must be HVX-aligned (128-byte for 128B mode)
#[target_feature(enable = "hvxv60")]
unsafe fn vertical_121_pass(src: *const u8, stride: isize, width: usize, dst: *mut u8) {
    let inp0 = src.offset(-stride) as *const HvxVector;
    let inp1 = src as *const HvxVector;
    let inp2 = src.offset(stride) as *const HvxVector;
    let outp = dst as *mut HvxVector;

    let n_chunks = width / VLEN;
    for i in 0..n_chunks {
        let above = *inp0.add(i);
        let center = *inp1.add(i);
        let below = *inp2.add(i);

        // avg(above, below) ≈ (above + below) / 2
        let avg_ab = q6_vub_vavg_vubvub_rnd(above, below);
        // avg(avg_ab, center) ≈ ((above + below)/2 + center) / 2
        //                     ≈ (above + 2*center + below) / 4
        let result = q6_vub_vavg_vubvub_rnd(avg_ab, center);

        *outp.add(i) = result;
    }
}

/// Horizontal 1-2-1 filter pass using byte averaging with vector alignment
///
/// Computes: dst[x] = avg(avg(src[x-1], src[x+1]), src[x])
///                   ≈ (src[x-1] + 2*src[x] + src[x+1]) / 4
///
/// Uses `valign` and `vlalign` to shift vectors by 1 byte for neighbor access.
///
/// # Safety
///
/// - `src` and `dst` must point to valid buffers of `width` bytes
/// - `width` must be a multiple of VLEN
/// - All pointers must be HVX-aligned
#[target_feature(enable = "hvxv60")]
unsafe fn horizontal_121_pass(src: *const u8, width: usize, dst: *mut u8) {
    let inp = src as *const HvxVector;
    let outp = dst as *mut HvxVector;

    let n_chunks = width / VLEN;
    let mut prev = q6_v_vzero();

    for i in 0..n_chunks {
        let curr = *inp.add(i);
        let next = if i + 1 < n_chunks {
            *inp.add(i + 1)
        } else {
            q6_v_vzero()
        };

        // Left neighbor (x-1): shift curr right by 1 byte, filling from prev
        // vlalign(curr, prev, 1) = { prev[VLEN-1], curr[0], curr[1], ..., curr[VLEN-2] }
        let left = q6_v_vlalign_vvr(curr, prev, 1);

        // Right neighbor (x+1): shift curr left by 1 byte, filling from next
        // valign(next, curr, 1) = { curr[1], curr[2], ..., curr[VLEN-1], next[0] }
        let right = q6_v_valign_vvr(next, curr, 1);

        // avg(left, right) ≈ (src[x-1] + src[x+1]) / 2
        let avg_lr = q6_vub_vavg_vubvub_rnd(left, right);
        // avg(avg_lr, curr) ≈ ((src[x-1] + src[x+1])/2 + src[x]) / 2
        //                   ≈ (src[x-1] + 2*src[x] + src[x+1]) / 4
        let result = q6_vub_vavg_vubvub_rnd(avg_lr, curr);

        *outp.add(i) = result;

        prev = curr;
    }
}

/// Apply Gaussian 3x3 blur to an entire image using separable filtering
///
/// Two-pass approach:
/// 1. Vertical pass: apply 1-2-1 filter across rows
/// 2. Horizontal pass: apply 1-2-1 filter across columns
///
/// Combined effect: 3x3 Gaussian kernel [1 2 1; 2 4 2; 1 2 1] / 16
///
/// # Safety
///
/// - `src` and `dst` must point to valid image buffers of `stride * height` bytes
/// - `tmp` must point to a valid temporary buffer of `width` bytes, HVX-aligned
/// - `width` must be a multiple of VLEN and >= VLEN
/// - `stride` must be >= `width`
/// - All buffers must be HVX-aligned (128-byte for 128B mode)
#[target_feature(enable = "hvxv60")]
pub unsafe fn gaussian3x3u8(
    src: *const u8,
    stride: usize,
    width: usize,
    height: usize,
    dst: *mut u8,
    tmp: *mut u8,
) {
    let stride_i = stride as isize;

    // Process interior rows (skip first and last which lack vertical neighbors)
    for y in 1..height - 1 {
        let row_src = src.offset(y as isize * stride_i);
        let row_dst = dst.offset(y as isize * stride_i);

        // Pass 1: vertical 1-2-1 into tmp
        vertical_121_pass(row_src, stride_i, width, tmp);

        // Pass 2: horizontal 1-2-1 from tmp into dst
        horizontal_121_pass(tmp, width, row_dst);
    }
}

/// Reference C implementation from Hexagon SDK (Gaussian3x3u8)
///
/// Kernel:
///     1 2 1
///     2 4 2  / 16
///     1 2 1
fn gaussian3x3u8_reference(src: &[u8], stride: usize, width: usize, height: usize, dst: &mut [u8]) {
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // Compute column sums (vertical 1-2-1 weights)
            let mut col = [0u32; 3];
            for i in 0..3 {
                col[i] = 1 * src[(y - 1) * stride + x - 1 + i] as u32
                    + 2 * src[y * stride + x - 1 + i] as u32
                    + 1 * src[(y + 1) * stride + x - 1 + i] as u32;
            }
            // Apply horizontal 1-2-1 weights and normalize
            // (1*col[0] + 2*col[1] + 1*col[2] + 8) / 16
            dst[y * stride + x] = ((1 * col[0] + 2 * col[1] + 1 * col[2] + 8) >> 4) as u8;
        }
    }
}

/// Scalar approximation matching the HVX byte-averaging approach
///
/// This matches the HVX implementation's behavior:
/// - Vertical: avg_rnd(avg_rnd(above, below), center)
/// - Horizontal: avg_rnd(avg_rnd(left, right), center)
/// where avg_rnd(a, b) = (a + b + 1) / 2
fn gaussian3x3u8_approx(src: &[u8], stride: usize, width: usize, height: usize, dst: &mut [u8]) {
    // Temporary buffer for vertical pass output
    let mut tmp = vec![0u8; width * height];

    // Vertical pass: 1-2-1 using rounding average
    for y in 1..height - 1 {
        for x in 0..width {
            let above = src[(y - 1) * stride + x] as u16;
            let center = src[y * stride + x] as u16;
            let below = src[(y + 1) * stride + x] as u16;
            let avg_ab = ((above + below + 1) / 2) as u8;
            tmp[y * width + x] = ((avg_ab as u16 + center + 1) / 2) as u8;
        }
    }

    // Horizontal pass: 1-2-1 using rounding average
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let left = tmp[y * width + (x - 1)] as u16;
            let center = tmp[y * width + x] as u16;
            let right = tmp[y * width + (x + 1)] as u16;
            let avg_lr = ((left + right + 1) / 2) as u8;
            dst[y * stride + x] = ((avg_lr as u16 + center + 1) / 2) as u8;
        }
    }
}

/// Generate deterministic test pattern
fn generate_test_pattern(buf: &mut [u8], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            buf[y * width + x] = ((x + y * 7) % 256) as u8;
        }
    }
}

fn main() {
    // Aligned buffers for HVX
    #[repr(align(128))]
    struct AlignedBuf<const N: usize>([u8; N]);

    let mut src = AlignedBuf::<{ WIDTH * HEIGHT }>([0u8; WIDTH * HEIGHT]);
    let mut dst_hvx = AlignedBuf::<{ WIDTH * HEIGHT }>([0u8; WIDTH * HEIGHT]);
    let mut tmp = AlignedBuf::<{ WIDTH }>([0u8; WIDTH]);
    let mut dst_ref = vec![0u8; WIDTH * HEIGHT];
    let mut dst_approx = vec![0u8; WIDTH * HEIGHT];

    // Generate test pattern
    generate_test_pattern(&mut src.0, WIDTH, HEIGHT);

    // Run HVX implementation
    unsafe {
        gaussian3x3u8(
            src.0.as_ptr(),
            WIDTH,
            WIDTH,
            HEIGHT,
            dst_hvx.0.as_mut_ptr(),
            tmp.0.as_mut_ptr(),
        );
    }

    // Run reference
    gaussian3x3u8_reference(&src.0, WIDTH, WIDTH, HEIGHT, &mut dst_ref);

    // Run scalar approximation (should match HVX exactly)
    gaussian3x3u8_approx(&src.0, WIDTH, WIDTH, HEIGHT, &mut dst_approx);

    // Verify HVX matches the byte-averaging approximation exactly
    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let idx = y * WIDTH + x;
            assert_eq!(
                dst_hvx.0[idx], dst_approx[idx],
                "HVX output differs from scalar approximation at ({}, {}): hvx={}, approx={}",
                x, y, dst_hvx.0[idx], dst_approx[idx]
            );
        }
    }

    // Verify HVX exactly matches reference for this test pattern
    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let idx = y * WIDTH + x;
            assert_eq!(
                dst_hvx.0[idx], dst_ref[idx],
                "HVX differs from reference at ({}, {}): hvx={}, ref={}",
                x, y, dst_hvx.0[idx], dst_ref[idx]
            );
        }
    }
}
