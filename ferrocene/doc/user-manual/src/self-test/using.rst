.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Using
=====

``ferrocene-self-test`` ensures that the Ferrocene toolchain is installed properly.

The most easy way to use the tool is with `CriticalUp <https://criticalup.ferrocene.dev>`_:

.. code::

   criticalup run ferrocene-self-test

If, instead, the toolchain was installed manually, ``ferrocene-self-test`` can be run directly:

.. code::

   <toolchain directory root>/bin/ferrocene-self-test

.. note::

   - ``ferrocene-self-test`` can only be used with the toolchain it is distributed with
   - ``ferrocene-self-test`` does not accept any command line options

All the performed checks will be displayed.
If any check fails, an error will be emitted along with an ID.
You can look up the identifier in :doc:`the error codes list <error-codes>`
to learn more about the failure and ways to fix it.
