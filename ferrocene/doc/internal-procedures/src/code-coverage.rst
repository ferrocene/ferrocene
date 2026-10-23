.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Code coverage
=============

The build system allows measuring code coverage with ``./x test --coverage``
when running tests.

The code coverage feature is designed to instrument a single component of
Ferrocene (for example, the standard library) and gather code coverage metrics
from multiple test suites, even if they are not strictly related to the
component whose coverage is being measured.

Because of this, the ``--coverage`` flag requires the name of the component to
measure as the flag value. If you need to gather coverage metrics for multiple
components, you will need to make multiple invocations of ``./x test``. The
following components are currently instrumented:

.. list-table::
   :header-rows: 1

   * - Flag
     - Measured components

   * - ``--coverage=library``
     - Standard library crates ``core``, ``alloc``, ``test``

When the flag is passed, the component to measure will be instrumented, and all
test suites will gather coverage metrics. Finally, an HTML report will be
generated, and its URL will appear at the end.

The specific crate to generate coverage for can be specified with ``--coverage-crate``, which can be passed multiple times and defaults to ``core``.

Gathering standard library coverage
-----------------------------------

To gather the standard library coverage, we recommend this command:

.. code-block::

   ./x test \
       library/core library/alloc \
       --tests \
       --coverage=library \
       --coverage-crate=core \
       --coverage-crate=alloc \
       --coverage-crate=std
.. note::

   Doctests are currently not supported when gathering the coverage of the
   standard library. This limitation will be lifted as soon as doc tests can be
   merged into one binary.
