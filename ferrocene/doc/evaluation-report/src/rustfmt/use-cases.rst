.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Use Cases
=========

.. hazop-use:: Installing rustfmt
   :id: USE_RUSTFMT_INSTALL

   | **Actor(s):** User, tar (compression utility)
   | **Input:** A rustfmt release tarball.
   | **Output:** The rustfmt binary in the Ferrocene directory.
   | **Environment constraints:** tar is correctly installed.

   **Description:**

   1. The `user` downloads the rustfmt archive.

   2. The `user` extracts the archive into the installation directory using `tar`::

       tar -C path/to/installation -xf path/to/archive.tar.xz

.. hazop-use:: Format a file
   :id: USE_RUSTFMT_FORMAT

   | **Actor(s):** User, rustfmt.
   | **Input:** One or multiple Rust source files.
   | **Output:** Rust source files with the same paths, potentially modified.
   | **Environment constraints:** rustfmt is installed.

   **Description:**

   1. The `user` calls `rustfmt` with the following command line arguments::

       <path>...
       # where <path> is the path to a Rust source file.

   2. `rustfmt` modifies Rust source files if they are not well-formatted.


.. hazop-use:: Check formatting of a file
   :id: USE_RUSTFMT_CHECK

   | **Actor(s):** User, rustfmt.
   | **Input:** One or multiple Rust source files.
   | **Output:** A terminal exit code.
   | **Output:** A diff between current and well-formatted code (omitted if code is well-formatted).
   | **Environment constraints:** rustfmt is installed.

   **Description:**

   1. The `user` calls `rustfmt` with the following command line arguments::

       --check
       <path>...
       # where <path> is the path to a Rust source file.

   2. `rustfmt` outputs a terminal status code, indicating if the checked files
      are well-formatted.
