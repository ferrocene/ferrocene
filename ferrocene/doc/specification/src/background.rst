.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Rust Project Contributors

.. default-domain:: spec

.. informational-page::

.. _fls_mLo6B3EWF50J:

FLS Background
==============

:dp:`fls_p7Jiyppmg0FU` The FLS is a document describing aspects of the Rust
language for Rust toolchain qualification purposes.

:dp:`fls_Uvf5tHsKSV19` It was created by Ferrous Systems, in an original joint
effort with AdaCore, as one of the prerequisites for qualifying `Ferrocene`_, a
Rust toolchain qualified for safety-critical environments. The FLS is compiled
of existing Rust documentation, but presented with a rigorous structure in order
to meet the requirements of qualification.

:dp:`fls_J7ZI7mBXffzZ` The FLS is not intended to be used as *the* normative
specification of the Rust language (see the `Rust Reference`_), nor is it meant
to replace the decision-making processes of the Rust project. Any difference
between the FLS and the behavior of the Rust compiler is considered an error on
our part and the FLS will be updated accordingly.

:dp:`fls_ffv8XSbBOMkR` The FLS text is licensed under either the ``MIT`` or
``Apache-2.0`` licenses, at your option. Individual files might have different
licensing. Licensing metadata is present in each file, and the full licenses
text is present in the ``LICENSES/`` directory.

.. _Ferrocene: https://ferrocene.dev
.. _Rust Reference: https://doc.rust-lang.org/reference/

.. _fls_UMsvnuLqzd99:

Acknowledging Ferrous Systems
-----------------------------

:dp:`fls_lmlLUAdtfCo5` The Rust Project would like to thank `Ferrous Systems`_
for `donating`_ the FLS (formerly the Ferrocene Language Specification) to the
Rust Project for its continued maintenance and development.

:dp:`fls_FZRrMT5AYsAQ` As a brief history, the FLS is a description of the Rust
programming language, developed by Ferrous Systems and `AdaCore`_ in July 2022
as part of Ferrocene, a Rust compiler and toolchain designed for safety-critical
and regulated industries. The FLS provides a structured and detailed reference
for Rust's syntax, semantics, and behavior, serving as a foundation for
verification, compliance, and standardization efforts. The FLS represented a
major step toward describing Rust in a way that aligns with industry
requirements, particularly in high-assurance domains. Until its transfer in
April 2025, Ferrous Systems had been the sole steward of the FLS since July
2023.

.. _Ferrous Systems: https://ferrous-systems.com
.. _donating: https://rustfoundation.org/media/ferrous-systems-donates-ferrocene-language-specification-to-rust-project/
.. _AdaCore: https://www.adacore.com