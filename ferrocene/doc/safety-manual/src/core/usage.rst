.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Usage
=====

Use the certified core library exactly how you'd use the normal Rust core library,
with the following additions and modifications:

- Always take into account the safety constraints of all core library functions you are using.
- Follow the guidance in :doc:`Handling Unsafety </rustc/unsafety>`.
