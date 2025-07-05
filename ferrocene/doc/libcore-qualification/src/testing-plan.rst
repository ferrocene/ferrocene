.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing Plan
============

Re-verification scope
---------------------

On every pull request, the full Ferrocene test suite, which includes the tests for libcore, is executed. Therefore no impact assessment is necessary, since all tests are run anyways.

Test workflow
-------------

- Before running the tests, the changes are reviewed by an independent Ferrocene reviewer.
- The tests are executed by the CI system, which also ensures that all tests succeed.
- In case of a test failure, the change author will review the CI logs and fix the problem. The change will be reviewed again and the full test suite is executed again.

Test suits
----------

Coretests
~~~~~~~~~

Libcore is primarily tested by the coretests test suite. It defines the test case and pass/fail criteria.

Although only a subset of libcore is certified, also tests for uncertified functions are executed.

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

This test suite is executed on all qualified targets of Ferrocene, see `the user manual <https://public-docs.ferrocene.dev/main/user-manual/targets/index.html#qualified-targets>`.

The results of the coretests test suite can be found in the "Library Test Suite" section of :doc:`each targets test results <qualification-report:rustc/index>`.

Doc-tests
~~~~~~~~~

Doc-tests are the code snippets in doc-comments. The doc-tests for libcore are executed as well. Note that doc-tests are not used for code coverage.

Code coverage tests
~~~~~~~~~~~~~~~~~~~

In order to gather code coverage information, an additional test run of the coretests test suite on the x86_64-linux-gnu target is performed.

This run is different to the normal test run, because
   - The ``libcore`` library and the ``coretests`` binary are instrumented with ``-Cinstrument-coverage``. This inserts llvm instrinsics into the binaries to collect code coverage information.
   - Inlining is disabled by modifying source code and inserting ``#[inline(never)]`` attributes

The collected code coverage information is compiled into a code coverage report using ``grcov``.

It is ensured that both the instrumented and not instrumented coretests run succeeds. This ensures that coverage instrumentation and disabling of inlining did not introduce any correctness issues.

Manual test coverage
""""""""""""""""""""

The code coverage tooling cannot handle some compiler optimizations like inlining or const functions. This results in some functions not being shown as covered although they are tested. Therefore those functions are annotated with a comment that states the test cases the function is tested by and gets excluded from the code coverage report by annotating the function with ``#[coverage(off)]``.

Line vs. function coverage
""""""""""""""""""""""""""


Note that code coverage of functions measures multiple instances of generic functions or macro generated functions, and so can have unexpected function to line ratio. See the three line function in <https://public-docs.ferrocene.dev/main/coverage/library-x86_64-unknown-linux-gnu/library/core/src/alloc/layout.rs.html#19> for an example. This is normal in rust.

Tidy test suite
~~~~~~~~~~~~~~~

The tidy test suite is executed as well. It ensure conformance with syntax and semantic rules.
