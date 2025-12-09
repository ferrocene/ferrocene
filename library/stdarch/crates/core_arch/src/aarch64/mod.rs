//! AArch64 intrinsics.
//!
//! The reference for NEON is [Arm's NEON Intrinsics Reference][arm_ref]. The
//! [Arm's NEON Intrinsics Online Database][arm_dat] is also useful.
//!
//! [arm_ref]: http://infocenter.arm.com/help/topic/com.arm.doc.ihi0073a/IHI0073A_arm_neon_intrinsics_ref.pdf
//! [arm_dat]: https://developer.arm.com/technologies/neon/intrinsics

#![cfg_attr(
    all(target_arch = "aarch64", target_abi = "softfloat"),
    // Just allow the warning: anyone soundly using the intrinsics has to enable
    // the target feature, and that will generate a warning for them.
    allow(aarch64_softfloat_neon)
)]

#[cfg(not(feature = "ferrocene_subset"))]
mod mte;
#[unstable(feature = "stdarch_aarch64_mte", issue = "129010")]
#[cfg(not(feature = "ferrocene_subset"))]
pub use self::mte::*;

mod neon;
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub use self::neon::*;

#[cfg(not(feature = "ferrocene_subset"))]
mod prefetch;
#[unstable(feature = "stdarch_aarch64_prefetch", issue = "117217")]
#[cfg(not(feature = "ferrocene_subset"))]
pub use self::prefetch::*;

#[stable(feature = "neon_intrinsics", since = "1.59.0")]
#[cfg(not(feature = "ferrocene_subset"))]
pub use super::arm_shared::*;

#[cfg(test)]
#[cfg(not(feature = "ferrocene_subset"))]
use stdarch_test::assert_instr;

#[cfg(test)]
#[cfg(not(feature = "ferrocene_subset"))]
pub(crate) mod test_support;
