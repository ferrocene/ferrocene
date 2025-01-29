.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Using
=====

``rustfmt`` formats Rust code or checks if Rust code is well-formatted.

Code is "well formatted" if it complies with the :ref:`Configuration`.

``rustfmt <file>...``
---------------------

To format one or more files, just pass the paths to ``rustfmt``. This will
format the files and all of their child modules.

``rustfmt --check <file>...``
-----------------------------

If the ``--check`` option is used, ``rusfmt`` checks if the files are
well-formatted. It reports what it would format, but will not modify any files.
For example:

.. code-block::

   $ rustfmt --check src/main.rs 
   Diff in /krate/src/main.rs at line 1:
    fn main() {
   -    println!(
   -        "Hello, world!"
   -    );
   +    println!("Hello, world!");
    }

Exit codes
----------

When running with ``--check``, ``rustfmt`` will exit with status code ``0`` if
``rustfmt`` would not make any formatting changes to the input, and status code
``1`` if ``rustfmt`` would make changes. In other modes, ``rustfmt`` will exit
with ``1`` if there was some error during formatting (for example a parsing or
internal error) and ``0`` if formatting completed without error (whether or not
changes were made).

.. _Configuration:

Configuration
-------------

By default ``rustfmt`` formats code according to the
`Rust Style Guide <../../style-guide/index.html>`_. The formatting can be
customized with :doc:`the various config options</rustfmt/config>`. They can be
set in a config file (``rustfmt.toml`` or ``.rustfmt.toml``) or passed via the
command-line (``--config``).

``cargo fmt``
-------------

.. caution::

   Use of ``cargo fmt`` is not qualified, and thus can't be used in a safety
   critical environment: in that case, you must invoke ``rustfmt`` directly.

``rustfmt`` can be used "through" ``cargo`` to improve the user experience.
``cargo fmt`` will format all the files in a cargo project.
``cargo fmt --check`` will check that all files in a cargo project are
well-formatted.
