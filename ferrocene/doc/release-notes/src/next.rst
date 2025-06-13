.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

:upcoming-release:

Next Ferrocene release
======================

This page contains the changes to be introduced in the upcoming Ferrocene
release.


New experimental features
-------------------------

* The proxy targets used to test ``core`` are now available for customer use.
  On their respective target pages, the following targets now have an equivalent
  a ``ferrocenecoretest`` proxy target which can be used for testing:

  * :target-with-triple:`aarch64-unknown-none`
  * :target-with-triple:`thumbv7em-none-eabi`
  * :target-with-triple:`thumbv7em-none-eabihf`

Removed experimental features
-----------------------------

Experimental features are not qualified for safety critical use, and are
shipped as a preview. In some circumstances, these features are removed.

* Experimental support has been removed for the following platforms:

  * :target-with-triple:`x86_64-apple-darwin`
