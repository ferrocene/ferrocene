.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

:upcoming-release:

Next Ferrocene release
======================

This page contains the changes to be introduced in the upcoming Ferrocene
release.

New features
------------

* A new Supported target has been added. Supported targets can often be qualified or quality
  managed upon request. The new target is:

  * :target-with-tuple:`aarch64v8r-unknown-none`

* Certify the ``core::fmt`` module.

  * This module contains the code to format Rust data structures into human-readable output. This enables customers to use this functionality in certified contexts.
  * Retire "certified panic runtime" compilation targets. They were used to certify panicking without certifying the formatting code, but are now obsolete. Following targets are being retired:

    * ``aarch64-ferrocene-none``
    * ``thumbv7em-ferrocene-none-eabi``
    * ``thumbv7em-ferrocene-none-eabihf``
    * ``x86_64-ferrocene-none``

New experimental features
-------------------------

Experimental features are not qualified for safety critical use, and are
shipped as a preview.

* Experimental support has been added for a new target. Note that experimental targets are
  not qualified for safety critical use. The new target is:

  * :target-with-tuple:`aarch64v8r-unknown-none-softfloat`
