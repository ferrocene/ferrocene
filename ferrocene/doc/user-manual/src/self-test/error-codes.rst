.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Ferrocene Self-test Error Codes
===============================

FST_001: No sysroot
-------------------

This error occurs when the ``ferrocene-self-test`` binary has not been
installed in the ``bin`` folder within the Ferrocene toolchain installation
directory, aka "the sysroot".

**Suggested fixes**

If you have moved the ``ferrocene-self-test`` binary out of the sysroot, move
it back into that directory.

Alternatively, reinstall the Ferrocene toolchain.

FST_002: Missing binary
-----------------------

This error occurs when one of the Ferrocene toolchain binaries has not been
installed.

**Suggested fixes**

Ensure that all packages of the Ferrocene toolchain have been properly
installed.

FST_003: Wrong binary permissions
---------------------------------

This error occurs when a Ferrocene toolchain binary lacks execution
permissions.

**Suggested fixes**

Grant execution permission to the indicated Ferrocene toolchain binary.

FST_004: Metadata fetch failed
------------------------------

This error occurs when a Ferrocene toolchain binary is not accessible.

**Suggested fixes**

Ensure that file ownership and system permissions are correctly set.

FST_005: Version fetch failed
-----------------------------

This error occurs when the invocation of the Ferrocene compiler failed.

**Suggested fixes**

Reinstall the Ferrocene toolchain.

FST_006: Version parse failed
-----------------------------

This error occurs when the versions of the Ferrocene compiler cannot be
parsed.

**Suggested fixes**

Ensure that another binary with the same name has not been installed in the
sysroot.

FST_007: Binary version mismatch
--------------------------------

This error occurs when the version of the ``ferrocene-self-test`` binary and
that of the Ferrocene compiler are not the same.

**Suggested fixes**

Do not use the same ``ferrocene-self-test`` binary with different Ferrocene
toolchains.

Reinstall the Ferrocene toolchain in order to obtain the expected
``ferrocene-self-test`` binary or Ferrocene compiler.

FST_008: Missing target library
-------------------------------

This error occurs when the installation of the Ferrocene toolchain for a
particular target is malformed.

**Suggested fixes**

Reinstall the Ferrocene toolchain.

FST_009: Duplicate target library
---------------------------------

This error occurs when two versions of the Ferrocene toolchain were
installed on top of each other.

**Suggested fixes**

Ensure that the sysroots of both installations are distinct.

FST_010: Target library discovery failed
----------------------------------------

This error occurs when a Ferrocene toolchain library is not accessible.

**Suggested fixes**

Ensure that file ownership and system permissions are correctly set.

FST_015: Bundled linker missing
-------------------------------

This error occurs when the Ferrocene toolchain linker is not available.

**Suggested fixes**

Ensure that the Ferrocene toolchain has been properly installed.

FLS_016: Path not in UTF-8
--------------------------

This error occurs when the Ferrocene self-test tool attempts to access a
path that is not in UTF-8.

**Suggested fixes**

Ensure that sysroot of the Ferrocene toolchain is a valid UTF-8 path.

FLS_017: Creation of temporary directory failed
-----------------------------------------------

This error occurs when the Ferrocene self-test tool cannot create a
temporary directory where it will compile and run sample test programs.

**Suggested fixes**

Ensure that file ownership and system permissions are correctly set.

FLS_018: Test program cannot be created
---------------------------------------

This error occurs when the Ferrocene self-test tool cannot create the
source file of a test program.

**Suggested fixes**

Ensure that file ownership and system permissions are correctly set.

Ensure that there is enough free disk space.

FLS_019: Test program cannot be compiled
----------------------------------------

This error occurs when the Ferrocene self-test tool cannot compile a test
program.

**Suggested fixes**

Ensure that the operating system is supported by the Ferrocene toolchain.

Ensure that the prerequisites of the Ferrocene toolchain has been properly
installed.

Ensure that the Ferrocene toolchain has been properly installed.

FLS_020: Compilation artifact cannot be read
--------------------------------------------

This error occurs when the Ferrocene self-test tool cannot read an
artifact produced by compiling a test program.

**Suggested fixes**

Ensure that file ownership and system permissions are correctly set.

FLS_021: Missing compilation artifact
-------------------------------------

This error occurs when the Ferrocene self-test tool cannot find an expected
artifact produced by compiling a test program.

**Suggested fixes**

Ensure that the Ferrocene sysroot is not tampered with while the self-test
tool is running.

Ensure that the Ferrocene toolchain has been properly installed.

FLS_022: Unexpected compilation artifact
----------------------------------------

This error occurs when the Ferrocene self-test tool finds an additional
unexpected artifact produced by compiling a test program.

**Suggested fixes**

Ensure that the Ferrocene sysroot is not tampered with while the self-test
tool is running.

Ensure that the Ferrocene toolchain has been properly installed.

FLS_023: Suitable C Compiler not found
--------------------------------------

This error occurs when the Ferrocene self-test tool is unable to find a C
compiler which meets the requirements for a specific target.

**Suggested fixes**

Install a suitable C compiler for that target, such as GCC or clang.

FLS_024: Linker Arguments error
-------------------------------

This error occurs when the Ferrocene self-test tool is unable to find a C
compiler which emits only valid linker arguments to the linker.

**Suggested fixes**

Install a suitable C compiler for that target, such as GCC or clang.
