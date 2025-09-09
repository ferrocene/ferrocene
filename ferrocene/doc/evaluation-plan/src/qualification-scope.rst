.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Qualification Scope
===================

The Ferrocene qualification encompasses the qualification of the
Rust toolchain's functionality as identified in the
:doc:`user-manual:index` and the :doc:`specification:index`

The Rust compiler, ``rustc``, links to the libraries ``alloc``, ``core``, and
``std``. These libraries are evaluated and tested within the scope of
Ferrocene qualification *for compiler use only*. The use of these libraries
by end-use code is outside the scope of the current Ferrocene
qualification. It is the end-user responsibility to qualify these libraries if
they are used in their code.

Additionally a subset of the ``core`` library is certified for use in end-use code.

Qualified tools are:

* ``rustc``

Other development tools are not qualified and are distributed for convenience
only. This includes but is not limited to:

* ``cargo``
* ``cargo-clippy``
* ``rust-analyzer``
* ``rust-gdb``
* ``rustdoc``
* ``rustfmt``
* the various llvm tools (example being ``llvm-objcopy``)
