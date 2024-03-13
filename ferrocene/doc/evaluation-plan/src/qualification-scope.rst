.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Qualification Scope
===================

The Ferrocene qualification encompasses the qualification of the
Ferrocene compiler's functionality as identified in the
:doc:`user-manual:index` and the :doc:`specification:index`

The Ferrocene compiler links to libraries ``liballoc``, ``libcore``, and
``libstd``. These libraries are evaluated and tested within the scope of
Ferrocene qualification *for compiler use only*. The use of these libraries
by end-use code is outside the scope of the current Ferrocene
qualification. It is the end-user responsibility to qualify these libraries if
they are used in their code.

The other development tools besides ``rustc`` are not qualified and are
distributed for convenience only. This includes but is not limited to:

* ``cargo``
* ``cargo-clippy``
* ``rust-analyzer``
* ``rust-demangler``
* ``rust-gdb``
* ``rustdoc``
* ``rustfmt``
* the various llvm tools (example being ``llvm-objcopy``)
