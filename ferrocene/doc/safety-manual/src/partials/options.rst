.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. |Tool| replace:: DEFAULT
.. |Link| replace:: DEFAULT

These CLI options must be utilized for each invocation of |Tool| to remain within the qualified scope.

For convenience, |Link| provides further details on CLI options available to the end user.

CLI options that impact code generation and can create unsafe conditions are tagged with a caution note. A CLI option or one of its arguments may be outside the scope of the |Tool| qualification, in which case the CLI option or argument must not be used in a safety critical context. For convenience to the end user, these limitations are provided directly in the CLI caution note when applicable.

Other CLI options which are not marked with a caution note, for example those used for debugging and testing, can be used since their behavior is clearly defined and tested by the |Tool| testsuite.

If the end user requires CLI options other than those identified in this chapter, they must verify that the feature exists and is compatible with their target, that their safety context is compatible with the use of the CLI option or feature, and develop appropriate verification activities and tests to ensure the correct functionality of the generated code.
