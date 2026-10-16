.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing Plan
============

Re-verification scope
---------------------

Before a pull request is merged, the full Ferrocene test suite is executed,
which includes the tests for the all certified libraries.
In addition, all release artifacts are built. As a result, no impact assessment is necessary.

Test workflow
-------------

- Before running the tests, the changes are reviewed by an independent Ferrocene reviewer.
- The tests are executed by the CI system, which also ensures that all tests succeed.
- In case of a test failure, the change author will review the CI logs and fix the problem. The change will be reviewed again and the full test suite is executed again.

Test suites
-----------

Unit tests
~~~~~~~~~~

The certified libraries are primarily tested by following unit test suites:

.. list-table::
   :align: left
   :header-rows: 1

   * - Library
     - Unit test suite
   * - ``core``
     - ``coretests``
   * - ``alloc``
     - ``alloctests``

The test suites define the test cases and pass/fail criteria.

Although only a subset of the libraries are certified, also tests for uncertified functions are executed.

One example test is ``coretests::slice::test_position``. It calls ``.position`` 4 times with different arguments and asserts that the results are correct:

.. code-block:: rust
  :linenos:

  #[test]
  fn test_position() {
      let b = [1, 2, 3, 5, 5];
      assert_eq!(b.iter().position(|&v| v == 9), None);
      assert_eq!(b.iter().position(|&v| v == 5), Some(3));
      assert_eq!(b.iter().position(|&v| v == 3), Some(2));
      assert_eq!(b.iter().position(|&v| v == 0), None);
  }

This test suites are executed on :doc:`all qualified targets of Ferrocene <user-manual:targets/index>`.

The results of the test suites can be found in the "Library Test Suite" section of :doc:`each targets test results <qualification-report:rustc/index>`.

Doc-tests
~~~~~~~~~

Doc-tests are the code snippets in doc-comments. The doc-tests for the certified libraries are executed as well. Note that doc-tests are not used for code coverage.

Code coverage tests
~~~~~~~~~~~~~~~~~~~

In order to gather code coverage information, an additional test run of the unit test suites is performed for each qualified target.

It is ensured that both the instrumented and not instrumented unit test suite runs succeed. This ensures that coverage instrumentation does not introduce any correctness issues.

How it works:

1. ``rustc`` is instructed to instrument the unit test binary by passing ``-Cinstrument-coverage``.
2. The unit test suites are executed. Due to the instrumentation, this will create ``.profraw`` files that contain the coverage information.
3. ``symbol-report`` is used to generate a ``symbol-report.json`` including all symbols in the certified subset and their spans.
4. ``blanket`` is used to generate a HTML coverage report from the ``.profraw`` files, the ``symbol-report.json``, and the instrumented binaries.

Manual test coverage
""""""""""""""""""""

If a line cannot be covered by automated tests it will be marked with a ``#[ferrocene::annotation(REASON)]`` attribute stating the reason.
This annotation will be displayed in the generated HTML report.

QNX
"""

Due to a bug in the QNX C toolchain, gathering code coverage on QNX 7.1 and QNX 8 targets is not possible:

.. list-table::
   :align: left
   :header-rows: 1

   * -
     - QNX 7.1
     - QNX 8
     - Coverage on
   * - **x86**
     - ``x86_64-pc-nto-qnx710``
     - ``x86_64-pc-qnx``
     - ``x86_64-unknown-linux-gnu``
   * - **aarch64**
     - ``aarch64-unknown-nto-qnx710``
     - ``aarch64-unknown-qnx``
     - ``aarch64-unknown-none``

Instead coverage is measured on the target specified in the "Coverage on" column. Those targets are sufficiently similar to ensure sufficient coverage on the QNX targets as well.

Tidy test suite
~~~~~~~~~~~~~~~

The tidy test suite ensures conformance with syntax and semantic rules.

rustc lints
~~~~~~~~~~~

During compilation, rustc runs the borrow checker and executes a set of lints to ensure correctness, enforce consistency and prevent common errors.

Integration tests
~~~~~~~~~~~~~~~~~

The certified libraries are heavily used in the other components of the Ferrocene toolchain, e.g. in ``rustc``, ``cargo``, ``rustdoc`` etc. The integration of the certified libraries is tested by building and testing those components on all the supported platforms.
