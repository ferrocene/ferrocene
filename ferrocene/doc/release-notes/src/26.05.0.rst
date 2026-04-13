.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

:upcoming-release:

Next Ferrocene release
======================

This page contains the changes to be introduced in the upcoming Ferrocene
release.

New features
------------

* New Supported targets have been added. Supported targets can often be qualified or quality
  managed upon request. The new targets are:

  * :target-with-tuple:`aarch64v8r-unknown-none`
  * :target-with-tuple:`aarch64-rhivos2-linux-gnu`
  * :target-with-tuple:`s390x-unknown-linux-gnu`
  * :target-with-tuple:`powerpc64le-unknown-linux-gnu`

* Certify the ``core::fmt`` module.

  * This module contains the code to format Rust data structures into human-readable output. This enables customers to use this functionality in certified contexts.
  * Retire "certified panic runtime" compilation targets. They were used to certify panicking without certifying the formatting code, but are now obsolete. Following targets are being retired:

    * ``aarch64-ferrocene-none``
    * ``thumbv7em-ferrocene-none-eabi``
    * ``thumbv7em-ferrocene-none-eabihf``
    * ``x86_64-ferrocene-none``

* Add a ``ferrocene::unvalidated`` lint which detects whether your code uses functions that
  are not in the certified subset of the standard library.

  * For information on how to use this lint, see :doc:`safety-manual:core/subset`.

  * For more background on why this change is being made, see
    `Callgraph Analysis <https://ferrous-systems.com/blog/callgraph-analysis/>`_.

  * Retire "certified subset" compilation targets. They were used to detect use of
    uncertified functionality, but are now obsolete. The following targets are being
    retired:

    * ``aarch64-unknown-ferrocene.subset``
    * ``thumbv7em-ferrocene-subset.eabi``
    * ``thumbv7em-ferrocene-subset.eabihf``
    * ``x86_64-unknown-ferrocene.subset``

New experimental features
-------------------------

Experimental features are not qualified for safety critical use, and are
shipped as a preview.

* Experimental support has been added for a new target. Note that experimental targets are
  not qualified for safety critical use. The new target is:

  * :target-with-tuple:`aarch64v8r-unknown-none-softfloat`
