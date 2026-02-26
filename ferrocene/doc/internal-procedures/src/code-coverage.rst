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

Gathering standard library coverage
-----------------------------------

To gather the standard library coverage, we recommend this command:

.. code-block::

   ./x test --tests --coverage=library library/core

.. note::

   Right now, only the ``core`` crate's coverage will be displayed in the
   report, as the other crates are ignored in
   ``src/bootstrap/src/ferrocene/code_coverage.rs``.

.. note::

   Doctests are not supported when gathering the coverage of the standard
   library, and the build system will enforce the presence of ``--tests``.
