.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Performing system calls
=======================

This chapter describes how to perform system calls using
`inline assembly <../../specification/inline-assembly.html>`_.

A system call is the programmatic way for a user program to request a service
from the operating system (OS). The mechanism for performing a system call is
defined by the OS.

The two most common approaches of performing a system call are using a C
function or using assembly instructions.

To check which approach applies to your program, consult your OS manual. Some
OSes provide a C API as the only stable way to perform system calls.

To perform a system call through a C library, refer to
`Building mixed-language programs <mixed-language.html>`_.

We are going to perform the ``WRITE`` system call on a 64-bit ARM machine
running Linux. On these targets, system calls are performed using the SVC
(SuperVisor Call) instruction.

The arguments to a system call are passed via registers, where the OS defines
which registers to use and what they mean to the system call. In the case of
64-bit ARM Linux, the system call number is passed in register ``X8``. The
first argument to the system call is passed in register ``X0``, the second in
``X1``, etc. The return value of the system call is written to register ``X0``.

In the case of a ``WRITE`` system call, the system call number is ``64``, the
first argument is the file descriptor to write to, the second argument is the
pointer to the data that will be written, the third argument is the size of the
input data, and the return value of the system call is the number of bytes that
were written, or an error if the value is negative.

The execution of a system call can modify registers other than the return value
register as a side effect. These registers must be "clobbered" in the ``asm!``
invocation using ``out`` registers. In the case of 64-bit ARM Linux, no
additional register needs to be clobbered.

A safe wrapper around the ``WRITE`` system call would look like this:

.. code-block:: rust

   use core::ffi::c_int;

   pub fn write(fd: c_int, buf: &[u8]) -> Result<usize, isize> {
       const SYSCALL_NUMBER: usize = 64;

       let buf_pointer = buf.as_ptr();
       let buf_size = buf.len();
       let retval: isize;

       unsafe {
           core::arch::asm!(
               "SVC 0",
               in("x8") SYSCALL_NUMBER,
               inout("x0") fd as usize => retval,
               in("x1") buf_pointer,
               in("x2") buf_size,
               options(nostack),
           );
       }
       if retval >= 0 {
           Ok(retval as usize)
       } else {
           Err(retval)
       }
   }
