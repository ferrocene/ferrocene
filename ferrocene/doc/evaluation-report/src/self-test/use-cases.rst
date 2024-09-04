.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Use Cases
=========

.. hazop-use:: Install
   :id: USE_SELF_TEST_INSTALL

   | **Actor(s):** User, tar (compression utility)
   | **Input:** A ferrocene-self-test release tarball.
   | **Output:** The ``ferrocene-self-test`` executable in the ``bin`` directory of
     the Ferrocene installation directory.
   | **Environment constraints:** ``tar`` is correctly installed.

   **Description:**

   1. The user downloads the ferrocene-self-test archive.

   2. The user extracts the archive into the installation directory using `tar`::

       tar -C path/to/installation -xf path/to/ferrocene-self-test-$hash.tar.xz

.. hazop-use:: Check toolchain installtion
   :id: USE_SELF_TEST_CHECK

   | **Actor(s):** User, ferrocene-self-test.
   | **Input:**
   | - The PATH environment variable
   | - The files in the installation directory
   | **Output:** Text on the console/terminal.
   | **Environment constraints:**
   | - ferrocene-self-test is installed.
   | - Ferrocene toolchain is installed
   | - There is a compatible and C compiler in PATH

   **Description:**

   The `user` calls `ferrocene-self-test` without any command line arguments
