.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers
   
IEC 61508-3 Annex A
-------------------

Table A.1
~~~~~~~~~

1a
""

Covered by :ref:`safety-plan:Requirements Management`.

Table A.2
~~~~~~~~~

7
"

.. include:: ../partials/simple-design.rst

8
"

N/A; the core library does not use external software elements.

11a
"""

N/A

.. include:: ../partials/simple-design.rst

13a
"""

N/A; core is a library.

Table A.3
~~~~~~~~~

1-2
"""

Rust has strong typing and assertions, is memory safe, and is well suited to structured and defensive programming. It is fully defined by the Ferrocene language specification and is a widely used general purpose programming language.

4a
""

The certified core library uses Ferrocene, the fully qualified Rust compiler according to IEC 61508.

Table A.4
~~~~~~~~~

1a
""

N/A

.. include:: ../partials/simple-design.rst

4
"

The core library is highly modularized.

5
"

N/A

.. include:: ../partials/implicit-coding-standard.rst

As such, the certified core library does not have a coding standard.

6
"

The Rust programming language encourages structured programming. It has support for modular designs and does not support goto jumps. Rust has complex return types and therefore the use of out parameters is not common.

7
"

N/A; the core library does not use external software elements.

Table A.5
~~~~~~~~~

2
"

N/A; core is a library.

3
"

.. include:: ../partials/core-testing.rst

4
"

.. include:: ../partials/core-testing.rst

8
"

Tests are managed and automated by the libtest tool. It compiles a test runner binary which executes all tests and collects and visualises all test results. Coretests is run by CI for every PR.

Table A.6
~~~~~~~~~

N/A; No electronics or other hardware.

Table A.7
~~~~~~~~~

4
"

.. include:: ../partials/core-testing.rst

Table A.8
~~~~~~~~~

1-3, 4a
"""""""

The complete toolchain is reverified for every change. For every change, all tests are run, and all release artifacts are built.

5
"

Covered by :doc:`qualification-plan:ci`.

6
"

Covered by :doc:`qualification-plan:development`.

Table A.9
~~~~~~~~~

3
"

The Ferrocene compiler performs various kinds of static analysis and lints when compiling the core library.

4
"

.. include:: ../partials/core-testing.rst

Table A.10
~~~~~~~~~~

3
"

Covered by :doc:`evaluation-report:rustc/tool-analysis`.
