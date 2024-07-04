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

* Experimental support has been added for new cross-compilation targets.
  Note that experimental targets are not qualified for safety critical use. The
  new targets are:

  * :target-with-triple:`armv8r-none-eabihf`
  * :target-with-triple:`armv7r-none-eabihf`
  * :target-with-triple:`armebv7r-none-eabihf`
  * :target-with-triple:`x86_64-apple-darwin`
  * :target-with-triple:`aarch64-unknown-nto-qnx710`
  * :target-with-triple:`x86_64-pc-nto-qnx710`

* Experimental support has been added for a new host platform.
  Note that experimental targets are not qualified for safety critical use. The
  new target is:

  * :target-with-triple:`aarch64-apple-darwin`
  * :target-with-triple:`x86_64-pc-windows-msvc`

Changes
-------

* The :target-with-triple:`x86_64-unknown-linux-gnu` target now requires
  glibc version of 2.31 or higher.
