.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Usage
=====

Use the certified libraries exactly how you'd use the normal Rust standard libraries,
with the following additions and modifications:

- Always take into account the safety constraints of all functions of the certified libraries you are using.
- Follow the guidance in :doc:`Handling Unsafety </rustc/unsafety>`.
