.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Proxy Targets
=============

This chapter describes how to use a Proxy Target to validate the operation of a bare-metal target.

Bare-metal targets do not have an operating system (OS), this means they link to ``libc`` or have
access to ``std`` functionality like ``println!("printing")`` or ``assert!()``.

Alongside qualified bare-metal targets additional ``ferrocenecoretest`` targets can be
:doc:`installed </rustc/install>` which act as a proxy to the respective bare-metal target,
while also providing access to ``std``.

Prerequisites
-------------

.. note::

   Proxy Targets are only supported on :ref:`aarch64-unknown-linux-gnu` and
   :ref:`x86_64-unknown-linux-gnu` at this time.

Using a proxy target requires ``qemu`` and ``binfmt`` to be configured correctly for the target
architecture. Setup instructions differ based on operating system and distribution.

While developing Ferrocene we use the instructions found in
:doc:`internal-procedures:testing-other-targets`.

Using Proxy Targets
-------------------

We are going to write a simple library that can be both built as a library on a bare-metal target and
as an executable on a proxy target.


.. code-block:: rust

   #![cfg_attr(not(target_os = "linux"), no_std)]

   #[cfg(target_os = "linux")]
   use std::ffi::c_int;
   #[cfg(not(target_os = "linux"))]
   use core::ffi::c_int;

   #[unsafe(no_mangle)]
   extern "C" fn return_a_number() -> c_int {
      return 1;
   }

   #[cfg(target_os = "linux")]
   fn main() {
      assert_eq!(return_a_number(), 1);
      assert_eq!(return_a_number(), 2); // Bang!
   }


   #[cfg(not(target_os = "linux"))]
   #[panic_handler]
   fn panic(_info: &core::panic::PanicInfo) -> ! {
      loop {}
   }

We can build a library for a bare-metal target (:ref:`thumbv7em-none-eabihf`) by running:

.. code-block::

   rustc --target thumbv7em-none-eabihf src/lib.rs --crate-type lib

We can build an executable for the equivalent proxy target by running:

.. code-block::
   
   rustc --target thumbv7em-ferrocenecoretest-eabihf src/lib.rs

