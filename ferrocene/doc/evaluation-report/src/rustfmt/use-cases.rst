.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Use Cases
=========

Installing rustfmt
------------------

.. id:: RUSTFMT_UC0_INST

**Actor(s):** User, tar (compression utility)

**Input:** A rustfmt release tarball.

**Output:** The rustfmt binary in the Ferrocene directory.

**Environment constraints:** tar is correctly installed.

**Description:**

1. The `user` downloads the rustfmt archive.

2. The `user` extracts the archive into the installation directory using `tar`::

    tar -C path/to/installation -xf path/to/archive.tar.xz

Format a file
-------------

.. id:: RUSTFMT_UC1_FMT

**Actor(s):** User, rustfmt.

**Input:** One or multiple Rust source files.

**Output:** Rust source files with the same paths, potentially modified.

**Environment constraints:** rustfmt is installed.

**Description:**

1. The `user` calls `rustfmt` with the following command line arguments::

    <path>...
    # where <path> is the path to a Rust source file.

2. `rustfmt` modifies Rust source files if they are not well-formatted.

Check formatting of a file
--------------------------

.. id:: RUSTFMT_UC2_CHECK

**Actor(s):** User, rustfmt.

**Input:** One or multiple Rust source files.

**Output:**

- A terminal exit code.
- Diff between current and well-formatted code. Omitted if code is well-formatted.

**Environment constraints:** rustfmt is installed.

**Description:**

1. The `user` calls `rustfmt` with the following command line arguments::

    --check
    <path>...
    # where <path> is the path to a Rust source file.

2. `rustfmt` outputs a terminal status code, indicating if the checked files
   are well-formatted.
