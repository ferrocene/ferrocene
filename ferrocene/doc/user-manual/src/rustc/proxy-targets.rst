.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Proxy Targets
=============

This chapter describes how to use a Proxy Target to validate the operation of a bare-metal target.

Bare-metal targets do not have an operating system (OS), this means they do not link to ``libc`` or have
access to ``std`` functionality like ``println!("printing")`` or ``assert_eq!()``.

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

Proxy targets enable profiling support, as well as access to ``std`` (such as ``assert_eq!()``), compared to their bare metal
targets. Behind the scenes, the proxy targets are providing the equivalent of a Linux ``libc`` to the binary.

The following configurations can be used to flag code to only run on the proxy targets:

.. code-block:: rust
   
   #[cfg(target_os = "linux")]
   fn only_on_proxy() { /* ... */ }

   #[cfg_attr(target_os = "linux", derive(Debug))]
   struct OnlyHasDebugOnProxy { /* ... */ }

   #[cfg(target_os = "linux")]
   fn only_prints_on_proxy(value: u32) {
      let x = value;
      // ...

      #[cfg(target_os = "linux")]
      {
         println!("Recieved {x}");
      }

      // ...
      x
   }

This is useful for things like test harnesses, extra instrumentation, and debug assertions.

The following configures code to not be run on proxies (so, only on bare metal):

.. code-block:: rust
   
   #[cfg(not(target_os = "linux"))]
   fn only_on_bare_metal() {}

   #[cfg_attr(not(target_os = "linux"), no_mangle)]
   struct OnlyHasNoMangleOnBareMetal {}
   
   #[cfg(target_os = "linux")]
   fn only_does_thing_on_bare_metal() {
      #[cfg(not(target_os = "linux"))]
      {
         // Initialize on-board LED that the proxy can't use
         // ...
      }

      // Run calculations (proxy compatible)
      // ...
   }

This is useful when your code would normally bring up hardware not present on the proxy.

Testing
^^^^^^^

.. note::

   Currently ``cargo`` is not a qualified component of Ferrocene, its test harness
   (used during ``cargo test``) is also unqualified.

   If your organization needs a qualified test harness, or wishes to use ``cargo``
   in a qualified way, please contact sales.


Since proxy targets include a ``std`` it's possible to use ``assert!()`` with them. This means that
you can do basic logic testing without any hardware in the loop.

Let's assume we have a simple function to test:

.. code-block:: rust

   #[no_std]

   use core::ffi::c_int;

   #[unsafe(no_mangle)]
   extern "C" fn return_a_number(even: bool) -> c_int {
      if even {
         return 2;
      } else {
         return 1;
      }
   }

First, let's make sure this builds:

.. code-block:: bash

   rustc --target thumbv7em-none-eabihf --crate-type=staticlib src/thing.rs --out-dir artifacts

Now, we'll add support for the proxy target:

.. code-block:: rust

   #![cfg_attr(not(target_os = "linux"), no_std)]

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

Next, we'll write a simple test suite using assertions that runs only on the proxy target:

.. code-block:: rust

   #[test]
   fn it_works() {
      assert_eq!(return_a_number(false), 1);
      assert_eq!(return_a_number(true), 2);
   }

Build with the proxy target:

.. code-block:: bash
   
   rustc --target thumbv7em-ferrocenecoretest-eabihf --test -Z panic-abort-tests -C instrument-coverage src/thing.rs --out-dir artifacts

Then run it:

.. code-block:: bash
   
   $ ./artifacts/thing
   
   running 1 test
   test it_works ... ok

   test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

Try changing one of the assertions to be wrong then observe the failure:

.. code-block:: bash

   $ ./artifacts/thing

   running 1 test
   qemu: uncaught target signal 6 (Aborted) - core dumped
   SIGABRT: child process './artifacts/thing' core dumped


Code Coverage
^^^^^^^^^^^^^

We are going to write a simple library that can be both built as a library on a bare-metal target and
as an executable on a proxy target.


.. code-block:: rust
   
   // src/thing.rs

   #![cfg_attr(not(target_os = "linux"), no_std)]

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

   #[test]
   fn it_works() {
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

.. code-block:: bash
   
   rustc --target thumbv7em-ferrocenecoretest-eabihf --test -Z panic-abort-tests -C instrument-coverage src/thing.rs --out-dir artifacts

To create the ``profraw`` file:

.. code-block::
   
   LLVM_PROFILE_FILE="profiling/thing-%p-%m.profraw" artifacts/thing

Then create the ``profdata``:

.. code-block::
   
   rust-profdata merge --sparse profiling/thing-*.profraw -o profiling/thing.profdata

Then create the coverage report:

.. code-block::
   
   rust-cov report  -Xdemangler=rustfilt artifacts/thing \
      --instr-profile=profiling/thing.profdata \
      --show-instantiation-summary

That should output something like the following:

.. code-block::
   
Filename              Regions    Missed Regions     Cover   Functions  Missed Functions  Executed  Instantiations   Missed Insts.  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
$CWD/src/thing.rs           9                 1    88.89%           2                 0   100.00%               2               0   100.00%           8                 1    87.50%           0                 0         -
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                       9                 1    88.89%           2                 0   100.00%               2               0   100.00%           8                 1    87.50%           0                 0         -

Not bad, but the coverage could be improved! Uncomment the ``assert_eq!(return_a_number(true), 2);``
line and run it again. Coverage should now be 100%.