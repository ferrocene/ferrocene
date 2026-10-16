.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Uncertified and unused code
===========================

Uncertified code
----------------

It has to be ensured that no uncertified code from the certified libraries is being used in a customer project.

This is achieved in two steps:

#. Ferrous Systems ensures that the certified subset only contains certified code.

#. Customers ensure to only use code from the certified subset.

Both ensuring that the subset only contains certified code and that only certified code is used, is achieved with the ``ferrocene::unvalidated`` lint.

All uncertified code, and certified code that is not called, is unused code.

Unused code
-----------

The qualified Ferrocene compiler ensures that no code that is not used in source code is being executed.

Additionally, the compiler usually removes unused functions from the final binary.
But this behavior is not specified and depends on the configuration of compiler and linker.
Therefore it cannot not be relied upon.
