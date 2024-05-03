.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Constraints
===========

To use ``rustfmt`` in a safety critical setup you need to adhere to following workflow:

1. Make a change to the source code of your crate.
2. Commit or stage the change. The working tree must be clean.
3. Run ``rustfmt`` on the Rust files of your crate.
4. Perform following checks to ensure correct behaviour of ``rustfmt``
    1. Compile your crate, to ensure that ``rustfmt`` did not introduce any syntax errors.
    2. Review the changes ``rustfmt`` made, to ensure there are only syntactic, but no semantic changes.
    3. Run your test suite, to ensure no semantic changes occured.
