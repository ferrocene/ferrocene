.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Test variants
=============

As described :ref:`in the qualification plan
<qualification-plan:test-variants>`, test variants are a way to run a test suite
multiple times for each target, each time varying some of the configuration.

To enable a variant when running tests, add the ``--test-variant=NAME`` to ``./x
test``. The default variant (when the flag is not passed) depends on the target
you are testing: if you are reproducing a CI failure make sure to specify the
correct variant.

Available variants
------------------

The available variants are defined in
``src/bootstrap/src/ferrocene/test_variants.rs``:

.. This is not the best presentation possible for the test variants (it would be
   prettier to have a table), but it's not worth the time to create a whole new
   extension and pass the content from bootstrap to Sphinx just to render them.

.. literalinclude:: ../../../../src/bootstrap/src/ferrocene/test_variants.rs
   :start-after: INTERNAL_PROCEDURES_START_TEST_VARIANTS
   :end-before: INTERNAL_PROCEDURES_END_TEST_VARIANTS
   :dedent:
