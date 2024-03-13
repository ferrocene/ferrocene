.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Installation Procedures
=======================

Installing Prerequisites
------------------------

See :doc:`User Manual - System requirements <user-manual:system-requirements>`
for the detailed installation prerequisites for Ferrocene.

Installing Ferrocene
--------------------

See :doc:`User Manual - Installation <user-manual:install>` for the
detailed installation procedure for Ferrocene.

Installation Validation
-----------------------

Ferrocene provides a checker called ``ferrocene-self-test`` (see
:ref:`Ferrocene - Validation <user-manual:install:Validation>` for details),
for verifying the installation of the toolchain in a non-certification context.

This tool is not qualified. Consequently, in certification context, the
following manual checks must be performed as per the :doc:`User Exported
Constraints <safety-manual:constraints>`:

- the tarballs were extracted correctly, and permissions were preserved
- only one version of Ferrocene is installed in the same directory, no
  duplicate versions or in-place updates
- the used linker driver should be valid according to the description in the
  :ref:`Linker Options <safety-manual:linker-options>`
