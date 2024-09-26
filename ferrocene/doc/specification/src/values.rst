.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_94a8v54bufn8:

Values
======

.. rubric:: Legality Rules

:dp:`fls_buyaqara7am4`
A :t:`value` is either a :t:`literal` or the result of a computation, that may
be stored in a memory location, and interpreted based on some :t:`type`.

:dp:`fls_CUJyMj0Sj8NS`
An :dt:`allocated object` is a :t:`value` stored at some memory address.

:dp:`fls_kaomYy0Ml4Nh`
An :t:`[allocated object]s` :dt:`base address` is the the memory address the
object is stored.

:dp:`fls_B5cmkWfD5GNt`
An :t:`[allocated object]s` :dt:`memory size` is the number of bytes the object
spans in memory from its :t:`base address`.

:dp:`fls_rixdyyc525xp`
Two :t:`[value]s` :t:`overlap` when

* :dp:`fls_m6ctqq70vcxr`
  Both :t:`[value]s` are the same, or

* :dp:`fls_s231d18x5eay`
  One :t:`value` is of an :t:`abstract data type` and the other denotes a
  :t:`field` of the same :t:`value`, or

* :dp:`fls_dfr4yqo93fsn`
  One :t:`value` denotes an :t:`array` and the other denotes an element of the
  same :t:`value`, or

* :dp:`fls_eoak5mdl6ma`
  Both :t:`[value]s` are elements of the same :t:`array`.

.. rubric:: Undefined Behavior

:dp:`fls_6lg0oaaopc26`
It is undefined behavior to create a :t:`value` from uninitialized memory unless
the :t:`type` of the :t:`value` is a :t:`union type`.

:dp:`fls_oqhQ62mDLckN`
It is undefined behavior to create an :t:`allocated object` at :t:`base address`
:c:`null`.

:dp:`fls_uhwpuv6cx4ip`
It is undefined behavior to create an :t:`allocated object` with :t:`memory
size` ``size`` at a :t:`base address` ``base`` where ``base + size`` is greater
than the architectures maximum :c:`usize` value.

:dp:`fls_xuuFKmm181bs`
It is undefined behavior to create an :t:`allocated object` with :t:`memory
size` ``size`` where ``size`` is greater than the architectures maximum
:c:`isize` value.

.. _fls_ixjc5jaamx84:

Constants
---------

.. rubric:: Syntax

.. syntax::

   ConstantDeclaration ::=
       $$const$$ (Name | $$_$$) TypeAscription ConstantInitializer? $$;$$

   ConstantInitializer ::=
       $$=$$ Expression

.. rubric:: Legality Rules

:dp:`fls_5o5iu4j8in4l`
A :t:`constant` is an :t:`immutable` :t:`value` whose uses are substituted by
the :t:`value`.

:dp:`fls_3mhj0kkupwuz`
An :t:`unnamed constant` is a :t:`constant` declared with character 0x5F (low
line).

:dp:`fls_ka4y2yd100dx`
The :t:`type specification` of a :t:`constant` shall have ``'static``
:t:`lifetime`.

:dp:`fls_vt9tlkd676ql`
The :t:`type` of a :t:`constant` shall implement the :std:`core::marker::Sized`
:t:`trait`.

:dp:`fls_ndmfqxjpvsqy`
A :t:`constant initializer` is a :t:`construct` that provides the :t:`value` of
its related :t:`constant`.

:dp:`fls_6rxwbbhf5tc5`
A :t:`constant` shall have a :t:`constant initializer`, unless it is an
:t:`associated trait constant`.

:dp:`fls_vnc3ttnid1qr`
The :t:`expression` of a :t:`constant initializer` shall be a
:t:`constant expression`.

:dp:`fls_deuo1pn8cjd6`
The value of a :t:`constant` is determined by evaluating its
:t:`constant initializer`.

:dp:`fls_5x0jv4cgbolx`
A use of a :t:`constant` is a :t:`value expression` and creates a copy of the
constant's value.

.. rubric:: Dynamic Semantics

:dp:`fls_xezt9hl069h4`
The :t:`elaboration` of a :t:`constant` evaluates its :t:`constant initializer`.

:dp:`fls_ndobth7s92if`
A :t:`path` that refers to a :t:`constant` is replaced with the :t:`value` of
the :t:`constant`.

.. rubric:: Examples

.. code-block:: rust

   const ZERO: u32 = 0;

.. _fls_xdvdl2ssnhlo:

Statics
-------

.. rubric:: Syntax

.. syntax::

   StaticDeclaration ::=
       $$static$$ $$mut$$? Name TypeAscription StaticInitializer? $$;$$

   StaticInitializer ::=
       $$=$$ Expression

.. rubric:: Legality Rules

:dp:`fls_ibrmiwfypldh`
A :t:`static` is a :t:`value` that is associated with a specific memory
location.

:dp:`fls_mt94jvoot9dx`
A :t:`static` defined within a :t:`generic function` exists once in the
output executable or library.

:dp:`fls_k0r2c6uq29tu`
The :t:`type specification` of a :t:`static` shall have ``'static``
:t:`lifetime`.

:dp:`fls_b6ods85htuyn`
The :t:`type` of a :t:`static` shall implement the :std:`core::marker::Sized`
:t:`trait`.

:dp:`fls_doi4z6u55bi7`
A :t:`mutable static` is a :t:`static` with :t:`keyword` ``mut`` whose
:t:`value` can be modified.

:dp:`fls_74hp208pto22`
Access to a :t:`mutable static` shall require :t:`unsafe context`.

:dp:`fls_jfde2vg6mtww`
An :t:`immutable static` is a :t:`static` whose :t:`value` cannot be modified.

:dp:`fls_k4tyqb1j6zjo`
The type of an :t:`immutable static` shall implement the
:std:`core::marker::Sync` :t:`trait`.

:dp:`fls_t17h5h6a6v4c`
A :t:`static initializer` is a :t:`construct` that provides the :t:`value` of
its related :t:`static`.

:dp:`fls_yq0hpy4jx2qb`
A :t:`static` shall have a :t:`static initializer`, unless it is an
:t:`external static`.

:dp:`fls_vgidvfwzm4ks`
The :t:`expression` of a :t:`static initializer` shall be a
:t:`constant expression`.

:dp:`fls_8dcldbvu7lav`
A use of a :t:`static` is a :t:`place expression` referring to the unique
location of the :t:`static`.

.. rubric:: Dynamic Semantics

:dp:`fls_w0nb0mphho7b`
The :t:`elaboration` of a :t:`static` evaluates its :t:`static initializer`.

:dp:`fls_eeocxst9vafn`
All :t:`[path]s` that refer to a :t:`static` refer to the same memory location.

:dp:`fls_47khd5ljsxeq`
A :t:`static` is not :t:`dropped` during :t:`destruction`.

:dp:`fls_dowxbphqvk3n`
A :t:`mutable static` whose :t:`type` is not subject to
:t:`interior mutability` may reside in read-only memory.

.. rubric:: Undefined Behavior

:dp:`fls_b5wsmii7vz3v`
It is undefined behavior to mutate an :t:`immutable static` whose :t:`type` is
not subject to :t:`interior mutability`.

.. rubric:: Examples

.. code-block:: rust

   static mut GLOBAL: u32 = 0;

.. _fls_cleoffpn5ew6:

Temporaries
-----------

.. rubric:: Legality Rules

:dp:`fls_awpw61yofckz`
A :t:`temporary` is an anonymous :t:`variable` produced by some intermediate
computation.

.. _fls_gho955gmob73:

Variables
---------

.. rubric:: Legality Rules

:dp:`fls_hl5tnd9yy252`
A :t:`variable` is a placeholder for a :t:`value` that is allocated on the
stack.

:dp:`fls_vgi0gh5zmoiu`
The following :t:`[construct]s` are :t:`[variable]s`:

* :dp:`fls_81dlbula47nu`
  A :t:`binding`.

* :dp:`fls_3p0sb9ppmg3w`
  A :t:`temporary`.

:dp:`fls_r9km9f969bu8`
A :t:`variable` shall be used only after it has been initialized through all
:t:`[reachable control flow path]s` up to the point of its usage.

.. rubric:: Dynamic Semantics

:dp:`fls_g8etd5lsgn9j`
A :t:`variable` is not initialized when allocated.

.. _fls_wttihxen35as:

Constant Promotion
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_udn9lyf3m0z6`
:t:`Constant promotion` is the process of converting a :t:`value expression`
into a :t:`constant`.

:dp:`fls_yvkdcs4pmxjf`
:t:`Constant promotion` is possible only when

* :dp:`fls_n570za6a9nqd`
  The :t:`value expression` is a :t:`constant expression`, and

* :dp:`fls_tms5r9f5ogcb`
  The :t:`type` of the :t:`value expression` does not have a :t:`destructor`,
  and

* :dp:`fls_bysv5r7iuf5j`
  The :t:`value expression` does not employ a :t:`struct expression`
  constructing a :std:`core::cell::UnsafeCell`, and

* :dp:`fls_3h5vr7xk2rrt`
  The :t:`value expression` only consists of operations that will always succeed
  evaluation, and

* :dp:`fls_3BGncWvMumEt`
  The :t:`value expression` is the :t:`operand` of an
  :t:`immutable borrow expression`.

:dp:`fls_m690b8qg9d9r`
:t:`Constant promotion` is always possible for :t:`expression` ``&mut []``,
promoting the produced :t:`mutable borrow` to have ``'static`` :t:`lifetime`.

:dp:`fls_uf0sg25awre6`
:t:`Constant promotion` proceeds as follows:

#. :dp:`fls_o7cqfdnr253y`
   An anonymous :t:`constant` is created, whose :t:`constant initializer` holds
   the result of the :t:`value expression`.

#. :dp:`fls_ap85svxyuhvg`
   The :t:`value` of the anonymous :t:`constant` is :t:`borrowed` with
   ``'static`` :t:`lifetime`.
