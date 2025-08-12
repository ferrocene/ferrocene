.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Usage
=====

Use the certified subset of libcore exactly how you'd use standard Rust libcore,
with the following additions and modifications:

- Always take into account the safety constraints of all libcore functions you are using.
- Follow the guidance in :doc:`Handling Unsafety </rustc/unsafety>`
