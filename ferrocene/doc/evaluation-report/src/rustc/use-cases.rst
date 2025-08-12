.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Use Cases
=========

Installing Ferrocene
--------------------

.. id:: RUSTC_UC0_INST

**Actor(s):** User, tar (compression utility)

**Input:** A Ferrocene release tarball.

**Output:** The directory structure and contents of Ferrocene.

**Environment constraints:** tar is correctly installed.

**Description:**

1. The `user` downloads all the archives needed to install Ferrocene.

2. The `user` extracts each archive into the installation directory using `tar`::

    tar -C path/to/installation -xf path/to/archive.tar.xz

Building a Library
------------------

.. id:: RUSTC_UC1_RLIB

**Actor(s):** User, rustc.

**Input:** A Rust compilation unit.

**Output:** A static Rust library.

**Environment constraints:** Ferrocene is correctly installed and the
environment is correctly set.

**Description:**

1. The `user` calls `rustc` with the following command line arguments::

    --edition 2021
    --crate-type rlib
    <path>
    # where <path> is the path to the root of the compilation unit, as a
    # positional argument.

2. `rustc` parses the command line arguments.

3. `rustc` parses the Rust compilation unit.

4. `rustc` analyzes the Rust compilation unit.

5. `rustc` generates LLVM IR for the Rust compilation unit.

6. `rustc` invokes `LLVM`, passing the generated LLVM IR along with
   LLVM-related arguments.

7. `LLVM` generates a static Rust library.

8. (Optional): If the `user` is building a certified library, the `user` must verify that only certified functions from the core library are used.

.. id:: RUSTC_UC2_STATICLIB

**Actor:** User, rustc.

**Input:** A Rust compilation unit.

**Output:** A C-compatible static library.

**Environment constraints:** Ferrocene is correctly installed and the
environment is correctly set.

**Description:**

1. The `user` calls `rustc` with the following command line arguments::

    --edition 2021
    --crate-type staticlib
    <path>
    # where <path> is the path to the root of the compilation unit, as a
    # positional argument.

2. `rustc` parses the command line arguments.

3. `rustc` parses the Rust compilation unit.

4. `rustc` analyzes the Rust compilation unit.

5. `rustc` generates LLVM IR for the Rust compilation unit.

6. `rustc` invokes `LLVM`, passing the generated LLVM IR along with
   LLVM-related arguments.

7. `LLVM` generates a C-compatible static library.

8. (Optional): If the `user` is building a certified library, the `user` must verify that only certified functions from the core library are used.


Building an Executable
----------------------

.. id:: RUSTC_UC3_EXEC

**Actor:** User, rustc.

**Input:** A Rust compilation unit.

**Output:** A Rust executable.

**Environment constraints:** Ferrocene is correctly installed, the
compilation unit has the proper file extension, and the environment is correctly
set.

**Description:**

1. The `user` calls `rustc` with the following command line arguments::

    --codegen-units 1
    --edition 2021
    <path>
    # where <path> is the path to the root of the compilation unit, as a
    # positional argument.

2. `rustc` parses the command line arguments.

3. `rustc` parses the Rust compilation unit.

4. `rustc` analyzes the Rust compilation unit.

5. `rustc` generates LLVM IR for the Rust compilation unit.

6. `rustc` invokes `LLVM`, passing the generated LLVM IR along with
   LLVM-related arguments.

7. `LLVM` generates an object file.

8. `rustc` invokes the linker, passing the generated object file along with
   linker-related arguments.

9. The linker generates a Rust executable.

10. (Optional): If the `user` is building a certified executable, the `user` must verify that only certified functions from the core library are used.

.. id:: RUSTC_UC4_EXEC_RLIB

**Actor:** User, rustc.

**Input:** A Rust compilation unit, a static Rust library.

**Output:** A Rust executable linked to a static Rust library.

**Environment constraints:** Ferrocene is correctly installed, a static
Rust library generated with the same rustc, the compilation unit has the proper
file extension, and the environment is correctly set. If multiple static Rust
libraries are used, then their names must be unique within the set of all
directories included by compiler argument `-L`.

**Description:**

1. (Optional): The `user` performs use case :id:`RUSTC_UC1_RLIB` to generate a static Rust library.

2. The `user` calls `rustc` with the following command line arguments::

    --codegen-units 1
    --edition 2021
    -L <directory>
    --extern <name>
    <path>
    # where <directory> is the path to the directory that contains the static
    # Rust library, <name> is the name of the static Rust library, and <path>
    # is the path to the root of the compilation unit, as a positional argument.

3. `rustc` parses the command line arguments.

4. `rustc` parses the Rust compilation unit.

5. `rustc` analyzes both the Rust compilation unit and the Rust library.

6. `rustc` generates LLVM IR for the Rust compilation unit.

7. `rustc` invokes `LLVM`, passing the generated LLVM IR along with
   LLVM-related arguments.

8. `LLVM` generates an object file.

9. `rustc` invokes the linker, passing the generated object file along with
   linker-related arguments.

10. The linker generates a Rust executable that links to a static Rust library.

11. (Optional): If the `user` is building a certified executable, the `user` must verify that only certified functions from the core library are used.

Building Mixed-Language Programs
--------------------------------

.. id:: RUSTC_UC5_EXEC_CLIB

**Actor:** User, rustc, a C toolchain.

**Input:** A Rust compilation unit, a C library.

**Output:** A Rust executable that links to a C library.

**Environment constraints:** The C and Ferrocene toolchains are installed,
the compilation unit has the proper file extension, and the environment is
correctly set. If multiple C libraries are used, then their names must be
unique within the set of all directories included by compiler argument `-L`.

**Description:**

1. (Optional): The `user` generates a library using a C toolchain.

2. The `user` calls `rustc` with the following command line arguments::

    --codegen-units 1
    --edition 2021
    -L <directory>
    -l <name>
    <path>
    # where <directory> is the path to the directory that contains the C
    # library, <name> is the name of the C library, and <path> is the path to
    # the root of the compilation unit, as a positional argument.

3. `rustc` parses the command line arguments.

4. `rustc` parses the Rust compilation unit.

5. `rustc` analyzes the Rust compilation unit.

6. `rustc` generates LLVM IR for the Rust compilation unit.

7. `rustc` invokes `LLVM`, passing the generated LLVM IR along with
   LLVM-related arguments.

8. `LLVM` generates an object file.

9. `rustc` invokes the linker, passing the generated object file along with
   linker-related arguments.

10. The linker generates a Rust executable that links to a C library.

11. (Optional): If the `user` is building a certified executable, the `user` must verify that only certified functions from the core library are used.

