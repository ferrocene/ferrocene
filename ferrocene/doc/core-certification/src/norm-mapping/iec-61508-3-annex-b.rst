.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

IEC 61508-3 Annex B
-------------------

Table B.1 - Design and coding standards
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

.. include:: ../partials/implicit-coding-standard.rst

2
"

The core library does not use heap-allocation.

7
"

Rust does not allow unstructured control flow (i.e. goto statements), except in inline assembly.

8
"

Type conversions in Rust are exclicit (``.into`` or ``as``), except for convenience (e.g. `& &mut T` to `&T`) or method dispatch.

Table B.2 - Dynamic analysis and testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

.. include:: ../partials/boundary-value-analysis.rst

7b
""

Covered by :doc:`core-certification:code-coverage`.

Table B.3 - Functional and black-box testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

4
"

.. include:: ../partials/boundary-value-analysis.rst

Table B.4 - Failure analysis
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

3
"

Covered by :ref:`safety-plan/index:Failure analysis`.


Table B.5 - Modelling
~~~~~~~~~~~~~~~~~~~~~

3
"

N/A

.. include:: ../partials/no-system.rst


Table B.6 - Performance testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

2-3
"""

N/A

.. include:: ../partials/no-system.rst

Table B.7 - Semi-formal methods
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Covered by :ref:`safety-plan/index:requirements management`.

Table B.8 - Static analysis
~~~~~~~~~~~~~~~~~~~~~~~~~~~

3-4
"""

The Ferrocene rustc compiler performs thorough control flow and data flow analysis. The data flow analysis is usually referred to as "Borrow Checker" and is one of the core features of Rust. One example for the outstanding control flow analysis is that rustc detects when some variants of an enum are not handled and throws a hard error.

Table B.9 - Modular approach
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

The core library does not impose a module size limit, but instead structures modules according to their function.

3
"

All fields and methods of a data structure are private by default and must be made public explicitly.

5
"

Functions in Rust can only be called through one interface (i.e. no overloading).

6
"

All items within a module are private by default and must be made public explicitly.
