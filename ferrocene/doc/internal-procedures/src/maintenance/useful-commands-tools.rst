.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Useful commands, tools, and information
=======================================

This document contains information pertaining to periodic maintenance.

Conflict markers
----------------

In the daily automation pull, the periodic action will create a pull request and use markers to denote any conflicts that may arise when compared to the upstream.

TODO: add example image/screenshot.

You can use the script ``ferrocene/ci/scripts/detect-conflict-markers.py`` to list the markers. This is the same script that the CI runs.

There are special non-standard conflict markers for the files that were deleted.

TODO: add example image/screenshot.