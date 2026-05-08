.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Overview
========

Bare metal Rust programs, such as those using the ``cortex-m-rt`` (v0.6.x) crate on
the :ref:`thumbv7em-none-eabi` or :ref:`thumbv7em-none-eabihf` targets, may not necessarily
be memory safe in the presence of stack overflows.

The following program, which contains no unsafe code block, can run into undefined behavior if
it reaches a stack overflow condition:

.. code-block::

    // static variables placed in the .bss / .data sections
    static FLAG1: AtomicBool = AtomicBool::new(false); // .bss
    static FLAG2: AtomicBool = AtomicBool::new(true);  // .data

    fn main() {
        let _x = fib(100);
    }

    #[inline(never)]
    fn fib(n: u32) -> u32 {
        // allocate and initialize 4 kilobytes of stack memory
        let _use_stack = [0xAA; 1024];

        if n < 2 {
            1
        } else {
            fib(n - 1) + fib(n - 2) // recursion
        }
    }

    #[interrupt]
    fn interrupt_handler() {
        // does some operation with `FLAG1` and `FLAG2`
    }

This is due to the default memory layout of ARM Cortex-M programs in RAM, shown below:

.. figure:: ../../../../tools/flip-link/assets/overflow.svg

    The default memory layout of ARM Cortex-M programs in RAM.

The function call stack, also known as the "stack", grows downwards on function calls and when
local variables (e.g. ``let x``) are created (these variables are also placed on the stack).

If the stack grows too large it collides with the ``.bss+.data`` region, which contains all the
program's static variables. The collision results in the static variables being overwritten
with unrelated data. This can result in the program observing the static variables in an invalid
state: for example an ``AtomicBool`` may hold the value ``3`` -- this is undefined behavior because
the Rust ABI expects this single-byte variable to be either ``0`` or ``1``.

One potential solution is to change the memory layout of the program and place the stack below the
``.bss+.data`` region.

With this flipped memory layout (pictured below) the stack cannot collide with the static
variables. Instead it will collide with the boundary of the physical RAM memory region. In the ARM
Cortex-M architecture, trying to read or write past the boundaries of the RAM region produces a
"hardware exception". The ``cortex-m-rt`` crate provides an API to handle this condition: a
``HardFault`` exception handler can be defined; this "handler" (function) will be executed when the
invalid  memory operation is attempted.

``flip-link`` implements this stack overflow solution. Linking your program with ``flip-link``
produces the flipped memory layout, which is memory safe in presence of stack overflows.

.. figure:: ../../../../tools/flip-link/assets/flipped.svg

    The memory layout of ARM Cortex-M programs in RAM after ``flip-link``.
