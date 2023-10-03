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

The environment variables listed below were identified as potential sources of
errors as they may lead to incorrect behavior of the Ferrocene toolchain. Users
must verify that the values of these variables have not been set prior to using
the Ferrocene toolchain.

The following environment variables control or specify the inputs to the
compiler.

.. list-table:: Input Environment Variables
   :align: left
   
   * - ``HOST`` 
   * - ``CARGO``
   * - ``CARGO_ENCODED_RUSTFLAGS``

The following environment variables control or specify the inputs to the
compiler.

.. list-table:: Output Environment Variables
   :align: left

   * - ``CARGO_CACHE_RUSTC_INFO``
   * - ``CARGO_CFG_TARGET_ARCH``
   * - ``CARGO_CFG_TARGET_ENDIAN``
   * - ``CARGO_CFG_TARGET_ENV``
   * - ``CARGO_CFG_TARGET_FAMILY``
   * - ``CARGO_CFG_TARGET_FEATURE``
   * - ``CARGO_CFG_TARGET_OS``
   * - ``CARGO_CFG_TARGET_POINTER_WIDTH``
   * - ``CARGO_CFG_TARGET_VENDOR``
   * - ``CARGO_CFG_UNIX``
   * - ``CARGO_ENCODED_RUSTFLAGS`` 
