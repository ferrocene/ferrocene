.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

``rustfmt`` Command-Line Interface
==================================

.. cli:program:: rustfmt

   .. cli:option:: --check

      Check if the input is formatted correctly.

      Exits with status code 0 if input is formatted correctly.
      Exits with status code 1 and prints a diff if formatting is not correct.

   .. cli:option:: --emit [files|stdout]

      What kind of output to emit.

      - ``files`` emits formatted output to files; this is the default
      - ``stdout`` emits the formatted output to stdout

   .. cli:option:: --backup

      Backup any modified files.

      Before a file is modified, rustfmt will create a copy of it in the same directory,
      with the ``.bk`` extension.
      Note that existing ``.bk`` files are over-written.

   .. cli:option:: --config-path

      Recursively searches the given path for the ``rustfmt.toml`` or ``.rustfmt.toml`` config file.
      If not found, it reverts to the default behavior of searching the parent directories of the
      file to be formatted.

   .. cli:option:: --edition [2015|2018|2021]

      Rust edition to use, which defaults to 2015 edition.

      If using a different edition than the default, specify it here,
      because the formatting may be different.

   .. cli:option:: --color [always|never|auto]

      Use colored output (if supported)

   .. cli:option:: --print-config [Path for the configuration file]

      Dumps a default or minimal config to PATH. A minimal config is the subset of the current config file used for formatting the current program. ``current`` writes to stdout current config as if formatting the file at PATH.

   .. cli:option:: --files-with-diff

      Short option: ``-l``.

      Prints the names of mismatched files that were formatted. Prints the names of files that would be formatted when used with ``--check``.

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

