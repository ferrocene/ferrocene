.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing Plan
============

Re-verification scope
---------------------

On every pull request, the full Ferrocene test suite is executed, which includes the tests for the core library. Therefore no impact assessment is necessary, since all tests are run anyways.

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

This test suite is executed on :ref:`all qualified targets of Ferrocene <targets/index:qualified targets>`.

The results of the coretests test suite can be found in the "Library Test Suite" section of :doc:`each targets test results <qualification-report:rustc/index>`.

Doc-tests
~~~~~~~~~

Doc-tests are the code snippets in doc-comments. The doc-tests for the core library are executed as well. Note that doc-tests are not used for code coverage.

Code coverage tests
~~~~~~~~~~~~~~~~~~~

In order to gather code coverage information, an additional test run of the coretests test suite on the x86_64-linux-gnu target is performed.

This run is different to the normal test run, because the ``core`` library and the ``coretests`` binary are instrumented with ``-Cinstrument-coverage``. This inserts llvm intrinsics into the binaries to collect code coverage information.

The collected code coverage information is compiled into a code coverage report using ``grcov``.

It is ensured that both the instrumented and not instrumented coretests run succeeds. This ensures that coverage instrumentation does not introduce any correctness issues.

Manual test coverage
""""""""""""""""""""

If a function cannot be covered through automated tests, this function will be annotated with a comment that states the test cases the function is tested by and the function will get excluded from the code coverage report by annotating it with ``#[coverage(off)]``.

Line vs. function coverage
""""""""""""""""""""""""""

The coverage report does show both "line coverage" and "function coverage".

Line coverage states how many of the lines in source code are executed at least once. This is our primary metric and we aim to achieve 100% line coverage.

Function coverage states how many functions are executed at least once. Note that one function in source code may map to more than one function in the binary. This is due to generics and macros. One generic function in source code will produce one function for each type it is used with. This is a secondary metric, and we do not aim to achieve 100% function coverage.

Tidy test suite
~~~~~~~~~~~~~~~

The tidy test suite ensures conformance with syntax and semantic rules.

rustc lints
~~~~~~~~~~~

During compilation, rustc runs the borrow checker and executes a set of lints to ensure correctness, enforce consistency and prevent common errors.

Integration tests
~~~~~~~~~~~~~~~~~

The core library is heavily used in the other components of the Ferrocene toolchain, e.g. in ``rustc``, the ``std`` library, ``rustdoc`` etc. The integration of the core library is tested by building and testing those components on all the supported platforms.
