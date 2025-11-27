.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Licensing
=========

Each file in the Ferrocene repository should have a license and a copyright assignment.

The standard license of Ferrocene matches that of upstream: ``MIT OR Apache-2.0``.

The copyright assignment should be the following: ``The Ferrocene Developers``.

There is a CI check to ensure that all files in the Ferrocene repository have been accounted for.

To perform the check locally, run this command:

.. code-block:: console

   ./x test collect-license-metadata

Adding new files
----------------

If you add a new file to the repository, it will have to pass the above check.
You can use one of 2 ways to ensure that the check is successful:

- Include the filename in REUSE.toml
- Add either of the following headers to the top of the file:

  .. code-block:: text

     // SPDX-License-Identifier: MIT OR Apache-2.0
     // SPDX-FileCopyrightText: The Ferrocene Developers

     OR

     # SPDX-License-Identifier: MIT OR Apache-2.0
     # SPDX-FileCopyrightText: The Ferrocene Developers

     OR (for reStructuredText files)

     .. SPDX-License-Identifier: MIT OR Apache-2.0
        SPDX-FileCopyrightText: The Ferrocene Developers

These examples are for files created by Ferrocene Developers and not derived elsewhere.

Were you to incorporate code from elsewhere, you will need to adjust the text accordingly,
either in REUSE.toml (should you choose to use that apporach) or in the derived files themselves.

An example of the latter:

.. code-block:: text

   # SPDX-License-Identifier: MIT OR Apache-2.0
   # SPDX-FileCopyrightText: The Ferrocene Developers
   # SPDX-FileCopyrightText: The Rust Project Developers (see https://thanks.rust-lang.org)

Updating the license database
-----------------------------

If the licensing check fails, you can update the license database with the following command:

.. code-block:: text

   ./x run collect-license-metadata
