.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Ownership and Destruction
=========================

Ownership
---------

.. rubric:: Legality Rules

:dp:`fls_wt81sbsecmu0`
:dt:`Ownership` is a property of :t:`[object]s` that is central to the memory
management model of Rust. (**better explanation?**)

:dp:`fls_ckcnkbb6y3cq`
An :dt:`owner` is an :t:`object` that holds (**possesses?**) a :t:`value`.

:dp:`fls_ze0u9gfylmhn`
A :t:`value` shall have only one :t:`owner`.

Initialization
--------------

.. rubric:: Legality Rules

:dp:`fls_drfzu02bo7oe`
:t:`Initialization` is the act of supplying (**assigning?**) an initial
:t:`value` to an :t:`object`.

:dp:`fls_wnhci8phdu4m`
When an :t:`object` holds a :t:`value`, the :t:`object` is considered to be
:dt:`initialized`.

:dp:`fls_ch2lvm50olqd`
When an :t:`object` lacks a :t:`value` or its :t:`value` has been transferred
:t:`by move`, the :t:`object` is considered to be :dt:`uninitialized`.

:dp:`fls_46910buiwvv9`
An :t:`object` shall be :t:`initialized` before it is used.

:dp:`fls_u655u4buipi4`
**Should this subchapter talk about core::mem::MaybeUninit?**

.. rubric:: Runtime Semantics

:dp:`fls_caufcwkpz689`
All memory starts as uninitialized.

.. rubric:: Undefined Behavior

:dp:`fls_bp14qp2ll72l`
It is undefined behavior to reference (**read?**) uninitialized memory as a
:t:`value` of any :t:`type`.

.. rubric:: Examples

:dp:`fls_4lg92b9yima6`
Variable ``a`` is initialized.

.. code-block:: text

   let a: i32 = 42;

:dp:`fls_xn1au0blioa3`
Variable ``b`` starts off as uninitialized, but is later initialized by virtue
of the assignment statement.

.. code-block:: text

   let b: i32;
   b = 42;

:dp:`fls_jmcjboopvytb`
Variable ``c`` starts off initialized, but is later uninitialized by virtue of a
transfer by move.

.. code-block:: text

   use core::sync::atomic::AtomicI32;

   let c: AtomicI32 = AtomicI32::new(42);
   let d: AtomicI32 = c;

Transfer of Values
------------------

.. rubric:: Legality Rules

:dp:`fls_2tk0l8cntph9`
**Better term for "transfer"? By value use perhaps?**

:dp:`fls_6678l9kjjzmc`
:dt:`Transfer` is the act of **???**

:dp:`fls_425d9pw7ggd3`
The :t:`transfer` of a :t:`value` occurs when the :t:`value` is

* :dp:`fls_r7eg7r6o4q56`
  Assigned using an :t:`assignment expression`,

* :dp:`fls_hf0rwoqzk9et`
  Bound to a :t:`function parameter` in a :t:`call expression`,

* :dp:`fls_ra3n5mymgk1s`
  Bound to a :t:`pattern`,

* :dp:`fls_80a48wx5nkah`
  Captured by a :t:`capture expression`,

* :dp:`fls_esmvh2cffgpk`
  Returned from a :t:`function`.

* :dp:`fls_s9beyiawoeo8`
  **More?**

:dp:`fls_mxzibussnlpz`
A :dt:`by copy type` is a :t:`type` that implements the
:std:`core::marker::Copy` :t:`trait`.

:dp:`fls_9jmn63arik30`
A :t:`value` of a :t:`by copy type` is transferred :dt:`by copy`. Transferring
:t:`by copy` does not change the :t:`owner` of the :t:`value`.

:dp:`fls_ljfwr5dc9ixo`
A :dt:`by move type` is a :t:`type` that does not implement the
:std:`core::marker::Copy` :t:`trait`.

:dp:`fls_kg0lq5938rg4`
A :t:`value` of a :t:`by move type` is transferred :dt:`by move`. Transferring
:t:`by move` changes the :t:`owner` of the :t:`value`.

.. rubric:: Dynamic Semantics

:dp:`fls_gfk78xottqyj`
The :t:`transfer` of a :t:`value` :t:`by copy` from a source :t:`owner` to a
target :t:`owner` proceeds as follows:

#. :dp:`fls_fk2wfgul1d66`
   The ``core::marker::Copy::clone(&value)`` :t:`function` of the source
   :t:`owner` is invoked.

#. :dp:`fls_fs8pbfoi7co2`
   The result of :std:`core::marker::Copy::clone` is assigned to the target
   :t:`owner`.

:dp:`fls_j42hzl4rfq1s`
The :t:`transfer` of a :t:`value` :t:`by move` from a source :t:`owner` to a
target :t:`owner` proceeds as follows:

#. :dp:`fls_l046vfwad2kg`
   The :t:`value` is unassigned from the source :t:`owner`.

#. :dp:`fls_gdqyto7qnys6`
   The :t:`value` is assigned to the target :t:`owner`.

.. rubric:: Examples

:dp:`fls_evmk65swttl5`
Type ``i32`` is a by copy type. By the end of the second let statement, ``x`` is
the owner of the original ``42`` and ``y`` is the owner of a cloned ``42``.

.. code-block:: text

   let x: i32 = 42;
   let y: i32 = x;

:dp:`fls_dq7gew6yxi6m`
Type :std:`core::sync::atomic::AtomicI32` is a by move type. By the end of the
second let statement, ``x`` is uninitialized and ``y`` is the sole owner of the
atomic ``42``.

.. code-block:: text

   use core::sync::atomic::AtomicI32;

   let x: AtomicI32 = AtomicI32::new(42);
   let x: AtomicI32 = y;

References
----------

.. rubric:: Legality Rules

:dp:`fls_7x9pi2o7pee7`
A :dt:`reference` is a :t:`value` of a :t:`reference type`. A :t:`reference`
can be obtained explicitly by using a :t:`borrow expression` or implicitly in
certain scenarios.

:dp:`fls_tsqvr3fmcel`
A :dt:`referent` is the :t:`value` pointed-to (**better term?**) by a
:t:`reference`.

:dp:`fls_ev4a82fdhwr8`
A :t:`reference` shall point to an :t:`initialized` :t:`referent`.

:dp:`fls_cckf6dtkgwb4`
The :t:`lifetime` of a :t:`referent` shall be at least as long as the
:t:`lifetime` of its :t:`reference`.

:dp:`fls_8kqb8754e6p4`
A :t:`reference` is :dt:`active` from the point of obtaining its :t:`referent`
upto the last use of the :t:`reference`, prior to another assignment to the
:t:`reference` or the end of the :t:`scope` of the :t:`reference`.

:dp:`fls_v69rptdjao42`
A :t:`referent` shall not be transferred :t:`by move` while a :t:`reference` to
it is :t:`active`.

:dp:`fls_vg9h6tz6z37w`
A :t:`referent` shall not be modified while a :t:`reference` to it is
:t:`active`.

:dp:`fls_wcf5mxrzbujn`
An :dt:`immutable reference` is a :t:`value` of a :t:`shared reference type`,
and prevents the mutation of its :t:`referent`.

:dp:`fls_fckoj1jh5mrc`
A :dt:`mutable reference` is a :t:`value` of a :t:`mutable reference type`, and
allows the mutation of its :t:`referent`.

:dp:`fls_i1ny0k726a4a`
While a :t:`mutable reference` is :t:`active`, no other :t:`reference` shall
refer to a :t:`value` that :t:`[overlap]s` with the :t:`referent` of the
:t:`mutable reference`.

.. rubric:: Examples

.. code-block:: text

   let immutable_reference: &i32 = &42;
   let mutable_reference: &mut i32 = &mut 42;

Borrowing
---------

.. rubric:: Legality Rules

:dp:`fls_c02flohk54pc`
:dt:`Borrowing` is the process of temporarily associating a :t:`reference` with
a :t:`value` without transferring :t:`ownership` permanently.

:dp:`fls_j9kof0px3l7s`
A :dt:`borrow` is a :t:`reference` produced by :t:`borrowing`.

:dp:`fls_zepwytjwy049`
An :dt:`implicit borrow` is a :t:`borrow` that is not present syntactically in
program text. An :t:`implicit borrow` occurs in the following contexts:

* :dp:`fls_kky9ufexrvaw`
  The :t:`indexed array operand` of an :t:`array index expression`,

* :dp:`fls_nordokzfy36d`
  The :t:`call operand` of a :t:`call expression`,

* :dp:`fls_yfmy4v5zlgw9`
  The :t:`assigned operand` of a :t:`compound assignment expression`,

* :dp:`fls_bjf3futso849`
  The :t:`[operand]s` of a :t:`comparison expression`,

* :dp:`fls_jv18y618j2s3`
  The :t:`operand` of a :t:`field access expression`,

* :dp:`fls_g4i0jb27iryr`
  The :t:`operand` of a :t:`dereference expression`,

* :dp:`fls_o5oq4jfswr4q`
  The :t:`receiver operand` of a :t:`method call expression`.

:dp:`fls_hyl4bdjbuzbw`
An :dt:`immutable borrow` is an :t:`immutable reference` produced by
:t:`borrowing`.

:dp:`fls_pu19i4sj6yg0`
A :dt:`mutable borrow` is a :t:`mutable reference` produced by :t:`borrowing`.

:dp:`fls_kxws4zmaahj6`
:t:`Borrowing` a :t:`field` of a :t:`union type` borrows all remaining
:t:`[field]s` using the same :t:`lifetime`.

:dp:`fls_kup2ou22nwyl`
Immutably :t:`borrowing` a :t:`value` proceeds as follows:

#. :dp:`fls_5bf2x4sm5ei`
   **???** (**this should describe the order of borrowing and when the borrow
   is returned**)

:dp:`fls_g4aefz28pl04`
Uniquely immutably :t:`borrowing` a :t:`value` proceeds as follows:

#. :dp:`fls_dpe3ubsd67ra`
   **???**

:dp:`fls_f9we73i8vwq3`
Mutably :t:`borrowing` a :t:`value` proceeds as follows:

#. :dp:`fls_1xme4or6udai`
   **???**

.. rubric:: Examples

.. code-block:: text

   let immutable_borrow = &42;

:dp:`fls_yhl7tkgu8las`
**Example of unique immutable borrowing?**

.. code-block:: text

   let mutable_borrow = &mut 42;

Destruction
-----------

.. rubric:: Legality Rules

:dp:`fls_e7ucq87s806d`
:dt:`Destruction` is the process of recovering resources associated with an
:t:`object` as it goes out of scope.

Destructors
-----------

.. rubric:: Legality Rules

:dp:`fls_9m0gszdle0qb`
A :dt:`drop type` is a :t:`type` that implements the :std:`core::ops:Drop`
:t:`trait` or contains a :t:`field` that is of a :t:`drop type`. (**should this
include the field portion?**)

:dp:`fls_4nkzidytpi6`
A :dt:`destructor` is an anonymous :t:`function` that performs the
:t:`destruction` of an :t:`object` of a :t:`drop type`.

:dp:`fls_wzuwapjqtyyy`
:dt:`Dropping` an :t:`object` is the act of invoking the :t:`destructor` of the
related :t:`type`. Such an object is said to be :dt:`dropped`.

:dp:`fls_gfvm70iqu1l4`
An :t:`uninitialized` :t:`object` is not :t:`dropped`.

.. rubric:: Dynamic Semantics

:dp:`fls_l2xkdjeydqtx`
:t:`Dropping` an :t:`initialized` :t:`object` proceeds as follows:

#. :dp:`fls_bync24y6gp93`
   If the :t:`drop type` implements the :std:`core::ops:Drop` :t:`trait`, then
   ``core::ops:Drop::drop()`` is invoked.

#. :dp:`fls_jzancf72i95f`
   If the :t:`drop type` is an :t:`array type`, then its elements are
   :t:`dropped` from the first element to the last element.

#. :dp:`fls_gjn2jnsal9gs`
   Otherwise, if the :t:`drop type` is a :t:`closure type`, then all
   :t:`[capture target]s` whose :t:`capture mode` is :t:`by move mode` are
   :t:`dropped` in unspecified order.

#. :dp:`fls_ol2w2292frfi`
   Otherwise, if the :t:`drop type` is an :t:`enum type`, then the :t:`[field]s`
   of the active :t:`enum variant` are :t:`dropped` in declaration order.

#. :dp:`fls_6ii5o68vuymj`
   Otherwise, if the :t:`drop type` is a :t:`slice type`, then its elements are
   :t:`dropped` from the first element to the last element.

#. :dp:`fls_sup43es8ps8r`
   Otherwise, if the :t:`drop type` is a :t:`struct type`, then its
   :t:`[field]s` are :t:`dropped` in declaration order.

#. :dp:`fls_y9q0eqr865b3`
   Otherwise, if the :t:`drop type` is a :t:`trait object type`, then the
   :t:`destructor` of the underlying :t:`type` is invoked.

#. :dp:`fls_kdqng6eovxns`
   Otherwise, if the :t:`drop type` is a :t:`tuple type`, then its :t:`[field]s`
   are :t:`dropped` in declaration order.

.. rubric:: Examples

.. code-block:: text

   struct PrintOnDrop(&'static str);

   impl core::ops::Drop for PrintOnDrop {
       fn drop(&mut self) {
           println!("{}", self.0);
       }
   }

:dp:`fls_tw36n3g32a0y`
When object ``array`` is dropped, its destructor drops the first element, then
the second element.

.. code-block:: text

   let array = [PrintOnDrop("first element to be dropped"),
                PrintOnDrop("second element to be dropped")];

:dp:`fls_fmn33zhorkf`
Object ``uninitialized`` is not dropped.

.. code-block:: text

   let uninitialized: PrintOnDrop;

Drop Scopes
-----------

.. rubric:: Legality Rules

:dp:`fls_7uav7vkcv4pz`
A :dt:`drop scope` is a region of program text that governs the :t:`dropping` of
:t:`[object]s`. When control flow leaves a :t:`drop scope`, all :t:`[object]s`
associated with that :t:`drop scope` are :t:`dropped` based on a :t:`drop
order`.

:dp:`fls_y88ye36v4qs7`
:t:`[Drop scope]s` are determined after replacing :t:`[if let expression]s`,
:t:`[for loop expression]s`, and :t:`[while let loop expression]s` with
equivalent :t:`[match expression]s`, ignoring :t:`[binding mode]s` and
overloaded operators. (**what are these equivalent expressions?**)

:dp:`fls_txvxrn6wbyql`
A :dt:`drop construct` is a :t:`construct` that employs a :t:`drop scope`. The
following :t:`[construct]s` are :t:`[drop construct]s`:

* :dp:`fls_n6y6brm6pghr`
  :t:`[Expression]s`,

* :dp:`fls_gdh6wwvi7ci6`
  :t:`[Function]s`,

* :dp:`fls_owqk2fcpvc4s`
  A :t:`match arm` of a :t:`match expression`,

* :dp:`fls_ckh8wkq0y5ja`
  :t:`[Statement]s`.

:dp:`fls_2zwwnzepgmje`
:t:`[Drop scope]s` are nested within one another as follows:

* :dp:`fls_vlbx5ukw5c8l`
  The :t:`drop scope` of a :t:`function` is the outermost :t:`drop scope`.
  (**does this include closure expressions?**)

* :dp:`fls_d5yg6w8gv6aq`
  The :t:`drop scope` of a :t:`function body` is the :t:`drop scope` of its
  related :t:`function`.

* :dp:`fls_qidma4fpkhb0`
  The parent :t:`drop scope` of an :t:`operand` in an :t:`expression statement`
  is the :t:`drop scope` of the :t:`expression statement`.

* :dp:`fls_1o9ye6cwoyiq`
  The parent :t:`drop scope` of the :t:`expression` of a :t:`let statement` is
  the :t:`drop scope` of the :t:`let statement`.

* :dp:`fls_16htxf824xbk`
  The parent :t:`drop scope` of a :t:`statement` is the :t:`drop scope` of the
  :t:`block expression` that contains the :t:`statement`.

* :dp:`fls_lbsfhg42yiqy`
  The parent :t:`drop scope` of the :t:`operand` of a :t:`match guard` is the
  :t:`drop scope` of the :t:`match arm` that contains the :t:`match guard`.

* :dp:`fls_5m3u3k6f00bd`
  The parent :t:`drop scope` of the :t:`operand` of a :t:`match arm` is the
  :t:`drop scope` of the :t:`match arm` that contains the :t:`operand`.

* :dp:`fls_m86ljncnmo7j`
  The parent :t:`drop scope` of a :t:`match arm` is the :t:`drop scope` of the
  related :t:`match expression`.

* :dp:`fls_bewcu5xceu8i`
  The parent :t:`drop scope` of all other :t:`[drop scope]s` is the :t:`drop
  scope` of the immediately enclosing :t:`expression`.

:dp:`fls_vrqgac634wpr`
A :t:`variable` declared in a :t:`let statement` is associated with the :t:`drop
scope` of the :t:`block expression` that contains the :t:`let statement`.

:dp:`fls_fnvr5w2wzxns`
A :t:`variable` declared in a :t:`match expression` is associated with the
:t:`drop scope` of the :t:`match arm` of the :t:`match expression`.

:dp:`fls_8r39duatupxw`
A :t:`temporary` that is not subject to :t:`constant promotion` is associated
with the smallest :t:`drop scope` that contains the :t:`expression` which
produced the :t:`temporary`, taking into account :t:`drop scope extension`. The
possible :t:`[drop scope]s` are as follows:

* :dp:`fls_2peq1tihglnr`
  The :t:`drop scope` of a :t:`function body`. (**what about closure
  expressions?**)

* :dp:`fls_il09n7sq0a3k`
  The :t:`drop scope` of a :t:`statement`.

* :dp:`fls_s1wbqld139gz`
  The :t:`drop scope` of a :t:`block expression` of an :t:`if expression`, an
  :t:`infinite loop expression`, or a :t:`while loop expression`.

* :dp:`fls_asvuef2pc3m0`
  The :t:`drop scope` of an :t:`else expression`.

* :dp:`fls_560437qmeqtr`
  The :t:`drop scope` of the :t:`subject expression` of an :t:`if expression`.

* :dp:`fls_8cunkfc6x24q`
  The :t:`drop scope` of the :t:`iteration expression` of a :t:`while loop
  expression`.

* :dp:`fls_n108lvc4otoc`
  The :t:`drop scope` of the :t:`operand` of a :t:`match arm`.

* :dp:`fls_ptk6yibqyfzi`
  The :t:`drop scope` of the :t:`operand` of a :t:`match guard`.

* :dp:`fls_dltmd8e8c5ia`
  The :t:`drop scope` of the :t:`right operand` of a :t:`lazy boolean
  expression`.

:dp:`fls_dlycy35wdpah`
A :t:`function parameter` is associated with the :t:`drop scope` of the related
:t:`function body`.

:dp:`fls_nbha4yxqvvew`
A :t:`function argument` is associated with the :t:`drop scope` of the related
:t:`call expression` or :t:`method call expression`.

.. rubric:: Dynamic Semantics

:dp:`fls_eimiow8ap9io`
:t:`Dropping` a :t:`raw pointer` has no effect.

Constant Promotion
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_le2ip4najsot`
**Is this the right place for this subchapter?**

:dp:`fls_udn9lyf3m0z6`
:dt:`Constant promotion` is the process of converting a :t:`value expression`
into a :t:`constant`.

:dp:`fls_ysp33rove4fp`
**What are the conditions under which constant promotion can take place?
Something about not having interior mutability and destructors...**

:dp:`fls_uf0sg25awre6`
:t:`Constant promotion` proceeds as follows:

#. :dp:`fls_o7cqfdnr253y`
   An anonymous :t:`constant` is created, whose :t:`constant initializer` holds
   the result of the :t:`value expression`.

#. :dp:`fls_ap85svxyuhvg`
   The :t:`value` of the anonymous :t:`constant` is :t:`borrowed`.

#. :dp:`fls_v9c0aaxotpe8`
   The :t:`borrow` is dereferenced in the original context where the :t:`value
   expression` resided. (**does the borrow replace the original value
   expression?**)

Interior Mutability
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_1jdi9p4jv15a`
**Is this the right place for this subchapter?**

:dp:`fls_khy2e23i9o7z`
:dt:`Interior mutability` is a property of :t:`[type]s` whose :t:`[value]s` can
be modified through :t:`[immutable reference]s`.

:dp:`fls_b0qz36dboifg`
**What are the rules and ramifications of interior mutability? The Rust
Reference just chats about this...**

Drop Scope Extension
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_kflqez2mtbit`
:dt:`Drop scope extension` is the process of extending a :t:`drop scope`
associated with a :t:`temporary` to prevent the premature :t:`dropping` of the
:t:`temporary`.

:dp:`fls_xjw82bujm148`
An :dt:`extending pattern` is either

* :dp:`fls_965wt48ooqyw`
  An :t:`identifier pattern` whose :t:`binding mode` is "by reference" or "by
  mutable reference", or

* :dp:`fls_r8nt0zp8dnyp`
  A :t:`slice pattern`, a :t:`struct pattern`, or a :t:`tuple pattern` that
  contains at least one :t:`subpattern` that is an :t:`extending pattern`.

:dp:`fls_3ycn4u1fe9h`
If the :t:`pattern-without-alternation` of a :t:`let statement` is an
:t:`extending pattern`, then the :t:`drop scope` of the :t:`expression` of
the :t:`let statement` is extended to the :t:`drop scope` of the :t:`block
expression` that contains the :t:`let statement`.

:dp:`fls_wyzau8hhq74d`
An :dt:`extending expression` is either

* :dp:`fls_ju7cpftd8r2g`
  The :t:`expression` of a :t:`let statement`, or

* :dp:`fls_gjd1ow3l7swe`
  The :t:`operand` of an extending :t:`array expression`, an extending
  :t:`borrow expression`, an extending :t:`cast expression`, an extending
  :t:`struct expression`, or an extending :t:`tuple expression` (**what are
  these?**), or

* :dp:`fls_iqw0d1l1lj3i`
  The :t:`tail expression` of an extending :t:`block expression`.

:dp:`fls_j4ygps77scfl`
**Whose drop scope do extending expressions affect, and how?**

.. rubric:: Examples

:dp:`fls_29y59x8bmw75`
See :p:`15.6.1. <fls_u2mzjgiwbkz0>` for the declaration of ``PrintOnDrop``.

:dp:`fls_subo2w7ln43q`
The drop scope of ``a`` is extended to the drop scope of the block expression.
As a result, the example prints ``"a"`` twice, the first time when the
``println!`` macro is invoked and the second time when ``a`` is dropped.

.. code-block:: text

   {
       let a = PrintOnDrop("a");
       println!("{}", a.0);
   }

Drop Order
----------

.. rubric:: Legality Rules

:dp:`fls_n6o1xzjiz8cv`
:dt:`Drop order` is the order by which :t:`[object]s` are :t:`dropped` when a
:t:`drop scope` is left.

:dp:`fls_jwofws3022ar`
When a :t:`drop scope` is left, all :t:`[object]s` associated with that :t:`drop
scope` are :t:`dropped` as follows:

* :dp:`fls_g07zq3n55094`
  :t:`[Variable]s` are :t:`dropped` in reverse declaration order.

* :dp:`fls_a5tmilqxdb6f`
  Temporaries are :t:`dropped` in reverse creation order.

:dp:`fls_3i348l3pbtrx`
When multiple :t:`[drop scope]s` are left at once, the :t:`[object]s` are
:t:`dropped` from the innermost :t:`drop scope` to the outermost :t:`drop
scope`.

.. rubric:: Examples

:dp:`fls_oe8l81y0wnao`
See :p:`15.6.1. <fls_u2mzjgiwbkz0>` for the declaration of ``PrintOnDrop``.

:dp:`fls_4sgca9wcl8h0`
The drop order of the following variables is ``b``, ``c``, ``a``. Dropping
proceeds as follows:

#. :dp:`fls_a2m4ibzhgupa`
   The scope of the block expression is left first because it is an inner scope.

#. :dp:`fls_go3bvd23vzi9`
   ``b`` is dropped.

#. :dp:`fls_7rwo0he8x143`
   The outer scope is left.

#. :dp:`fls_43yqlxjr3a10`
   ``c`` is dropped because dropping occurs in reverse declarative order.

#. :dp:`fls_a7lsq2kkzkk4`
   ``a`` is dropped.

.. code-block:: text

   let a = PrintOnDrop("3");
   {
       let b = PrintOnDrop("1");
   }
   let c = PrintOnDrop("2");

