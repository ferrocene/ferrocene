.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

System Requirements
===================

Even though any machine can run the Ferrocene toolchain, in order to get the
best experience, we recommend using a machine with many cores, since parts of
the compilation process are internally parallelized. The benefit of more cores
will vary depending on the structure of the project being compiled.

A comfortable setup for a compiler server is a machine with 8 physical cores or
more, with at least 16 GB of memory (2 GB per core). For a desktop machine, a
minimum of 2 cores is recommended (8 preferred), with at least 2GB per core (so
4 to 16GB).

Note that using local and fast drives will also make a difference in terms of
build and link time. Network drives should be avoided as much as possible and
will result in degraded performance.
