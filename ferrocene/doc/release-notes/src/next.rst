.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

:upcoming-release:

Next Ferrocene release
======================

This page contains the changes to be introduced in the upcoming Ferrocene
release.

New experimental features
-------------------------

Experimental features are not qualified for safety critical use, and are
shipped as a preview.

* Experimental support has been added for ``cargo miri``, an unstable undefined
  behavior detection tool. Other than installation, usage is
  `according to upstream documentation <https://github.com/rust-lang/miri>`_.
  
  The package is available as:

  * ``miri-${rustc-host}``

* The testing facades used to test ``core`` are now available for customer use.
  On their respective target pages, the following targets now have an equivalent
  testing facade which can be used for testing:

  * :target-with-tuple:`aarch64-unknown-none`
  * :target-with-tuple:`thumbv7em-none-eabi`
  * :target-with-tuple:`thumbv7em-none-eabihf`
