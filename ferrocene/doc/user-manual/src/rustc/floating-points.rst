.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Floating Points
===============

This chapter describes floating points in Rust and ways to improve the safety of their use.

The following Rust types map to `IEEE 754-2008 <https://ieeexplore.ieee.org/document/4610935>`_
types:

* ``f32`` maps to IEEE 754-2008's ``binary32``
* ``f64`` maps to IEEE 754-2008's ``binary64``
* ``f128`` maps to IEEE 754-2008's ``binary128``

Disallowing Floating Point Arithmetic
-------------------------------------

In some contexts, such as constrained embedded scenarios, it's desirable to disallow floating point
operations entirely.

Clippy provides a lint ``float_arithmetic`` that can be used to ensure code does not perform
floating point operations. The below example produces an error when ``cargo clippy`` is run:

.. code-block::

    #![deny(clippy::float_arithmetic)]

    fn main() {
        let x: f64 = 0.0;
        let y: f64 = 1.0;
        println!("{}", x + y);
    }

When using ``clippy`` lints, users should include running ``cargo clippy`` in their testing
processes.

Use More Precise Expressions
----------------------------

In safety critical contexts, precision is typically preferred over performance.

.. note::

    The lints mentioned in this section are in the Clippy 'nursery' which contains lints
    that are still in development, and may be incomplete or have false-positives.

    If a lint is determined to be a false positive, it can be permitted on a case by case basis
    with ``#[allow(clippy::the_lint)``, where ``the_lint`` is the lint in question.

Clippy provides a lint ``imprecise_flops`` that can be used to suggest more accurate floating
point operations. The below example produces errors with suggested remedies when ``cargo clippy``
is run:

.. code-block::

    #![deny(clippy::imprecise_flops)]

    fn main() {
        let a = 3f32;
        let _ = a.powf(1.0 / 3.0);
        let _ = (1.0 + a).ln();
        let _ = a.exp() - 1.0;
    }

Clippy also provides the ``suboptimal_flops`` lint which can suggest more optimal floating point
operations, both in accuracy and performance. The below example produces errors with suggested
remedies when ``cargo clippy`` is run:

.. code-block::

    #![deny(clippy::suboptimal_flops)]
    use std::f32::consts::E;

    fn main() {
        let a = 3f32;
        let _ = (2f32).powf(a);
        let _ = E.powf(a);
        let _ = a.powf(1.0 / 2.0);
        let _ = a.log(2.0);
        let _ = a.log(10.0);
        let _ = a.log(E);
        let _ = a.powf(2.0);
        let _ = a * 2.0 + 4.0;
        let _ = if a < 0.0 {
            -a
        } else {
            a
        };
        let _ = if a < 0.0 {
            a
        } else {
            -a
        };
    }

When using ``clippy`` lints, users should include running ``cargo clippy`` in their testing
processes.

Do Not Declare Lossy Literals
-----------------------------

It is generally not intended to declare literal floats that cannot be represented by the type.

Clippy provides a lint ``lossy_float_literal`` that can be used to suggest code more accurate
floating point operations. The below example produces an error when ``cargo clippy`` is run:

.. code-block::

    #![deny(clippy::lossy_float_literal)]

    fn main() {
        let _: f32 = 16_777_217.0; // Becomes 16_777_216.0
    }


When using ``clippy`` lints, users should include running ``cargo clippy`` in their testing
processes.


Be Extra Mindful of CPU/FPU Errata
----------------------------------

Floating point operations are a common source of CPU/FPU errata. Users should invest extra
diligence into monitoring their relevant chipset and hardware errata when using floating
point operations.
