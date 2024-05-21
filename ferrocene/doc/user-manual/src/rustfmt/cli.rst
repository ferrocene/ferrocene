.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

``rustfmt`` Command-Line Interface
==================================

.. cli:program:: rustfmt

   .. cli:option:: --check

      Run in 'check' mode. Exits with 0 if input is formatted correctly. Exits with 1 and prints a diff if formatting is required.

   .. cli:option:: --emit [files|stdout|coverage|checkstyle|json]

      What data to emit and how

   .. cli:option:: --backup

      Backup any modified files.

   .. cli:option:: --config-path

      Recursively searches the given path for the rustfmt.toml config file. If not found reverts to the input file path

   .. cli:option:: --edition [2015|2018|2021]

      Rust edition to use

   .. cli:option:: --color [always|never|auto]

      Use colored output (if supported)

   .. cli:option:: --print-config [Path for the configuration file]

      Dumps a default or minimal config to PATH. A minimal config is the subset of the current config file used for formatting the current program. ``current`` writes to stdout current config as if formatting the file at PATH.

   .. cli:option:: --files-with-diff

      Short option: ``-l``.

      Prints the names of mismatched files that were formatted. Prints the names of files that would be formatted when used with ``--check`` mode.

   .. cli:option:: --config [key1=val1,key2=val2...]

      Set options from command line. These settings take priority over .rustfmt.toml

   .. cli:option:: --verbose

      Short option: ``-v``.

      Print verbose output

   .. cli:option:: --quiet

      Short option: ``-q``.

      Print less output

   .. cli:option:: --version

      Short option: ``-V``.

      Show version information

   .. cli:option:: --help [=TOPIC]

      Short option: ``-h``.

      Show this message or help about a specific topic: ``config`` or ``file-lines``
   