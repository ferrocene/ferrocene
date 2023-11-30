.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Tool Options
============

Ferocene is qualified exclusively for the following command line options:

- users shall pass command line option ``--edition 2021`` to each invocation of
  the Ferrocene compiler.

- users shall pass command line option ``-C opt-level=2`` to each invocation of
  the Ferrocene compiler.

- users shall pass command line option
  ``-C llvm-args=-protect-from-escaped-allocas=true`` to each invocation of the
  Ferrocene compiler.

- users shall pass all target-specific command line options, as listed in the
  page of the target in the :doc:`User Manual - Compilation Targets
  <user-manual:index>`.

These CLI options must be utilized for each invocation of the Ferrocene
toolset to remain within the qualified scope.

For convenience, :doc:`user-manual:cli` provides further details on CLI options
available to the end user.

CLI options that impact code generation and can create unsafe conditions are
tagged with a caution note. A CLI option or one of its arguments may be outside
the scope of the Ferrocene |iso_ref| qualification, in which case the CLI
option or argument must not be used in a safety critical context. For
convenience to the end user, these limitations are provided directly in the CLI
caution note when applicable.

Other CLI options which are not marked with a caution note, for example those
used for debugging and testing, can be used since their behavior is clearly
defined and tested by the Ferrocene testsuite.

If the end user requires CLI options other than those identified in this
chapter, they must verify that the feature exists and is compatible with their
target, that their safety context is compatible with the use of CLI option or
feature, and develop appropriate verification activities and tests to ensure
the correct functionality of the generated code.
