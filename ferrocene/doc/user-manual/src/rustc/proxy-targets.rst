.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Proxy Targets
=============

This chapter describes how to use a Proxy Target to validate the operation of a bare-metal target.

Bare-metal targets do not have an operating system (OS), this means they do not link to ``libc`` or have
access to ``std`` functionality like ``println!("printing")`` or ``assert!()``.

Alongside qualified bare-metal targets additional ``ferrocenecoretest`` targets can be
:doc:`installed </rustc/install>` which act as a proxy to the respective bare-metal target,
while also providing access to things like profiling support for the purposes of testing and tooling.


Prerequisites
-------------

.. note::

   Proxy Targets are only supported on the :target:`aarch64-unknown-linux-gnu` and
   :ref:`x86_64-unknown-linux-gnu` host platforms at this time.

Using a proxy target requires ``qemu`` and ``binfmt`` to be configured correctly for the target
architecture. Setup instructions differ based on operating system and distribution. While
developing Ferrocene we use the instructions found in :doc:`internal-procedures:testing-other-targets`.



For coverage, you'll also need to install ``rustfilt``, as well as ``rust-profdata`` (from ``cargo-binutils``).

.. code-block:: rust
   
   cargo install rustfilt
   cargo install cargo-binutils

Using Proxy Targets
-------------------

Proxy Targets for Coverage
^^^^^^^^^^^^^^^^^^^^^^^^^^

We are going to write a simple library that can be both built as a library on a bare-metal target and
as an executable on a proxy target.


.. code-block:: rust
   
   // src/thing.rs

   #![cfg_attr(not(target_os = "linux"), no_std)]
   #![no_main]

   #[cfg(target_os = "linux")]
   use std::ffi::c_int;
   #[cfg(not(target_os = "linux"))]
   use core::ffi::c_int;

   #[unsafe(no_mangle)]
   extern "C" fn return_a_number(even: bool) -> c_int {
      if even {
         return 2;
      } else {
         return 1;
      }
   }

   #[unsafe(no_mangle)]
   fn main() {
      assert_eq!(return_a_number(false), 1);
      // assert_eq!(return_a_number(true), 2);
   }


   #[cfg(not(target_os = "linux"))]
   #[panic_handler]
   fn panic(_info: &core::panic::PanicInfo) -> ! {
      loop {}
   }

We can build for a bare-metal target (:ref:`thumbv7em-none-eabihf`) by running:

.. code-block::

   rustc --target thumbv7em-none-eabihf src/thing.rs --out-dir artifacts

We can build for the equivalent proxy target, with instrumentation, by running:

.. code-block::
   
   rustc --target thumbv7em-ferrocenecoretest-eabihf src/thing.rs --out-dir artifacts -C instrument-coverage


To create the ``profraw`` file:

.. code-block::
   
   LLVM_PROFILE_FILE="thing.profraw" artifacts/thing

Then create the ``profdata``:

.. code-block::
   
   rust-profdata merge --sparse thing.profraw -o thing.profdata

Then create the coverage report:

.. code-block::
   
   rust-cov report  -Xdemangler=rustfilt artifacts/thing \
      --instr-profile=thing.profdata \
      --show-instantiation-summary

That should output something like the following:

.. code-block::
   
   Filename              Regions    Missed Regions     Cover   Functions  Missed Functions  Executed  Instantiations   Missed Insts.  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
   ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
   $CWD/src/thing.rs           9                 1    88.89%           2                 0   100.00%               2               0   100.00%           8                 1    87.50%           0                 0         -
   ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
   TOTAL                       9                 1    88.89%           2                 0   100.00%               2               0   100.00%           8                 1    87.50%           0                 0         -

Not bad, but the coverage could be improved. Uncomment the ``assert_eq!(return_a_number(true), 2);``
line and run it again. Coverage should now be 100%.