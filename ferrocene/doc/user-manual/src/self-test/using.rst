.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Using
=====

``ferrocene-self-test`` ensures that the Ferrocene toolchain is installed properly.

The most easy way to use the tool is with `CriticalUp <https://criticalup.ferrocene.dev>`_:

.. code::

   criticalup run ferrocene-self-test

If, instead, the toolchain was installed manually, the tool can be run directly:

.. code::

   <toolchain directory root>/bin/ferrocene-self-test

.. note::

   - The tool can only be used with the toolchain it is distributed with
   - The tool does not accept any command line options

All the performed checks will be displayed.
If any check fails, an error will be emitted along with an ID.
You can look up the identifier in :doc:`the error codes list <error-codes>`
to learn more about the failure and ways to fix it.

Environment variables
---------------------

The tool makes use of the following:

- **PATH**

  The tool will quit early if this variable is not found,
  because it is used to find particular executables as part of its checks.

- **FST_PRINT_DETAILED_ARGS**

  Specifying this variable will make the tool display more detailed output for some of its checks.

- **FST_PRINT_DETAILED_ERRORS**

  Specifying this variable will make the tool display more detailed errors for some of its checks.
