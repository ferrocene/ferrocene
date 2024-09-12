.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Qualification Method
====================

According to the Tool Confidence Level determined previously and the
recommendation of Table 4 of [|iso_ref|] part 8,
the chosen qualification method is "Validation of the software tool in accordance with 11.4.9".
To ensure the absence of potential issues in ``ferrocene-self-test``,
an automated test suite is executed on :target:`x86_64-unknown-linux-gnu`.
In particular, the following test suites are executed before any change is merged:

- Unit tests

  These test internal functionality of the tool.

- End-to-end tests

  These test the tool itself by executing it directly.
