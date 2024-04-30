.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Using ``rustfmt``
=================

``rustfmt`` formats Rust code or checks if Rust code is well formatted.

``rustfmt <file>...``
---------------------

To format one or more files, just pass the paths to ``rusfmt``. This will format the passed files and all of their child modules.

Assuming following project structure, ``rustfmt main.rs`` will format the whole project, while ``rustfmt mod1.rs`` will format ``mod1.rs``, ``mod1/mod10.rs`` and ``mod1/mod11.rs``, but neither ``mod2.rs`` nor ``mod2/mod20.rs``.

.. code-block::

   $ tree src/
   src/
   ├── main.rs
   ├── mod1
   │   ├── mod10.rs
   │   └── mod11.rs
   ├── mod1.rs
   ├── mod2
   │   └── mod20.rs
   └── mod2.rs


``rustfmt --check <file>...``
-----------------------------

If the ``--check`` option is passed ``rusfmt`` checks if the files are well formatted. It reports what it would format, but will not modify any files. For example:

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

When running with ``--check``, ``rustfmt`` will exit with ``0`` if ``rustfmt`` would not make any formatting changes to the input, and ``1`` if ``rustfmt`` would make changes. In other modes, ``rustfmt`` will exit with ``1`` if there was some error during formatting (for example a parsing or internal error) and ``0`` if formatting completed without error (whether or not changes were made).

``cargo fmt``
-------------

.. caution::

   Use of ``cargo fmt`` is not qualified, and thus can't be used in a safety critical environment: in that case, you must invoke ``rustfmt`` directly.

``rustfmt`` can be use "through" ``cargo`` to improve the user experience. ``cargo fmt`` will format all the files in a cargo project. ``cargo fmt --check`` will check that all files in a cargo project are well formatted.
