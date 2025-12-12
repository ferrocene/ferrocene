.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Uncertified and unused code
===========================

Uncertified code
----------------

It has to be ensured that no uncertified code from the core library is being used in a customer project.

This is achieved in three steps.

Firstly, it is ensured that the certified subset only contains certified code.

Secondly, customers must ensure they only use code from the certified subset.

Thirdly,  customers must ensure they only use a certified target in their final build.

All uncertified code, and certified code that is not called, is unused code.

Unused code
-----------

The qualified Ferrocene compiler ensures that no code that is not used in source code is being executed.

Additionally the compiler usually removes unused functions from the final binary. But this behavior is not specified and can therefore not be relied upon.
