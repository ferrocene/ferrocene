.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Handling Unsafety
=================

:doc:`Unsafety <specification:unsafety>` is the presence of code that the
compiler cannot automatically prove with respect to the memory safety
guarantees of Rust. The use of unsafe code is necessary for operations such as
dereferencing a raw pointer, accessing or modifying a mutable static variable, or
directly interacting with the operating system and hardware. If implemented
incorrectly however, an unsafe operation may result in undefined behavior.
Consequently, it is the responsibility of the program author rather than the
compiler to ensure the unsafe code respects the memory safety guarantees of
Rust.

The user shall avoid operations that are known to result in undefined behavior,
as enumerated in
:doc:`List of Undefined Behavior <specification:undefined-behavior>`.

When considering writing unsafe code, the user shall first examine the unsafe
API of the ``core`` library for readily available functionality.

When writing unsafe code, the user at a minimum shall:

* Keep the use of unsafety as localized as possible, never exceeding the level
  of a module.

* Ensure that the said module fulfills exactly one task.

* Employ assertions, preconditions, postconditions, and invariants in both
  the unsafe code and its clients.

* Employ safety-related comments in both the unsafe code and its clients.

The following snippet is taken from module ``core::slice`` of the standard
library, and provides a good example on the use of defensive code and
documentation.

.. code-block:: rust

    . . .
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used. The caller has to ensure that
    /// `0 <= mid <= self.len()`.
    ///
    /// [`split_at_mut`]: slice::split_at_mut
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    . . .
    pub const unsafe fn split_at_mut_unchecked(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
        let len = self.len();
        let ptr = self.as_mut_ptr();

        // SAFETY: Caller has to check that `0 <= mid <= self.len()`.
        //
        // `[ptr; mid]` and `[mid; len]` are not overlapping, so returning a mutable reference
        // is fine.
        unsafe {
            (from_raw_parts_mut(ptr, mid), from_raw_parts_mut(ptr.add(mid), len - mid))
        }
    }

    . . .
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used. The caller has to ensure that
    /// `0 <= mid <= self.len()`.
    ///
    /// [`split_at`]: slice::split_at
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    . . .
    pub const fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
        assert!(mid <= self.len());
        // SAFETY: `[ptr; mid]` and `[mid; len]` are inside `self`, which
        // fulfills the requirements of `from_raw_parts_mut`.
        unsafe { self.split_at_mut_unchecked(mid) }
    }

    pub fn as_rchunks_mut<const N: usize>(&mut self) -> (&mut [T], &mut [[T; N]]) {
        assert_ne!(N, 0, "chunks cannot have a size of zero");
        let len = self.len() / N;
        let (remainder, multiple_of_n) = self.split_at_mut(self.len() - len * N);
        // SAFETY: We already panicked for zero, and ensured by construction
        // that the length of the subslice is a multiple of N.
        let array_slice = unsafe { multiple_of_n.as_chunks_unchecked_mut() };
        (remainder, array_slice)
    }

When verifying unsafe code, the user at a minimum shall:

* Perform code review by either:

  * At least two senior engineers, or

  * Utilize a third party review.

* Perform code review of the entire module where the unsafe code resides.

* Perform unit testing on the unsafe code and its clients,

* Perform integration testing between the unsafe code and its clients.

The user shall develop mitigation strategies for the unsafe code.
