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

.. note:: ``ferrocene-self-test`` can only be used with the toolchain it is distributed with.
