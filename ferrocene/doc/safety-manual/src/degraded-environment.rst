.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Degraded Environment
====================

Ferrocene usage depends on the following environment attributes:

* the amount of available RAM,

* the correct tool installation,

* the value specific environment variables.

An insufficient amount of RAM will cause an out-of-memory error and prevent
code generation. An invalid installation or an incorrectly set environment
variable could lead to incorrect execution of the toolchain.

In case of invalid Ferrocene installation or configuration, the behavior of
the tool is unpredictable. The behavior can vary from a compilation error to an
undefined code generation. Therefore, the correctness of the Ferrocene
toolchain installation needs to be verified before use. This is covered by
constrain :id:`CSTR_0010_INSTALL`.

The following environment variables control or specify behavior of the
compiler.

.. list-table:: Configuration Environment Variables
   :align: left
   :header-rows: 1

   * - Environment Variable
     - Description

   * - ``RUST_MIN_STACK``
   * - Sets the default stack size for spawned threads of the compiler.


The environment variables listed below were identified as potential sources of
errors as they may lead to incorrect behavior of the Ferrocene toolchain. Users
must verify that the values of these variables have not been set prior to using
the Ferrocene toolchain.

The following environment variables may not be set prior to using the Ferrocene
toolchian.

.. list-table:: Forbidden Environment Variables
   :align: left

   * - ``RUST_TARGET_PATH``
   * - ``RUSTC_BOOTSTRAP``
