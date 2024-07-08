.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_ronnwodjjjsh:

Ownership and Destruction
=========================

.. _fls_svkx6szhr472:

Ownership
---------

.. rubric:: Legality Rules

:dp:`fls_wt81sbsecmu0`
:t:`Ownership` is a property of :t:`[value]s` that is central to the resource
management model of Rust.

:dp:`fls_ckcnkbb6y3cq`
An :t:`owner` is a :t:`variable` that holds a :t:`value`.

:dp:`fls_ze0u9gfylmhn`
A :t:`value` shall have only one :t:`owner`.

.. _fls_3xvm61x0t251:

Initialization
--------------

.. rubric:: Legality Rules

:dp:`fls_drfzu02bo7oe`
:t:`Initialization` is the act of supplying an initial :t:`value` to a
:t:`variable`.

:dp:`fls_wnhci8phdu4m`
When a :t:`variable` holds a :t:`value`, the :t:`variable` is considered to be
:dt:`initialized`.

:dp:`fls_ch2lvm50olqd`
When a :t:`variable` lacks a :t:`value` or its :t:`value` has been
:t:`passed <passing convention>` :t:`by move`, the :t:`variable` is considered
to be :dt:`uninitialized`.

:dp:`fls_46910buiwvv9`
A :t:`variable` shall be :t:`initialized` before it is accessed.

.. rubric:: Runtime Semantics

:dp:`fls_caufcwkpz689`
All memory starts as uninitialized.

.. rubric:: Examples

:dp:`fls_4lg92b9yima6`
Variable ``a`` is initialized.

.. code-block:: rust

   let a: i32 = 42;

:dp:`fls_xn1au0blioa3`
Variable ``b`` starts off as uninitialized, but is later initialized by virtue
of the assignment statement.

.. code-block:: rust

   let b: i32;
   b = 42;

:dp:`fls_jmcjboopvytb`
Variable ``c`` starts off initialized, but is later uninitialized by virtue of a
transfer by move.

.. code-block:: rust

   use core::sync::atomic::AtomicI32;

   let c: AtomicI32 = AtomicI32::new(42);
   let d: AtomicI32 = c;

.. _fls_v5x85lt5ulva:

References
----------

.. rubric:: Legality Rules

:dp:`fls_7x9pi2o7pee7`
A :t:`reference` is a :t:`value` of a :t:`reference type`. A :t:`reference`
can be obtained explicitly by using a :t:`borrow expression` or implicitly in
certain scenarios.

:dp:`fls_tsqvr3fmcel`
A :t:`referent` is the :t:`value` pointed-to by a :t:`reference`.

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
A :t:`referent` shall not be :t:`passed <passing convention>` :t:`by move` while
a :t:`reference` to it is :t:`active`.

:dp:`fls_vg9h6tz6z37w`
A :t:`referent` shall not be modified while a :t:`reference` to it is
:t:`active`.

:dp:`fls_wcf5mxrzbujn`
An :t:`immutable reference` is a :t:`value` of a :t:`shared reference type`, and
prevents the mutation of its :t:`referent`.

:dp:`fls_fckoj1jh5mrc`
A :t:`mutable reference` is a :t:`value` of a :t:`mutable reference type`, and
allows the mutation of its :t:`referent`.

:dp:`fls_hqxsuyn285he`
The :t:`referent` of an :t:`immutable reference` shall be mutated only when the
:t:`type` of the :t:`referent` is subject to :t:`interior mutability`.

:dp:`fls_i1ny0k726a4a`
While a :t:`mutable reference` is :t:`active`, no other :t:`reference` shall
refer to a :t:`value` that :t:`[overlap]s` with the :t:`referent` of the
:t:`mutable reference`.

.. rubric:: Undefined Behavior

:dp:`fls_eT1hnLOx6vxk`
It is undefined behavior to access a :t:`value` through aliasing
:t:`[mutable reference]s` from :t:`unsafe context`.

.. rubric:: Examples

.. code-block:: rust

   let immutable_reference: &i32 = &42;
   let mutable_reference: &mut i32 = &mut 42;

.. _fls_a14slch83hzn:

Borrowing
---------

.. rubric:: Legality Rules

:dp:`fls_c02flohk54pc`
:t:`Borrowing` is the process of temporarily associating a :t:`reference` with a
:t:`value` without transferring :t:`ownership` permanently.

:dp:`fls_j9kof0px3l7s`
A :t:`borrow` is a :t:`reference` produced by :t:`borrowing`.

:dp:`fls_zepwytjwy049`
An :t:`implicit borrow` is a :t:`borrow` that is not present syntactically in
program text. An :t:`implicit borrow` occurs in the following contexts:

* :dp:`fls_nordokzfy36d`
  The :t:`call operand` of a :t:`call expression`,

* :dp:`fls_bjf3futso849`
  The :t:`[operand]s` of a :t:`comparison expression`,

* :dp:`fls_yfmy4v5zlgw9`
  The :t:`assigned operand` of a :t:`compound assignment expression`,

* :dp:`fls_jv18y618j2s3`
  The :t:`operand` of a :t:`field access expression`,

* :dp:`fls_g4i0jb27iryr`
  The :t:`operand` of a :t:`dereference expression`,

* :dp:`fls_kky9ufexrvaw`
  The :t:`indexed operand` of an :t:`index expression`,

* :dp:`fls_o5oq4jfswr4q`
  The :t:`receiver operand` of a :t:`method call expression`.

:dp:`fls_MT69AGRXH9pS``
An :t:`implicit borrow` may be an :t:`immutable borrow` or a :t:`mutable borrow`
if required.

:dp:`fls_hyl4bdjbuzbw`
An :t:`immutable borrow` is an :t:`immutable reference` produced by
:t:`borrowing`.

:dp:`fls_pu19i4sj6yg0`
A :t:`mutable borrow` is a :t:`mutable reference` produced by :t:`borrowing`.

:dp:`fls_kxws4zmaahj6`
:t:`Borrowing` a :t:`field` of a :t:`union type` borrows all remaining
:t:`[field]s` using the same :t:`lifetime`.

:dp:`fls_kup2ou22nwyl`
Immutably :t:`borrowing` a :t:`value` proceeds as follows:

#. :dp:`fls_5bf2x4sm5ei`
   **???** (**this should describe the order of borrowing and when the borrow
   is returned**)

#. :dp:`fls_8q5ly4x104ai`
   An :t:`immutable borrow` of :t:`type` ``&'a T`` is created, where
   :t:`lifetime` ``'a`` is replaced by a :t:`lifetime variable`, and
   ``T`` is replaced by the borrowed :t:`type`.

#. :dp:`fls_yhchu2bpil4m`
   Lifetime inference is performed.

#. :dp:`fls_568o7nyihndd`
   The :t:`immutable borrow` is checked against other :t:`[borrow]s` and
   :t:`by move` passing within the enclosing :t:`item`.

#. :dp:`fls_Z2n7EJiLY5CT`
   An :t:`immutable reference` to the borrowed :t:`value` is produced.

#. :dp:`fls_ibi9mqGeHNLp`
   The :t:`immutable borrow` is released immediately after the last usage of its
   related :t:`immutable reference`.

:dp:`fls_f9we73i8vwq3`
Mutably :t:`borrowing` a :t:`value` proceeds as follows:

#. :dp:`fls_w5bjgaov8w60`
   A :t:`mutable borrow` of :t:`type` ``&'a mut T`` is created, where
   :t:`lifetime` ``'a`` is replaced by a :t:`lifetime variable`, and
   ``T`` is replaced by the borrowed :t:`type`.

#. :dp:`fls_gbqizu6gu6kk`
   Lifetime inference is performed.

#. :dp:`fls_ovkkxeybumvt`
   The :t:`mutable borrow` is checked against other :t:`[borrow]s` and
   :t:`by move` passing within the enclosing :t:`item`.

#. :dp:`fls_7Kzu0qXiBUCb`
   A :t:`mutable reference` to the borrowed :t:`value` is produced.

#. :dp:`fls_VDHBP4oAoqT5`
   The :t:`mutable borrow` is released immediately after the last usage of its
   related :t:`mutable reference`.

.. rubric:: Examples

.. code-block:: rust

   let mutable_borrow = &mut 42;
   let immutable_borrow = &42;

.. _fls_77scxuomlbgs:

Passing Conventions
-------------------

.. rubric:: Legality Rules

:dp:`fls_fvwx2ufeyzcs`
A :t:`passing convention` is the mechanism that defines how a :t:`value` is
transferred between :t:`[place]s`.

:dp:`fls_h2pgsij1rbms`
A :t:`copy type` is a :t:`type` that implements the :std:`core::marker::Copy`
:t:`trait`.

:dp:`fls_yx2knbby70fy`
A :t:`value` of a :t:`copy type` is :t:`passed <passing convention>`
:dt:`by copy`. Passing :t:`by copy` does not change the :t:`owner` of the
:t:`value`.

:dp:`fls_6ul3f6v0foma`
A :t:`move type` is a :t:`type` that implements the :std:`core::marker::Sized`
:t:`trait` and is not a :t:`copy type`.

:dp:`fls_3ztdz02efeoc`
A :t:`value` of a :t:`move type` is :t:`passed <passing convention>`
:dt:`by move`. Passing :t:`by move` changes the :t:`owner` of the :t:`value`.

:dp:`fls_konzgoybhfqm`
A :t:`value` of a :t:`place expression` shall be
:t:`passed <passing convention>` :t:`by move` only when it denotes:

* :dp:`fls_4bnbv7mqod57`
  A :t:`field` of a :t:`place expression` that can be
  :t:`passed <passing convention>` :t:`by move` and whose :t:`type` does not
  implement the :std:`core::ops::Drop` :t:`trait`, or

* :dp:`fls_3xk3p1unbjy5`
  A :t:`temporary`, or

* :dp:`fls_vk1xhvdaakh0`
  A :t:`variable` which is not currently :t:`borrowed`.

:dp:`fls_vveEJn7lngT8`
A :t:`value` of a :t:`value expression` is always
:t:`passed <passing convention>` :t:`by move`.

:dp:`fls_gq35gqagw35`
A :t:`value` not subject to :t:`by copy` or :t:`by move` :t:`passing convention`
shall not be :t:`passed <passing convention>` between :t:`[place]s`.

.. rubric:: Dynamic Semantics

:dp:`fls_cfqzpmnfeh8h`
Passing a :t:`value` :t:`by copy` from a source :t:`owner` to a target
:t:`owner` proceeds as follows:

#. :dp:`fls_go9gdlk5d3km`
   The :t:`value` of the source :t:`owner` is copied.

#. :dp:`fls_459xx6febmf0`
   The copy is assigned to the target :t:`owner`.

:dp:`fls_3xyq50abdiv6`
Passing a :t:`value` :t:`by move` from a source :t:`owner` to a target
:t:`owner` proceeds as follows:

#. :dp:`fls_7kcx3u8gvl1d`
   The :t:`value` is unassigned from the source :t:`owner`.

#. :dp:`fls_i4hrifsb9msr`
   The :t:`value` is assigned to the target :t:`owner`.

.. rubric:: Examples

:dp:`fls_7tadh1zel0fc`
Type ``i32`` is a copy type. By the end of the second let statement, ``x`` is
the owner of the original ``42`` and ``y`` is the owner of a cloned ``42``.

.. code-block:: rust

   let x: i32 = 42;
   let y: i32 = x;

:dp:`fls_ywt328hcieka`
Type :std:`core::sync::atomic::AtomicI32` is a move type. By the end of the
second let statement, ``x`` is uninitialized and ``y`` is the sole owner of the
atomic ``42``.

.. code-block:: rust

   use core::sync::atomic::AtomicI32;

   let x: AtomicI32 = AtomicI32::new(42);
   let y: AtomicI32 = x;

.. _fls_4jiw35pan7vn:

Destruction
-----------

.. rubric:: Legality Rules

:dp:`fls_e7ucq87s806d`
:t:`Destruction` is the process of recovering resources associated with a
:t:`value` as it goes out of scope.

.. _fls_u2mzjgiwbkz0:

Destructors
-----------

.. rubric:: Legality Rules

:dp:`fls_9m0gszdle0qb`
A :t:`drop type` is a :t:`type` that implements the :std:`core::ops::Drop`
:t:`trait` or contains a :t:`field` that has a :t:`drop type`.

:dp:`fls_4nkzidytpi6`
A :t:`destructor` is a :t:`function` that is invoked immediately before the
:t:`destruction` of a :t:`value` of a :t:`drop type`.

:dp:`fls_wzuwapjqtyyy`
:t:`Dropping` a :t:`value` is the act of invoking the :t:`destructor` of the
related :t:`type`. Such an object is said to be :dt:`dropped`.

:dp:`fls_gfvm70iqu1l4`
An :t:`uninitialized` :t:`variable` is not :t:`dropped`.

.. rubric:: Dynamic Semantics

:dp:`fls_l2xkdjeydqtx`
:t:`Dropping` an :t:`initialized` :t:`variable` proceeds as follows:

#. :dp:`fls_bync24y6gp93`
   If the :t:`drop type` implements the :std:`core::ops::Drop` :t:`trait`, then
   ``core::ops::Drop::drop`` of the :t:`drop type` is invoked.

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

#. :dp:`fls_ag249y74jg6c`
   Otherwise, :t:`dropping` has no effect.

.. rubric:: Examples

.. code-block:: rust

   struct PrintOnDrop(&'static str);

   impl core::ops::Drop for PrintOnDrop {
       fn drop(&mut self) {
           println!("{}", self.0);
       }
   }

:dp:`fls_tw36n3g32a0y`
When object ``array`` is dropped, its destructor drops the first element, then
the second element.

.. code-block:: rust

   let array = [PrintOnDrop("first element to be dropped"),
                PrintOnDrop("second element to be dropped")];

:dp:`fls_fmn33zhorkf`
Object ``uninitialized`` is not dropped.

.. code-block:: rust

   let uninitialized: PrintOnDrop;

.. _fls_rm4ncoopcdvj:

Drop Scopes
-----------

.. rubric:: Legality Rules

:dp:`fls_7uav7vkcv4pz`
A :t:`drop scope` is a region of program text that governs the :t:`dropping` of
:t:`[value]s`. When control flow leaves a :t:`drop scope`, all :t:`[value]s`
associated with that :t:`drop scope` are :t:`dropped` based on a
:t:`drop order`.

:dp:`fls_txvxrn6wbyql`
A :t:`drop construct` is a :t:`construct` that employs a :t:`drop scope`. The
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

* :dp:`fls_d5yg6w8gv6aq`
  The parent :t:`drop scope` of a :t:`function body` is the :t:`drop scope` of
  its related :t:`function`.

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
  The parent :t:`drop scope` of the :t:`operand` of a :t:`match arm guard` is
  the :t:`drop scope` of the :t:`match arm` that contains the
  :t:`match arm guard`.

* :dp:`fls_5m3u3k6f00bd`
  The parent :t:`drop scope` of the :t:`operand` of a :t:`match arm` is the
  :t:`drop scope` of the :t:`match arm` that contains the :t:`operand`.

* :dp:`fls_m86ljncnmo7j`
  The parent :t:`drop scope` of a :t:`match arm` is the :t:`drop scope` of the
  related :t:`match expression`.

* :dp:`fls_bewcu5xceu8i`
  The parent :t:`drop scope` of all other :t:`[drop scope]s` is the
  :t:`drop scope` of the immediately enclosing :t:`expression`.

:dp:`fls_iHHhlxCiqNWZ`
A :t:`binding` declared in a :t:`for loop expression` is associated with
the :t:`drop scope` of the :t:`block expression` of the
:t:`for loop expression`.

:dp:`fls_swXXkbZGLPKa`
A :t:`binding` declared in an :t:`if let expression` is associated with the
:t:`drop scope` of the :t:`block expression` of the :t:`if let expression`.

:dp:`fls_vrqgac634wpr`
A :t:`binding` declared in a :t:`let statement` is associated with the
:t:`drop scope` of the :t:`block expression` that contains the
:t:`let statement`.

:dp:`fls_fnvr5w2wzxns`
A :t:`binding` declared in a :t:`match expression` is associated with the
:t:`drop scope` of the :t:`match arm` of the :t:`match expression`.

:dp:`fls_1vZFa5mDEbXW`
A :t:`binding` declared in a :t:`while let loop expression` is associated with
the :t:`drop scope` of the :t:`block expression` of the
:t:`while let loop expression`.

:dp:`fls_3jtU8grRaMM9`
A :t:`value` or :t:`binding` of a :t:`function parameter` is associated with the
:t:`drop scope` of the :t:`function` of the :t:`function parameter`.

:dp:`fls_8r39duatupxw`
A :t:`temporary` that is not subject to :t:`constant promotion` is associated
with the innermost :t:`drop scope` that contains the :t:`expression` which
produced the :t:`temporary`, taking into account :t:`drop scope extension`. The
possible :t:`[drop scope]s` are as follows:

* :dp:`fls_2peq1tihglnr`
  The :t:`drop scope` of a :t:`function body`.

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
  The :t:`drop scope` of the :t:`iteration expression` of a
  :t:`while loop expression`.

* :dp:`fls_n108lvc4otoc`
  The :t:`drop scope` of the :t:`operand` of a :t:`match arm`.

* :dp:`fls_ptk6yibqyfzi`
  The :t:`drop scope` of the :t:`operand` of a :t:`match arm guard`.

* :dp:`fls_dltmd8e8c5ia`
  The :t:`drop scope` of the :t:`right operand` of a
  :t:`lazy boolean expression`.

.. _fls_5eima0pd31c0:

Drop Scope Extension
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_kflqez2mtbit`
:t:`Drop scope extension` is the process of extending a :t:`drop scope`
associated with a :t:`temporary` to prevent the premature :t:`dropping` of the
:t:`temporary`.

:dp:`fls_xjw82bujm148`
An :dt:`extending pattern` is either

* :dp:`fls_965wt48ooqyw`
  A :t:`reference identifier pattern`, or

* :dp:`fls_r8nt0zp8dnyp`
  A :t:`slice pattern`, a :t:`struct pattern`, :t:`tuple pattern` or a
  :t:`tuple struct pattern` that contains at least one :t:`subpattern` that is
  an :t:`extending pattern`.

:dp:`fls_3ycn4u1fe9h`
If the :t:`pattern-without-alternation` of a :t:`let statement` is an
:t:`extending pattern`, then the :t:`drop scope` of the :t:`expression` of
the :t:`let statement` is extended to the :t:`drop scope` of the
:t:`block expression` that contains the :t:`let statement`.

:dp:`fls_wyzau8hhq74d`
An :dt:`extending expression` is either

* :dp:`fls_ju7cpftd8r2g`
  The :t:`expression` of a :t:`let statement`, or

* :dp:`fls_gjd1ow3l7swe`
  The :t:`operand` of an :t:`extending expression` that is an
  :t:`array expression`, a :t:`borrow expression`, a :t:`struct expression`, a
  :t:`tuple expression`, or a :t:`type cast expression`, or

* :dp:`fls_Rj9zhVutfQod`
  The :t:`block expression` and :t:`else expression` of an :t:`if expression`
  that is an :t:`extending expression`.

* :dp:`fls_oodpp3LpXC13`
  The :t:`expression` of an :t:`else expression` that is an :t:`extending
  expression`.

* :dp:`fls_xGThCPoTUSAi`
  The :t:`expression` of a :t:`match arm` of a :t:`match expression` that is an
  :t:`extending expression`.

* :dp:`fls_iqw0d1l1lj3i`
  The :t:`tail expression` of a :t:`block expression` that is an
  :t:`extending expression`.

:dp:`fls_aq01wjpkxhq9`
The :t:`drop scope` of the :t:`operand` of a :t:`borrow expression` that is an
:t:`extending expression` is extended to the :t:`drop scope` of the
:t:`block expression` that contains the :t:`let statement`.

:dp:`fls_VDPi1dJzJMUb`
The :t:`drop scope` of the :t:`operand` of a :t:`borrow expression`, a
:t:`dereference expression`, or a :t:`field access expression` that has an
extended :t:`drop scope` is extended to the :t:`drop scope` of the
:t:`expression`.

:dp:`fls_elGH7HSawMRb`
The :t:`drop scope` of the :t:`indexed operand` of an :t:`index expression` that
has an extended :t:`drop scope` is extended to the :t:`drop scope` of the
:t:`expression`.

.. rubric:: Examples

:dp:`fls_29y59x8bmw75`
See :p:`15.6.1. <fls_u2mzjgiwbkz0>` for the declaration of ``PrintOnDrop``.

:dp:`fls_subo2w7ln43q`
The drop scope of the temporary created for expression ``AtomicI32::new(42)`` is
extended to the drop scope of the block expression.

.. code-block:: rust

   use core::sync::atomic::AtomicI32;

   {
       let ref mut a = AtomicI32::new(42);
       println!("{}", a);
   }

.. _fls_afafmafz4hf2:

Drop Order
----------

.. rubric:: Legality Rules

:dp:`fls_n6o1xzjiz8cv`
:t:`Drop order` is the order by which :t:`[value]s` are :t:`dropped` when a
:t:`drop scope` is left.

:dp:`fls_jwofws3022ar`
When a :t:`drop scope` is left, all :t:`[value]s` associated with that
:t:`drop scope` are :t:`dropped` as follows:

* :dp:`fls_g07zq3n55094`
  :t:`[Binding]s` are :t:`dropped` in reverse declaration order.

* :dp:`fls_a5tmilqxdb6f`
  :t:`Temporaries <temporary>` are :t:`dropped` in reverse creation order.

:dp:`fls_zQGkVGWIzMQ7`
When a :t:`drop scope` of a :t:`function` is left, then each
:t:`function parameter` is :t:`dropped` from right to left as follows:

#. :dp:`fls_K8whhUwAUVqR`
   All :t:`[binding]s` introduced by the :t:`pattern` of the
   :t:`function parameter` are :t:`dropped` in reverse declaration order,

#. :dp:`fls_NpCrm4t03oQg`
   The :t:`value` of the :t:`function parameter` is :t:`dropped`.

:dp:`fls_3i348l3pbtrx`
When multiple :t:`[drop scope]s` are left at once, the :t:`[value]s` are
:t:`dropped` from the innermost :t:`drop scope` to the outermost
:t:`drop scope`.

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

.. code-block:: rust

   let a = PrintOnDrop("3");
   {
       let b = PrintOnDrop("1");
   }
   let c = PrintOnDrop("2");

