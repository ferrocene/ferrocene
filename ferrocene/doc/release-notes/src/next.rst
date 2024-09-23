.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

:upcoming-release:

Next Ferrocene release
======================

This page contains the changes to be introduced in the upcoming Ferrocene
release.

New features
------------

* The ``rustfmt`` code formatter version |rustfmt_version| is now supported and
  qualified for safety critical use.
* Two new qualified targets have been added.
  
  * :target-with-triple:`aarch64-unknown-nto-qnx710`
  * :target-with-triple:`x86_64-pc-nto-qnx710`

New experimental features
-------------------------

Experimental features are not qualified for safety critical use, and are
shipped as a preview.

* Experimental support has been added for a new host platform.
  Note that experimental targets are not qualified for safety critical use. The
  new target is:

  * :target-with-triple:`aarch64-unknown-linux-gnu`
