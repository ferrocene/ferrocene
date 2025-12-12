.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing Plan
============

Re-verification scope
---------------------

Before a pull request is merged, the full Ferrocene test suite is executed,
which includes the tests for the core library.
In addition, all release artifacts are built. As a result, no impact assessment is necessary.

Test workflow
-------------

- Before running the tests, the changes are reviewed by an independent Ferrocene reviewer.
- The tests are executed by the CI system, which also ensures that all tests succeed.
- In case of a test failure, the change author will review the CI logs and fix the problem. The change will be reviewed again and the full test suite is executed again.

Test suites
-----------

Coretests
~~~~~~~~~

The core library is primarily tested by the coretests test suite. It defines the test case and pass/fail criteria.

Although only a subset of the core library is certified, also tests for uncertified functions are executed.

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

This test suite is executed on :ref:`all certified targets of Ferrocene <certified-core-targets>`.

The results of the coretests test suite can be found in the "Library Test Suite" section of :doc:`each targets test results <qualification-report:rustc/index>`.

Doc-tests
~~~~~~~~~

Doc-tests are the code snippets in doc-comments. The doc-tests for the core library are executed as well. Note that doc-tests are not used for code coverage.

Code coverage tests
~~~~~~~~~~~~~~~~~~~

In order to gather code coverage information, an additional test run of the coretests test suite on the ``x86_64-ferrocene-linux-gnu`` target is performed.

It is ensured that both the instrumented and not instrumented coretests run succeeds. This ensures that coverage instrumentation does not introduce any correctness issues.

Code coverage is measured only on one platform, ``x86_64-ferrocene-linux-gnu``. This is sufficient because the code of the core library is largely platform independent, and code coverage is only a measure of the quality of the test suite. Correctness is still tested by running the tests on all qualified targets.

How it works:

1. ``rustc`` is instructed to instrument the binary by passing ``-Cinstrument-coverage``.
2. The ``coretests`` test suite is executed. Due to the instrumentation, this will create ``.profraw`` files that contain the coverage information.
3. ``symbol-report`` is used to generate a ``symbol-report.json`` including all symbols in the certified subset and their spans.
4. ``blanket`` is used to generate a HTML coverage report from the ``.profraw`` files, the ``symbol-report.json``, and the instrumented binaries.

Manual test coverage
""""""""""""""""""""

If a line cannot be covered by automated tests it will be marked with a ``#[ferrocene::annotation(REASON)]`` attribute stating the reason.
This annotation will be displayed in the generated HTML report.

Tidy test suite
~~~~~~~~~~~~~~~

The tidy test suite ensures conformance with syntax and semantic rules.

rustc lints
~~~~~~~~~~~

During compilation, rustc runs the borrow checker and executes a set of lints to ensure correctness, enforce consistency and prevent common errors.

Integration tests
~~~~~~~~~~~~~~~~~

The core library is heavily used in the other components of the Ferrocene toolchain, e.g. in ``rustc``, the ``std`` library, ``rustdoc`` etc. The integration of the core library is tested by building and testing those components on all the supported platforms.
