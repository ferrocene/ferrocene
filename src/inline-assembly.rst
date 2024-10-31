.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_z1il3w9nulzy:

Inline Assembly
===============

.. informational-section::

.. rubric:: Legality Rules

:dp:`fls_leamdp1r3hez`
:t:`Inline assembly` is hand-written assembly code that is integrated into a
Rust program.

:dp:`fls_3fg60jblx0xb`
:t:`Inline assembly` is written as an :t:`assembly code block` that is
wrapped inside a :t:`macro invocation` of :t:`macro` :std:`core::arch::asm` or
:t:`macro` :std:`core::arch::global_asm`.

:dp:`fls_helnk2iz8qhp`
:t:`Inline assembly` is available on the following architectures:

* :dp:`fls_vanmt2dv7hzd`
  AArch64

* :dp:`fls_g8fuy1qnebxe`
  ARM

* :dp:`fls_2n9el98anp8c`
  RISC-V

* :dp:`fls_pnoh31rvu9a6`
  x86 and x86-64

.. _fls_j9l8wn6wgm06:

Registers
---------

.. rubric:: Syntax

.. syntax::

   RegisterName ::=
       $$"$$ ExplicitRegisterName $$"$$
     | RegisterClassName

   ExplicitRegisterName ::=
       $$a$$[$$0$$-$$31$$]
     | $$ah$$    | $$al$$    | $$ax$$
     | $$b$$[$$0$$-$$31$$]
     | $$bh$$    | $$bl$$    | $$bp$$    | $$bpl$$   | $$bx$$
     | $$ch$$    | $$cl$$    | $$cx$$
     | $$d$$[$$0$$-$$31$$]
     | $$dh$$    | $$di$$    | $$dil$$   | $$dl$$    | $$dx$$
     | $$eax$$   | $$ebp$$   | $$ebx$$   | $$ecx$$   | $$edi$$   | $$edx$$   | $$eip$$   | $$esi$$
     | $$esp$$
     | $$f$$[$$0$$-$$31$$]
     | $$fa$$[$$0$$-$$7$$]
     | $$ffr$$   | $$fp$$
     | $$fs$$[$$0$$-$$11$$]
     | $$ft$$[$$0$$-$$11$$]
     | $$gp$$
     | $$h$$[$$0$$-$$31$$]
     | $$ip$$
     | $$k$$[$$0$$-$$7$$]
     | $$lr$$
     | $$m$$[$$0$$-$$7$$]
     | $$p$$[$$0$$-$$15$$]
     | $$pc$$
     | $$q$$[$$0$$-$$31$$]
     | $$r$$[$$0$$-$$15$$]
     | $$r$$[$$8$$-$$15$$]$$d$$
     | $$r$$[$$8$$-$$15$$]$$w$$
     | $$ra$$    | $$rax$$
     | $$rb$$[$$8$$-$$15$$]
     | $$rbp$$   | $$rbx$$   | $$rcx$$
     | $$rdi$$   | $$rdx$$   | $$rfp$$   | $$rip$$   | $$rsi$$   | $$rsp$$
     | $$s$$[$$0$$-$$31$$]
     | $$si$$    | $$sil$$   | $$sl$$    | $$sp$$    | $$spl$$
     | $$st($$[$$0$$-$$7$$]$$)$$
     | $$t$$[$$0$$-$$6$$]
     | $$tmm$$[$$0$$-$$7$$]
     | $$tp$$
     | $$v$$[$$0$$-$$31$$]
     | $$w$$[$$0$$-$$31$$]
     | $$wsp$$   | $$wzr$$
     | $$x$$[$$0$$-$$31$$]
     | $$xmm$$[$$0$$-$$31$$]
     | $$xzr$$
     | $$ymm$$[$$0$$-$$31$$]
     | $$zero$$
     | $$zmm$$[$$0$$-$$31$$]

.. rubric:: Legality Rules

:dp:`fls_w5a6rybvptn6`
A :t:`register` is a hardware component capable of holding data that can be
read and written.

:dp:`fls_rr8rsy7oilf0`
An :t:`input register` is a :t:`register` whose :t:`register name` is used in
a :t:`register argument` subject to :t:`direction modifier` ``in``, ``inout``,
or ``inlateout``.

:dp:`fls_5ruu8n6r9mvp`
An :t:`output register` is a :t:`register` whose :t:`register name` is
used in a :t:`register argument` subject to :t:`direction modifier` ``out``,
``lateout``, ``inout``, or ``inlateout``.

:dp:`fls_HV3Y1A2xn0zr`
A :t:`register` that is not specified as an :t:`output register` shall have the
same :t:`value` upon exit from an :t:`assembly code block` as it did upon entry
into the :t:`assembly code block`.

:dp:`fls_vesfzh8h6qzu`
A :t:`register name` is either the :t:`explicit register name` of a
:t:`register`, or the :t:`register class name` of the :t:`register class` a
:t:`register` belongs to.

:dp:`fls_ffwqxlh60i5w`
An :t:`explicit register name` is a target-specific string that identifies
a :t:`register`.

:dp:`fls_3p8akc7gcsnx`
An :t:`explicit register name` may be aliased as follows:

.. list-table::

   * - :dp:`fls_7dlx7nt77xk`
     - **Architecture**
     - **Explicit register name**
     - **Aliases**
   * - :dp:`fls_w4z7yh5qyyed`
     - AArch64
     - ``sp``
     - ``wsp``
   * - :dp:`fls_rzp8eg6z6x3q`
     - AArch64
     - ``v[0-31]``
     - ``b[0-31]``, ``d[0-31]``, ``h[0-31]``, ``q[0-31]``, ``s[0-31]``
   * - :dp:`fls_e1w41918j49`
     - AArch64
     - ``x[0-30]``
     - ``w[0-30]``
   * - :dp:`fls_q0s90h7xmnn4`
     - AArch64
     - ``x29``
     - ``fp``
   * - :dp:`fls_3pt63w76isay`
     - AArch64
     - ``x30``
     - ``lr``
   * - :dp:`fls_f3clxd3vidhh`
     - AArch64
     - ``xzr``
     - ``wzr``
   * - :dp:`fls_vyeczg1cjxys`
     - ARM
     - ``r[0-3]``
     - ``a[1-4]``
   * - :dp:`fls_h5t153uhzoq3`
     - ARM
     - ``r[4-9]``
     - ``v[1-6]``
   * - :dp:`fls_jhph577nqds1`
     - ARM
     - ``r9``
     - ``rfp``
   * - :dp:`fls_mobj1y67vxvb`
     - ARM
     - ``r10``
     - ``sl``
   * - :dp:`fls_9ke412je1hqn`
     - ARM
     - ``r11``
     - ``fp``
   * - :dp:`fls_hndlas58937e`
     - ARM
     - ``r12``
     - ``ip``
   * - :dp:`fls_5x0yvjil3z8p`
     - ARM
     - ``r13``
     - ``sp``
   * - :dp:`fls_gxvlvnqs436h`
     - ARM
     - ``r14``
     - ``lr``
   * - :dp:`fls_mra7zuu7uzmb`
     - ARM
     - ``r15``
     - ``pc``
   * - :dp:`fls_maa7w0jwvat2`
     - RISC-V
     - ``f[0-7]``
     - ``ft[0-7]``
   * - :dp:`fls_az7kcaq70h4d`
     - RISC-V
     - ``f[8-9]``
     - ``fs[0-1]``
   * - :dp:`fls_xudmsflrhvo3`
     - RISC-V
     - ``f[10-17]``
     - ``fa[0-7]``
   * - :dp:`fls_px77cr1k8coy`
     - RISC-V
     - ``f[18-27]``
     - ``fs[2-11]``
   * - :dp:`fls_y1m7tlqk2dv7`
     - RISC-V
     - ``f[28-31]``
     - ``ft[8-11]``
   * - :dp:`fls_3dqq9319okv5`
     - RISC-V
     - ``x0``
     - ``zero``
   * - :dp:`fls_5l9qo5c0gek8`
     - RISC-V
     - ``x1``
     - ``ra``
   * - :dp:`fls_1m68zqsxjuyp`
     - RISC-V
     - ``x2``
     - ``sp``
   * - :dp:`fls_bck9slu4hsn1`
     - RISC-V
     - ``x3``
     - ``gp``
   * - :dp:`fls_3x72q39c8iwt`
     - RISC-V
     - ``x4``
     - ``tp``
   * - :dp:`fls_yfbrla8c801g`
     - RISC-V
     - ``x[5-7]``
     - ``t[0-2]``
   * - :dp:`fls_3nuf1gcldamv`
     - RISC-V
     - ``x8``
     - ``fp``, ``s0``
   * - :dp:`fls_nquf1uaoezx6`
     - RISC-V
     - ``x9``
     - ``s1``
   * - :dp:`fls_91oeyxc75vu5`
     - RISC-V
     - ``x[10-17]``
     - ``a[0-7]``
   * - :dp:`fls_r5btazdpwqtw`
     - RISC-V
     - ``x[18-27]``
     - ``s[2-11]``
   * - :dp:`fls_vpibsan8aful`
     - RISC-V
     - ``x[28-31]``
     - ``t[3-6]``
   * - :dp:`fls_lj6xcaaecokk`
     - x86
     - ``ax``
     - ``eax``, ``rax``
   * - :dp:`fls_bb1qjfin4zjc`
     - x86
     - ``bp``
     - ``bpl``, ``ebp``, ``rbp``
   * - :dp:`fls_7qj6pxuq2x9e`
     - x86
     - ``bx``
     - ``ebx``, ``rbx``
   * - :dp:`fls_2xkw4nqt1s5a`
     - x86
     - ``cx``
     - ``ecx``, ``rcx``
   * - :dp:`fls_dpzi4ygox7jw`
     - x86
     - ``di``
     - ``edi``, ``rdi``
   * - :dp:`fls_yr5ztipvgezk`
     - x86
     - ``dx``
     - ``edx``, ``rdx``
   * - :dp:`fls_n8ccafjut1yd`
     - x86
     - ``ip``
     - ``eip``, ``rip``
   * - :dp:`fls_iv23mcgw6l3r`
     - x86
     - ``r[8-15]``
     - ``r[8-15]b``, ``r[8-15]d``, ``r[8-15]w``
   * - :dp:`fls_yr7bac5k3uk7`
     - x86
     - ``si``
     - ``esi``, ``rsi``
   * - :dp:`fls_gvb2zcrseqci`
     - x86
     - ``sp``
     - ``esp``, ``rsp``, ``spl``
   * - :dp:`fls_z1b9nf49nbjh`
     - x86
     - ``st(0)``
     - ``st``
   * - :dp:`fls_etfkcesnrlwt`
     - x86
     - ``xmm[0-31]``
     - ``ymm[0-31]``, ``zmm[0-31]``

:dp:`fls_8sqzva4ibf0i`
Certain :t:`[explicit register name]s` are not supported on selected
architectures, as follows:

.. list-table::

   * - :dp:`fls_8bo9p6mybuzi`
     - **Architecture**
     - **Explicit register name**
     - **Reason**
   * - :dp:`fls_dk4z9dracvps`
     - all
     - ``sp``
     - The stack pointer must be restored to its original value on exit from an
       :t:`assembly code block`.
   * - :dp:`fls_7c7lgvw8gbkb`
     - AArch64
     - ``x18``
     - OS-reserved register on some AArch64 targets.
   * - :dp:`fls_sb0ffx31gul9`
     - AArch64
     - ``x19``
     - Internally used by LLVM as a base pointer for :t:`[function]s` with
       complex stack frames.
   * - :dp:`fls_f9psgjl6ervm`
     - AArch64
     - ``x29``
     - The frame pointer cannot be used as an input or output.
   * - :dp:`fls_3y3b7znxznzu`
     - AArch64
     - ``xzr``
     - The constant zero :t:`register` cannot be modified.
   * - :dp:`fls_7y201ev5b0mq`
     - ARM
     - ``pc``
     - The program counter cannot be modified.
   * - :dp:`fls_r2aoetnwhjkf`
     - ARM
     - ``r6``
     - Internally used by LLVM as a base pointer for :t:`[function]s` with
       complex stack frames.
   * - :dp:`fls_54n4ie3frc39`
     - ARM
     - ``r7``, ``r11``
     - The frame pointer cannot be used as an input or output.
   * - :dp:`fls_iok8gc906tc8`
     - ARM
     - ``r9``
     - OS-reserved :t:`register` on some ARM targets.
   * - :dp:`fls_2mgjkyngbmbr`
     - RISC-V
     - ``gp``, ``tp``
     - Reserved :t:`[register]s`.
   * - :dp:`fls_6tlnpguf23y3`
     - RISC-V
     - ``x0``
     - The constant zero :t:`register` cannot be modified.
   * - :dp:`fls_bz5xyv89i5m7`
     - RISC-V
     - ``x8``
     - The frame pointer cannot be used as an input or output.
   * - :dp:`fls_soitzjqze3rf`
     - RISC-V
     - ``x9``
     - Internally used by LLVM as a base pointer for :t:`[function]s` with
       complex stack frames.
   * - :dp:`fls_b2c02r5y6zm9`
     - x86
     - ``bp``
     - The frame pointer cannot be used as an input or output.
   * - :dp:`fls_2ufm8y5ttcxf`
     - x86
     - ``ip``
     - The program counter cannot be modified.
   * - :dp:`fls_ub321ic94bbl`
     - x86-32
     - ``si``
     - Internally used by LLVM as a base pointer for :t:`[function]s` with
       complex stack frames.
   * - :dp:`fls_gtwmwsmyrdxe`
     - x86-64
     - ``bx``
     - Internally used by LLVM as a base pointer for :t:`[function]s` with
       complex stack frames.

:dp:`fls_vy8alu9yuza9`
It is a static error to use an unsupported :t:`explicit register name`.

.. rubric:: Undefined Behavior

:dp:`fls_zEtLZ5KjQcHS`
It is undefined behavior if a :t:`register` that is not specified as an
:t:`output register` has a different :t:`value` upon exit from an
:t:`assembly code block` from the :t:`value` it had upon entry into the
:t:`assembly code block`.

.. _fls_pz2ioqchjtym:

Register Classes
----------------

.. rubric:: Syntax

.. syntax::

   RegisterClassName ::=
       $$dreg$$
     | $$dreg_low16$$
     | $$dreg_low8$$
     | $$freg$$
     | $$kreg$$
     | $$kreg0$$
     | $$mmx_reg$$
     | $$preg$$
     | $$qreg$$
     | $$qreg_low4$$
     | $$qreg_low8$$
     | $$reg$$
     | $$reg_abcd$$
     | $$reg_byte$$
     | $$sreg$$
     | $$sreg_low16$$
     | $$tmm_reg$$
     | $$vreg$$
     | $$vreg_low16$$
     | $$x86_reg$$
     | $$xmm_reg$$
     | $$ymm_reg$$
     | $$zmm_reg$$

.. rubric:: Legality Rules

:dp:`fls_7gxb7ztpuofj`
A :t:`register class` represents a set of :t:`[register]s`.

:dp:`fls_on0i2cpk254y`
A :t:`register class name` is a target-specific string that identifies a
:t:`register class`.

:dp:`fls_40ksem5g5xx9`
:t:`[Register]s` are organized into :t:`[register class]es` as follows:

.. list-table::

   * - :dp:`fls_fqvjk6caipq`
     - **Architecture**
     - **Register class name**
     - **Explicit register names**
   * - :dp:`fls_u0ie66ep3glg`
     - AArch64
     - ``preg``
     - ``p[0-15]``, ``ffr``
   * - :dp:`fls_wcvcansd88je`
     - AArch64
     - ``reg``
     - ``x[0-30]``
   * - :dp:`fls_sbllpky4d7ka`
     - AArch64
     - ``vreg``
     - ``v[0-31]``
   * - :dp:`fls_nmx5xs829ms`
     - AArch64
     - ``vreg_low16``
     - ``v[0-15]``
   * - :dp:`fls_nars4y8tv2w6`
     - ARM / Thumb2
     - ``reg``
     - ``r[0-12]``, ``r14``
   * - :dp:`fls_b5juxguclqjs`
     - ARM / Thumb1
     - ``reg``
     - ``r[0-7]``
   * - :dp:`fls_vmigixoxm5uf`
     - ARM
     - ``dreg``
     - ``d[0-31]``
   * - :dp:`fls_vdqtbc4t69v2`
     - ARM
     - ``dreg_low8``
     - ``d[0-8]``
   * - :dp:`fls_t2d77dazjyjo`
     - ARM
     - ``dreg_low16``
     - ``d[0-15]``
   * - :dp:`fls_jh02uk3ypett`
     - ARM
     - ``qreg``
     - ``q[0-15]``
   * - :dp:`fls_rjk5laiyqagy`
     - ARM
     - ``qreg_low4``
     - ``q[0-3]``
   * - :dp:`fls_26bq6wbwznx`
     - ARM
     - ``qreg_low8``
     - ``q[0-8]``
   * - :dp:`fls_6d25i0lkzd7u`
     - ARM
     - ``sreg``
     - ``s[0-31]``
   * - :dp:`fls_y52suhleyid2`
     - ARM
     - ``sreg_low16``
     - ``s[0-15]``
   * - :dp:`fls_7b7c8xtm8fr7`
     - RISC-V
     - ``freg``
     - ``f[0-31]``
   * - :dp:`fls_ue0se3dcop6w`
     - RISC-V
     - ``reg``
     - ``x1``, ``x[0-7]``, ``x[9-15]``, ``x[16-31]`` (on non-RV32E)
   * - :dp:`fls_2m49cyfqffvo`
     - RISC-V
     - ``vreg``
     - ``v[0-31]``
   * - :dp:`fls_mj1t0f9lp6v8`
     - x86
     - ``kreg``
     - ``k[1-7]``
   * - :dp:`fls_tpkkubhjt7lk`
     - x86
     - ``kreg0``
     - ``k0``
   * - :dp:`fls_ivq874v4lmga`
     - x86
     - ``mmx_reg``
     - ``mm[0-7]``
   * - :dp:`fls_2wdcrocczwyi`
     - x86
     - ``reg``
     - ``ax``, ``bp``, ``bx``, ``cx``, ``di``, ``dx``, ``r[8-15]``, ``si``
   * - :dp:`fls_v04te7p28dth`
     - x86
     - ``reg_abcd``
     - ``ax``, ``bx``, ``cx``, ``dx``
   * - :dp:`fls_uypct69j2h6a`
     - x86
     - ``x87_reg``
     - ``st([0-7])``
   * - :dp:`fls_gyet9huf6nr`
     - x86
     - ``xmm_reg``
     - ``xmm[0-7]``, ``xmm[8-15]`` (on 64bit only)
   * - :dp:`fls_5ekbq9hacho9`
     - x86
     - ``ymm_reg``
     - ``ymm[0-7]``, ``ymm[8-15]`` (on 64bit only)
   * - :dp:`fls_furnyxmwqn09`
     - x86
     - ``zmm_reg``
     - ``zmm[0-7]``, ``zmm[8-15]`` (on 64bit only)
   * - :dp:`fls_1c4ts991vkpq`
     - x86-32
     - ``reg_byte``
     - ``ah``, ``al``, ``bh``, ``bl``, ``ch``, ``cl``, ``dh``, ``dl``
   * - :dp:`fls_iwnb72jb9iwj`
     - x86-64
     - ``reg_byte``
     - ``al``, ``bl``,  ``bpl``, ``cl``, ``dil``, ``dl``, ``sil``, ``r[8-15]b``
   * - :dp:`fls_mw3axoixjgnq`
     - x86-64
     - ``tmm_reg``
     - ``tmm[0-7]``

:dp:`fls_mnzt6bxhycv9`
If a :t:`value` has a smaller size than the :t:`register` it is allocated in,
then

* :dp:`fls_drg7v8hxb5ca`
  On RISC-V architectures, if the :t:`register` belongs to :t:`register class`
  ``freg``, then :c:`f32` :t:`[value]s` are :t:`NaN-boxed <NaN-boxing>`. in a
  :c:`f64` :t:`value`.

* :dp:`fls_78gb8z1fyluc`
  Otherwise, for an :t:`input register`, the upper bits of the :t:`register`
  have an undefined :t:`value`.

* :dp:`fls_7dii7lee457t`
  Otherwise, for an :t:`output register`, the upper bits are ignored.

:dp:`fls_ujhjocg1361b`
If a :t:`register argument` has :t:`direction modifier` ``inout`` and an
:t:`input-output register expression`, then the :t:`input register expression`
and the :t:`output register expression` shall have the same :t:`type`.

.. _fls_hejgghwzblf:

Register Arguments
------------------

.. rubric:: Syntax

.. syntax::

   RegisterArgument ::=
       (Identifier $$=$$)? DirectionModifier $$($$ RegisterName $$)$$ RegisterExpression

   DirectionModifier ::=
       $$in$$
     | $$inlateout$$
     | $$inout$$
     | $$lateout$$
     | $$out$$

   RegisterExpression ::=
       InputOutputRegisterExpression
     | SimpleRegisterExpression
     | ConstRegisterExpression

   InputOutputRegisterExpression ::=
       InputRegisterExpression $$=>$$ OutputRegisterExpression
     | InputRegisterExpression $$=>$$ UnderscoreExpression

   InputRegisterExpression ::=
       Expression

   OutputRegisterExpression ::=
       Expression

   SimpleRegisterExpression ::=
       Expression
     | UnderscoreExpression

   ConstRegisterExpression ::=
       $$const$$ Expression

.. rubric:: Legality Rules

:dp:`fls_455dmnp4cxqv`
A :t:`register argument` is a :t:`construct` that configures the input
and output of a :t:`register`, and optionally binds the configuration to an
:t:`identifier`.

:dp:`fls_6bv3s8be5xif`
A :t:`register argument` shall be used within an :t:`assembly instruction`.

:dp:`fls_uddjvkz4g899`
A :t:`named register argument` is a :t:`register argument` whose configuration
is bound to an :t:`identifier`.

:dp:`fls_sqs5to20p0te`
A :t:`positional register argument` is a :t:`register argument` whose
configuration is not bound to an :t:`identifier`.

:dp:`fls_dzlycyk24euk`
A :t:`named register argument` shall appear after a
:t:`positional register argument`.

:dp:`fls_ics6gdzww1p`
An :t:`explicit register argument` is a :t:`register argument` that uses an
:t:`explicit register name`.

:dp:`fls_mmc1w8jjr55r`
An :t:`explicit register argument` shall appear after a
:t:`named register argument`.

:dp:`fls_9hhtcey2d4t6`
A :t:`register class argument` is a :t:`register argument` that uses a
:t:`register class name`.

:dp:`fls_8aynifgq02gt`
A :t:`register class argument` causes an assembler to select a suitable
:t:`register` from the related :t:`register class`.

:dp:`fls_5a3vfresnv5z`
A :t:`direction modifier` is a :t:`construct` that indicates whether a
:t:`register argument` initializes a :t:`register`, assigns the :t:`value` of a
:t:`register` to an :t:`expression`, or both.

:dp:`fls_fta1gb5tzi3a`
An :t:`input register expression` is an :t:`expression` that provides the
initial :t:`value` of a :t:`register`.

:dp:`fls_sopiivuae0x7`
An :t:`output register expression` is an :t:`expression` that is assigned the
:t:`value` of a :t:`register`.

:dp:`fls_81Ju1TEqJ48K`
A :dt:`const register expression` is an :t:`expression` that is evaluated at compile-time.

:dp:`fls_kkrcyk96w8x1`
An :t:`input-output register expression` is a :t:`construct` that specifies
both an :t:`input register expression` and an :t:`output register expression`.

:dp:`fls_aniw4ehsn2kb`
A :t:`simple register expression` is either an :t:`expression` or an
:t:`underscore expression`.

:dp:`fls_j9XOoXDmN5Dq`
A :t:`register expression` is either an :t:`input-output register expression`, a :t:`simple register expression` or a :t:`const register expression`.

:dp:`fls_jU8zg4k8dFsY`
The :t:`type` of a :t:`const register expression` shall be an :t:`integer type`.

:dp:`fls_66owmltvhnu4`
The :t:`type` of an :t:`input register expression`,
:t:`output register expression`, or :t:`simple register expression` shall
depend on the architecture and the target feature in effect, as follows:

.. list-table::

   * - :dp:`fls_72p8e4bo6pns`
     - **Architecture**
     - **Register class name**
     - **Target feature**
     - **Allowed types**
   * - :dp:`fls_z0dbmmp5yblf`
     - AArch64
     - ``preg``
     - n/a
     - Only clobbers
   * - :dp:`fls_4jdnt8uap95i`
     - AArch64
     - ``reg``
     - none
     - :c:`i8`, :c:`i16`, :c:`i32`, :c:`f32`, :c:`i64`, :c:`f64`
   * - :dp:`fls_wd2hzsbzdg2y`
     - AArch64
     - ``vreg``
     - ``neon``
     - :c:`i8`, :c:`i16`, :c:`i32`, :c:`f32`, :c:`i64`, :c:`f64`
   * - :dp:`fls_sqy00lg5j7c6`
     - ARM
     - ``dreg``
     - ``vfp2``
     - :c:`i64`, :c:`f64`
   * - :dp:`fls_vxba1ttvz6hh`
     - ARM
     - ``reg``
     - none
     - :c:`i8`, :c:`i16`, :c:`i32`, :c:`f32`
   * - :dp:`fls_xkbnla2avrn0`
     - ARM
     - ``sreg``
     - ``vfp2``
     - :c:`i32`, :c:`f32`
   * - :dp:`fls_w6jhcv616l9o`
     - RISC-V
     - ``freg``
     - ``f``
     - :c:`f32`
   * - :dp:`fls_xweobiwapog1`
     - RISC-V
     - ``freg``
     - ``d``
     - :c:`f64`
   * - :dp:`fls_4matyejw6cls`
     - RISC-V
     - ``vreg``
     - n/a
     - Only clobbers
   * - :dp:`fls_hklqabav1jju`
     - RISC-V32
     - ``reg``
     - none
     - :c:`i8`, :c:`i16`, :c:`i32`, :c:`f32`
   * - :dp:`fls_nq22h8gragil`
     - RISC-V64
     - ``reg``
     - none
     - :c:`i8`, :c:`i16`, :c:`i32`, :c:`f32`, :c:`i64`, :c:`f64`
   * - :dp:`fls_uxgcrs57bznk`
     - x86
     - ``kreg``
     - ``avx512f``
     - :c:`i8`, :c:`i16`
   * - :dp:`fls_ym05938ejwng`
     - x86
     - ``kreg``
     - ``avx512bw``
     - :c:`i32`, :c:`i64`
   * - :dp:`fls_5l77g8h8et2o`
     - x86
     - ``mmx_reg``
     - n/a
     - Only clobbers
   * - :dp:`fls_xlcliuums5b0`
     - x86
     - ``reg_byte``
     - none
     - :c:`i8`
   * - :dp:`fls_5p4hyl7mxgai`
     - x86
     - ``tmm_reg``
     - n/a
     - Only clobbers
   * - :dp:`fls_ilepg263w5o7`
     - x86
     - ``x87_reg``
     - n/a
     - Only clobbers
   * - :dp:`fls_tubmavru8wvn`
     - x86
     - ``xmm_reg``
     - ``sse``
     - :c:`i32`, :c:`f32`, :c:`i64`, :c:`f64`
   * - :dp:`fls_b1xi3u9k4pdl`
     - x86
     - ``ymm_reg``
     - ``avx``
     - :c:`i32`, :c:`f32`, :c:`i64`, :c:`f64`
   * - :dp:`fls_i9ds6724tv20`
     - x86
     - ``zmm_reg``
     - ``avx512f``
     - :c:`i32`, :c:`f32`, :c:`i64`, :c:`f64`
   * - :dp:`fls_trldyekxxlzx`
     - x86-32
     - ``reg``
     - none
     - :c:`i16`, :c:`i32`, :c:`f32`
   * - :dp:`fls_efmpbyi4qjmf`
     - x86-64
     - ``reg``
     - none
     - :c:`i16`, :c:`i32`, :c:`f32`, :c:`i64`, :c:`f64`

:dp:`fls_4x3w50w7qm8w`
If a :t:`register argument` has :t:`direction modifier` ``in`` and a
:t:`simple register expression`, then

* :dp:`fls_6cne58tlquze`
  Upon entry of an :t:`assembly code block`, the :t:`register` contains the
  :t:`value` of the :t:`simple register expression`.

* :dp:`fls_5w718fne9jsh`
  On exit from an :t:`assembly code block`, the :t:`register` shall contain
  the same :t:`value`, unless the :t:`register` is subject to
  :t:`direction modifier` ``lateout``.

:dp:`fls_tel7kogaqytg`
If a :t:`register argument` has :t:`direction modifier` ``out`` and a
:t:`simple register expression`, then

* :dp:`fls_aw61psz5drg8`
  Upon entry of an :t:`assembly code block`, the :t:`register` contains an
  undefined :t:`value`.

* :dp:`fls_sv2x3x81b32j`
  On exit from an :t:`assembly code block`, the :t:`value` of the :t:`register`
  is assigned to the :t:`simple register expression`. The
  :t:`simple register expression` shall be a :t:`place expression`.

* :dp:`fls_nebb0nhxf5ix`
  If the :t:`simple register expression` is an :t:`underscore expression`, then
  the :t:`value` of the :t:`register` is discarded.

:dp:`fls_j0pxc8g8kcxm`
If a :t:`register argument` has :t:`direction modifier` ``lateout`` and a
:t:`simple register expression`, then the :t:`register argument` behaves as a
:t:`register argument` with :t:`direction modifier` ``out``, except that the
:t:`register` can be reused with :t:`direction modifier` ``in``.

:dp:`fls_wwh6xyclxwqj`
If a :t:`register argument` has :t:`direction modifier` ``inout`` and a
:t:`simple register expression`, then

* :dp:`fls_qcb47z1ap9dz`
  Upon entry of an :t:`assembly code block`, the :t:`register` contains the
  :t:`value` of the :t:`simple register expression`.

* :dp:`fls_h01au4vk8mjd`
  On exit from an :t:`assembly code block`, the :t:`value` of the :t:`register`
  is assigned to the place indicated by the :t:`simple register expression`.
  The :t:`simple register expression` shall be a :t:`mutable place expression`.

:dp:`fls_92ijsf4p6yn`
If a :t:`register argument` has :t:`direction modifier` ``inout`` and an
:t:`input-output register expression`, then

* :dp:`fls_xkui7j3gnfg0`
  Upon entry of an :t:`assembly code block`, the :t:`register` contains the
  :t:`value` of the :t:`input register expression`.

* :dp:`fls_eahyqniqs2pn`
  On exit from an :t:`assembly code block`, the :t:`value` of the :t:`register`
  is assigned to the place indicated by the :t:`output register expression`.
  The :t:`output register expression` shall be a :t:`place expression`.

* :dp:`fls_5g7p2zo07gfe`
  If the :t:`output register expression` is an :t:`underscore expression`, then
  the :t:`value` of the :t:`register` is discarded.

:dp:`fls_dobbatnjs0yt`
If a :t:`register argument` has :t:`direction modifier` ``inlateout`` and a
:t:`simple register expression`, then the :t:`register argument` behaves as a
:t:`register argument` with :t:`direction modifier` ``inout``, except that the
:t:`register` can be reused with :t:`direction modifier` ``in``.

:dp:`fls_ax8t4uta34ym`
If a :t:`register argument` has :t:`direction modifier` ``inlateout`` and an
:t:`input-output register expression`, then the :t:`register argument` behaves
as a :t:`register argument` with :t:`direction modifier` ``inout``, except that
the :t:`register` can be reused with :t:`direction modifier` ``in``.

:dp:`fls_dvft4ha00wj3`
It is a static error to specify a :t:`register argument` with
:t:`direction modifier` and :t:`register expression` other than the
combinations listed above.

.. rubric:: Dynamic Semantics

:dp:`fls_2ekwpx2bwj1b`
The :t:`evaluation` of a :t:`register argument` proceeds as follows:

* :dp:`fls_3s2n9dlrlhz9`
  If a :t:`register argument` has an :t:`input-output register expression`,
  then

  #. :dp:`fls_nbkkz6krcngi`
     The :t:`input register expression` is evaluated.

  #. :dp:`fls_utrvenwrettz`
     The :t:`output register expression` is evaluated.

* :dp:`fls_n85sjh925x`
  If a :t:`register argument` has a :t:`simple register expression`, then the
  :t:`simple register expression` is evaluated.

.. rubric:: Examples

.. code-block:: rust

   let mut left_value: i32 = 1;
   let right_value: i32 = 2;

   unsafe {
       asm!(
           "add {left} {right}",
           left = inout(reg) left_value,
           right = in(reg) right_value,
       );
   }

.. _fls_e0896uk0mdyl:

Assembly Instructions
---------------------

.. rubric:: Syntax

.. syntax::

   AssemblyCodeBlock ::=
       AssemblyInstruction ($$,$$ AssemblyInstruction)*

   AssemblyInstruction ::=
       StringLiteral

.. rubric:: Legality Rules

:dp:`fls_4jr7eg6e0g4w`
An :t:`assembly instruction` is a :t:`string literal` that represents a
low-level assembly operation or an :t:`assembly directive`.

:dp:`fls_ihjhpy4osl53`
An :t:`assembly instruction` shall use the syntax of format strings as
defined in :t:`module` :std:`std::fmt`, and contain zero or more
:t:`[register parameter]s`.

:dp:`fls_2d05gcixjrzt`
An :t:`assembly code block` is a sequence of :t:`[assembly instruction]s`.

:dp:`fls_z64f094aivp6`
When an :t:`assembly code block` contains multiple :t:`[assembly instruction]s`,
the :t:`[assembly instruction]s` are treated as concatenated into a single
:t:`string literal`, with character 0x0A (new line) between them.

:dp:`fls_u8lifqig90gq`
The set of memory locations that an :t:`assembly code block` is allowed to
read and write are the same as those for an :t:`external function`, excluding
the memory locations that are private to the :t:`assembly code block`.

:dp:`fls_lfeun3er5sc9`
A tool is not required to guarantee that an :t:`assembly code block` appears
exactly once in the final assembly output.

:dp:`fls_mmdmymljq8a3`
A tool is not required to guarantee that two :t:`[assembly code block]s`
appear in the same declarative order in the final assembly output, or appear
contiguously in successive addresses.

:dp:`fls_xugsn2ghh73c`
A :t:`register parameter` is a substring delimited by characters 0x7B (left
curly bracket) and 0x7D (right curly bracket) that is substituted with a
:t:`register argument` in an :t:`assembly instruction`.

:dp:`fls_opnxq5kyw9jo`
On x86 architectures, direction flag ``DF`` in :t:`register` ``EFLAGS`` shall
be cleared on exit from an :t:`assembly code block`.

.. rubric:: Undefined Behavior

:dp:`fls_wydu9yft7a3r`
On x86 architectures, it is undefined behavior if direction flag ``DF`` in
:t:`register` ``EFLAGS`` remains set on exit from an :t:`assembly code block`.

.. rubric:: Examples

.. code-block:: rust

   "shl {value} 2"

.. _fls_lv19xysy1f7e:

Register Parameter Modifiers
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_2xilifichdqu`
A :t:`register parameter modifier` is a substring that starts with character
0x3A (colon), follows a :t:`register parameter`, and changes the formatting of
the related :t:`register parameter`.

:dp:`fls_o3fx9397ib74`
The effects of a :t:`register parameter modifier` depends on the architecture
and :t:`register class`, as follows:

.. list-table::

   * - :dp:`fls_3atrad53m22a`
     - **Architecture**
     - **Register class**
     - **Modifier**
     - **Example output**
   * - :dp:`fls_5zaqgz9jc8gy`
     - AArch64
     - ``reg``
     - none
     - ``x0``
   * - :dp:`fls_erjczjotwqc3`
     - AArch64
     - ``reg``
     - **:w**
     - ``w0``
   * - :dp:`fls_z8m45i9xqkct`
     - AArch64
     - ``reg``
     - **:x**
     - ``x0``
   * - :dp:`fls_adaavz3olha3`
     - AArch64
     - ``vreg``
     - none
     - ``v0``
   * - :dp:`fls_u5dzca5f2pfm`
     - AArch64
     - ``vreg``
     - **:b**
     - ``b0``
   * - :dp:`fls_6542a2hi5yhc`
     - AArch64
     - ``vreg``
     - **:d**
     - ``d0``
   * - :dp:`fls_94thqwabspw2`
     - AArch64
     - ``vreg``
     - **:h**
     - ``h0``
   * - :dp:`fls_xhp8xu8xvvgd`
     - AArch64
     - ``vreg``
     - **:q**
     - ``q0``
   * - :dp:`fls_g0p2ebuffnxb`
     - AArch64
     - ``vreg``
     - **:s**
     - ``s0``
   * - :dp:`fls_d0e2weni8q87`
     - AArch64
     - ``vreg``
     - **:v**
     - ``v0``
   * - :dp:`fls_bq3p4k42tzh8`
     - ARM
     - ``dreg``
     - none
     - ``d0``
   * - :dp:`fls_l5mkfdot97fz`
     - ARM
     - ``qreg``
     - none
     - ``q0``
   * - :dp:`fls_sarl1hoq0lco`
     - ARM
     - ``qreg``
     - **:e**, **:f**
     - ``d0``, ``d1``
   * - :dp:`fls_f4q9a0jrs4ek`
     - ARM
     - ``reg``
     - none
     - ``r0``
   * - :dp:`fls_w0hn6vkpuvpc`
     - ARM
     - ``sreg``
     - none
     - ``s0``
   * - :dp:`fls_a7bvkyh5otx5`
     - RISC-V
     - ``freg``
     - none
     - ``f0``
   * - :dp:`fls_klthi4cczkem`
     - RISC-V
     - ``reg``
     - none
     - ``x1``
   * - :dp:`fls_pizgnxvcnj46`
     - x86
     - ``kreg``
     - none
     - ``k1``
   * - :dp:`fls_pvzfiuxka6wg`
     - x86
     - ``reg``
     - **:e**
     - ``eax``
   * - :dp:`fls_pf8yj3m81wk4`
     - x86
     - ``reg``
     - **:x**
     - ``ax``
   * - :dp:`fls_2bf3ady2idq1`
     - x86
     - ``reg_abcd``
     - **:h**
     - ``ah``
   * - :dp:`fls_x1se4r75v58o`
     - x86
     - ``reg_byte``
     - none
     - ``al`` / ``ah``
   * - :dp:`fls_rgovn5r3caif`
     - x86
     - ``xmm_reg``
     - none
     - ``xmm0``
   * - :dp:`fls_faxm8xkhruvz`
     - x86
     - ``ymm_reg``
     - none
     - ``ymm0``
   * - :dp:`fls_x1kkkvugpkyd`
     - x86
     - ``zmm_reg``
     - none
     - ``zmm0``
   * - :dp:`fls_uonqmj16oqxe`
     - x86
     - ``xmm_reg``,  ``ymm_reg``,  ``zmm_reg``
     - **:x**
     - ``xmm0``
   * - :dp:`fls_ydhwlp56vmrz`
     - x86
     - ``xmm_reg``,  ``ymm_reg``,  ``zmm_reg``
     - **:y**
     - ``ymm0``
   * - :dp:`fls_yolqzfqbfjoh`
     - x86
     - ``xmm_reg``,  ``ymm_reg``,  ``zmm_reg``
     - **:z**
     - ``zmm0``
   * - :dp:`fls_ojzzhoed6t9l`
     - x86-32
     - ``reg``
     - none
     - ``eax``
   * - :dp:`fls_gw56ok8llid3`
     - x86-32
     - ``reg_abcd``
     - **:l**
     - ``al``
   * - :dp:`fls_ry7qoosmjrev`
     - x86-64
     - ``reg``
     - none
     - ``rax``
   * - :dp:`fls_i0ax45x2wskd`
     - x86-64
     - ``reg``
     - **:l**
     - ``al``
   * - :dp:`fls_op4dx1rqwhsf`
     - x86-64
     - ``reg``
     - **:r**
     - ``rax``

.. rubric:: Examples

.. code-block:: rust

   let mut left_value: i32 = 1;
   let right_value: i32 = 2;

   unsafe {
       asm!(
           "add {left}:e {right}:x",
           left = inout(reg) left_value,
           right = in(reg) right_value,
       );
   }

.. _fls_6momhvgx4w21:

Directive Support
~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_4tfod2vgz2m6`
An :t:`assembly directive` is a request to the assembler to perform a
particular action or change a setting.

:dp:`fls_3b0ab1nlo641`
If an :t:`assembly code block` contains stateful :t:`[assembly directive]s`
that modify how subsequent assembly code is processed, then the
:t:`assembly code block` shall undo the effects of the stateful
:t:`[assembly directive]s` before the :t:`assembly code block` is completed.

:dp:`fls_caqznttql5p8`
The common :t:`[assembly directive]s` are as follows:

#. :dp:`fls_bcheqswo7a1`
   ``.2byte``, ``.4byte``, ``.8byte``

#. :dp:`fls_qxcl999rdwam`
   ``.align``, ``.ascii``, ``.asciz``, ``.alt_entry``

#. :dp:`fls_2yi7kjnhkfme`
   ``.balign``, ``.balignl``, ``.balignw``, ``.balign``, ``.balignl``,
   ``.balignw``, ``.bss``, ``.byte``

#. :dp:`fls_q0jp60aj81nv`
   ``.comm``

#. :dp:`fls_akny3esj88yy`
   ``.data``, ``.def``, ``.double``

#. :dp:`fls_9jajt7jn9cxk`
   ``.endef``, ``.equ``, ``.equiv``, ``.eqv``

#. :dp:`fls_i7dr87fyrei8`
   ``.fill``, ``.float``

#. :dp:`fls_rjzgpxt8z8x`
   ``.globl``, ``.global``

#. :dp:`fls_iqrjkvgae5k`
   ``.inst``

#. :dp:`fls_tzb5diegx3d5`
   ``.lcomm``, ``.long``

#. :dp:`fls_82nia9oagat`
   ``.octa``, ``.option``

#. :dp:`fls_qg6wt4plwnw6`
   ``.private_extern``, ``.p2align``, ``.pushsection``, ``.popsection``

#. :dp:`fls_8ci8ukk25nz3`
   ``.quad``

#. :dp:`fls_ysbaz052rjg4`
   ``.scl``, ``.section``, ``.set``, ``.short``, ``.size``, ``.skip``,
   ``.sleb128``, ``.space``, ``.string``

#. :dp:`fls_lbazk0g9r350`
   ``.text``, ``.type``

#. :dp:`fls_2dui79hn30o7`
   ``.uleb128``

#. :dp:`fls_qzwyjj6xxwc2`
   ``.word``

:dp:`fls_9flwdfh5crsk`
The following :t:`[assembly directive]s` are relevant on ELF targets that
support DWARF unwind info.

#. :dp:`fls_u1c09ssrllil`
   ``.cfi_adjust_cfa_offset``

#. :dp:`fls_tx58qbvh3jz3`
   ``.cfi_def_cfa``, ``.cfi_def_cfa_offset``, ``.cfi_def_cfa_register``

#. :dp:`fls_anwe21ypcjws`
   ``.cfi_endproc``, ``.cfi_escape``

#. :dp:`fls_43rchr5ffxsv`
   ``.cfi_lsda``

#. :dp:`fls_rscmbo3kbrsm`
   ``.cfi_offset``

#. :dp:`fls_hnol9houwn1f`
   ``.cfi_personality``

#. :dp:`fls_es1lo6siw702`
   ``.cfi_register``, ``.cfi_rel_offset``, ``.cfi_remember_state``,
   ``.cfi_restore``, ``.cfi_restore_state``, ``.cfi_return_column``

#. :dp:`fls_xlk7kd26j2rm`
   ``.cfi_same_value``, ``.cfi_sections``, ``.cfi_signal_frame``,
   ``.cfi_startproc``

#. :dp:`fls_x9kaplz9g1z9`
   ``.cfi_undefined``

#. :dp:`fls_928ermlgde11`
   ``.cfi_window_save``

:dp:`fls_49bkqmxwl0d2`
The following :t:`[assembly directive]s` are relevant on targets with
structured exception handling.

#. :dp:`fls_xlvkpe975b58`
   ``.seh_endproc``, ``.seh_endprologue``

#. :dp:`fls_k3sy1ph0kvy`
   ``.seh_proc``, ``.seh_pushreg``

#. :dp:`fls_ku6noqc0poxq`
   ``.seh_savereg``, ``.seh_setframe``, ``.seh_stackalloc``

:dp:`fls_hny0patop479`
The following :t:`[assembly directive]s` are relevant on ARM targets.

#. :dp:`fls_jm61m237cww`
   ``.code``

#. :dp:`fls_wvje5eua16xm`
   ``.even``

#. :dp:`fls_frxn2f6v584d`
   ``.fnstart``, ``.fnend``

#. :dp:`fls_x6azw3td92b3`
   ``.movsp``

#. :dp:`fls_twerrggztho5`
   ``.save``

#. :dp:`fls_s4cbxrc4ijyp`
   ``.thumb``, ``.thumb_func``

:dp:`fls_2hdsgqko25l5`
The following :t:`[assembly directive]s` are relevant on x86 targets.

#. :dp:`fls_5tzwwove8mgq`
   ``.code16``, ``.code32``, ``.code64``

#. :dp:`fls_e6nvq9xygvh`
   ``.nops``

.. _fls_a3joqzqp1v9d:

ABI Clobbers
------------

.. rubric:: Syntax

.. syntax::

   AbiClobber ::=
       $$clobber_abi$$ $$($$ AbiKindList $$)$$

   AbiKindList ::=
       AbiKind ($$,$$ AbiKind)* $$,$$?

.. rubric:: Legality Rules

:dp:`fls_xa11ggykg0sh`
An :t:`ABI clobber` is an argument to :t:`macro` :std:`core::arch::asm` which
indicates that the :t:`[value]s` of selected :t:`[register]s` might be
overwritten during the :t:`execution` of an :t:`assembly code block`.

:dp:`fls_e43sj9inlsym`
Multiple :t:`[ABI clobber]s` may be specified for an :t:`assembly code block`.
Clobber constraints are applied for all unique :t:`[register]s` in the union of
all specified :t:`[ABI]s`.

:dp:`fls_gq2khxl1hixg`
The effects of an :t:`ABI clobber` depend on the :t:`ABI` in effect, as follows:

.. list-table::

   * - :dp:`fls_o2qn842y0vvc`
     - **Architecture**
     - :s:`AbiKind`
     - **Clobbered registers**
   * - :dp:`fls_msysjt5m2941`
     - AArch64
     - ``"C"``, ``"efiapi"``, ``"system"``
     - ``ffr``, ``p[0-15]``, ``v[0-31]``, ``x[0-17]``, ``x18`` (when not
       reserved), ``x30``
   * - :dp:`fls_vyhl5po6pl4x`
     - ARM
     - ``"aapcs"``, ``"C"``, ``"efiapi"``, ``"system"``
     - ``d[0-7]``, ``d[16-31]``, ``r[0-3]``, ``r12``, ``r14``, ``s[0-15]``
   * - :dp:`fls_d1be48ik4a8`
     - RISC-V
     - ``"C"``, ``"efiapi"``, ``"system"``
     - ``f[0-7]``, ``f[10-17]``, ``f[28-31]``, ``v[0-31]``, ``x1``, ``x[5-7]``,
       ``x[10-17]``, ``x[28-31]``
   * - :dp:`fls_49pus6qqmf72`
     - x86-32
     - ``"C"``, ``"cdecl"``, ``"efiapi"``, ``"fastcall"``, ``"stdcall"``,
       ``"system"``
     - ``ax``, ``cx``, ``dx``, ``k[0-7]``, ``mm[0-7]``, ``st([0-7])``,
       ``xmm[0-7]``
   * - :dp:`fls_tc727ietnawz`
     - x86-64
     - ``"C"``, ``"efiapi"``, ``"system"`` (on Windows), ``"win64"``
     - ``ax``, ``cx``, ``dx``, ``k[0-7]``, ``mm[0-7]``, ``st([0-7])``,
       ``r[8-11]``, ``tmm[0-7]``, ``xmm[0-31]``
   * - :dp:`fls_6jgsmfvww667`
     - x86-64
     - ``"C"``, ``"system"`` (on non-Windows), ``"sysv64"``
     - ``ax``, ``cx``, ``di``, ``dx``, ``k[0-7]``, ``mm[0-7]``, ``r[8-11]``,
       ``si``, ``st([0-7])``, ``tmm[0-7]``, ``xmm[0-31]``

:dp:`fls_gvzoq5mqwjx`
On x86 architectures, the x87 floating-point :t:`register` stack shall remain
unchanged unless all ``st(``\ [``0``-``7``]\ ``)`` :t:`[register]s` have been
clobbered.

:dp:`fls_bnwzzpcmiero`
On x86 architectures, if all x87 :t:`[register]s` are clobbered, then the x87
:t:`register` stack is presumed empty upon entry of an :t:`assembly code block`.
The x87 :t:`register` stack shall be empty on exit from an
:t:`assembly code block`.

.. rubric:: Examples

.. code-block:: rust

   clobber_abi("C", "system")

.. _fls_ylli0ortyegk:

Assembly Options
----------------

.. rubric:: Syntax

.. syntax::

   AssemblyOption ::=
       $$options$$ $$($$ OptionList $$)$$

   OptionList ::=
       Option ($$,$$ Option)* $$,$$?

   Option ::=
       $$att_syntax$$
     | $$nomem$$
     | $$noreturn$$
     | $$nostack$$
     | $$preserves_flags$$
     | $$pure$$
     | $$raw$$
     | $$readonly$$

.. rubric:: Legality Rules

:dp:`fls_i21l6t3vn95t`
An :t:`assembly option` is used to specify a characteristic of or a restriction
on the related :t:`assembly code block`.

:dp:`fls_g09kmp2a04g9`
:t:`Assembly option` ``att_syntax`` is applicable only to x86 architectures
and causes the assembler to use the ``.att_syntax`` prefix mode which prefixes
:t:`[register]s` with ``%``.

:dp:`fls_quer8ltdwnf2`
:t:`Assembly option` ``nomem`` indicates that the :t:`assembly code block` does
not read or write memory.

:dp:`fls_5wpgqpcm1v40`
:t:`Assembly option` ``noreturn`` indicates that the :t:`assembly code block`
does not return, preventing the :t:`dropping` of :t:`[variable]s`.

:dp:`fls_ejuap3kkvs57`
:t:`Assembly option` ``nostack`` indicates that the :t:`assembly code block`
does not push on the stack, or write to the stack red-zone (if supported).

:dp:`fls_1nopbk5bkeqm`
If :t:`assembly option` ``nostack`` is not in effect, then an
:t:`assembly code block` is allowed to use stack space below the stack pointer.
Upon entry of an :t:`assembly code block`, the stack pointer is suitably
aligned according to the target :t:`ABI` for :t:`[call expression]s`. The stack
pointer shall be restored to its original :t:`value` on exit from the
:t:`assembly code block`.

:dp:`fls_e5b1mp3byll2`
:t:`Assembly option` ``preserves_flags`` indicates that the
:t:`assembly code block` does not modify the flags :t:`register`.

:dp:`fls_2gf4wemrzaae`
If :t:`assembly option` ``preserves_flags`` is in effect, then the :t:`[value]s`
of the following flags :t:`[register]s` shall be restored on exit from an
:t:`assembly code block`:

.. list-table::

   * - :dp:`fls_5ebifab8dhy`
     - **Architecture**
     - **Flag registers**
   * - :dp:`fls_ae2x4ho3i0zr`
     - AArch64
     - Floating-point status ``FPRS`` :t:`register`

       Condition flags ``NZCV`` :t:`register`
   * - :dp:`fls_188ib65a1z36`
     - ARM
     - Condition flags ``C``, ``N``, ``V``, ``Z`` in :t:`register` ``CPSR``

       Condition flags ``C``, ``N``, ``V``, ``Z`` in :t:`register` ``FPSCR``

       Floating-point exception flags ``DZC``, ``IDC``, ``IOC``, ``IXC``,
       ``OFC``, ``UFC`` in :t:`register` ``FPSCR``

       Greater than or equal flag ``GE`` in :t:`register` ``CPRS``

       Saturation flag ``Q`` in :t:`register` ``CPRS``

       Saturation flag ``QC`` in :t:`register` ``FPSCR``
   * - :dp:`fls_ia3cg424d601`
     - RISC-V
     - Floating-point exception flags ``fflags`` in :t:`register` ``fcsr``

       Vector extension state ``vcsr``, ``vl``, ``vtype``
   * - :dp:`fls_j09bo53i5n69`
     - x86
     - Status flags ``AF``, ``CF``, ``OF``, ``PF``, ``SF``, ``ZF`` in
       :t:`register` ``EFLAGS``

       Floating-point exception flags ``DE``, ``IE``, ``OE``, ``PE``, ``UE``,
       ``ZE`` in :t:`register` ``MXCSR``

       Floating-point status word

:dp:`fls_eka6chp3hapa`
:t:`Assembly option` ``pure`` indicates that the :t:`assembly code block` has no
side effects, and its outputs depend only on direct inputs.

:dp:`fls_nszx1gllufi2`
:t:`Assembly option` ``raw`` causes :t:`[assembly instruction]s` to be parsed
raw, without any special handling of :t:`[register parameter]s`.

:dp:`fls_d169ppna563c`
:t:`Assembly option` ``readonly`` indicates that the :t:`assembly code block`
does not write memory.

:dp:`fls_h8549stij7pj`
:t:`[Assembly option]s` ``att_syntax`` and ``raw`` shall appear only in
:s:`GlobalAsmArguments`.

:dp:`fls_2drikpht6md9`
:t:`[Assembly option]s` ``nomem`` and ``readonly`` shall not be used together.

:dp:`fls_x66j1cn6zi6p`
:t:`Assembly option` ``noreturn`` shall not be specified on an
:t:`assembly code block` that has :t:`[output register]s`.

:dp:`fls_ikwbu1ho33is`
:t:`Assembly option` ``pure`` shall appear with either :t:`assembly option`
``nomem`` or :t:`assembly option` ``readonly``.

:dp:`fls_nf0h9crdzhfg`
:t:`Assembly option` ``pure`` shall not be specified on an
:t:`assembly code block` that either lacks :t:`[output register]s` or all
:t:`[register expression]s` of :t:`[output register]s` are
:t:`[underscore expression]s`.

.. rubric:: Undefined Behavior

:dp:`fls_wh0wasawjj5s`
It is undefined behavior if an :t:`assembly code block` subject to
:t:`assembly option` ``pure`` has side effects other than its direct outputs.

:dp:`fls_s0ivlbjefh1u`
It is undefined behavior if control reaches the end of an
:t:`assembly code block` subject to :t:`assembly option` ``noreturn``.

.. rubric:: Examples

.. code-block:: rust

   options(nomem, pure)

.. _fls_qezwyridmjob:

Macros asm and global_asm
-------------------------

.. rubric:: Syntax

.. syntax::

   AsmArguments ::=
       $$($$ AssemblyCodeBlock ($$,$$ RegisterArgument)* ($$,$$ AbiClobber)* ($$,$$ AssemblyOption)* $$,$$? $$)$$

   GlobalAsmArguments ::=
       $$($$ AssemblyCodeBlock ($$,$$ RegisterArgument)* ($$,$$ AssemblyOption)* $$,$$? $$)$$

.. rubric:: Legality Rules

:dp:`fls_ecteot716j8j`
:t:`[Assembly code block]s` are embedded within Rust source code using
:t:`[macro]s` :std:`core::arch::asm` and :std:`core::arch::global_asm`.

:dp:`fls_1ikzov7cxic1`
When invoking :t:`macro` :std:`core::arch::asm`, the :s:`DelimitedTokenTree` of
the related :t:`macro invocation` shall follow the syntax of :s:`AsmArguments`.

:dp:`fls_4lb6yh12w1cv`
Invoking :t:`macro` :std:`core::arch::asm` causes the related
:t:`assembly code block` to be integrated into the generated assembly of the
:t:`function` where the :t:`macro invocation` took place. A tool is free to
encapsulate the :t:`assembly code block` in a separate :t:`function` and
generate a :t:`call expression` to it.

:dp:`fls_tgzga1lanfuo`
When invoking :t:`macro` :std:`core::arch::global_asm`, the
:s:`DelimitedTokenTree` of the related :t:`macro invocation` shall follow the
syntax of :s:`GlobalAsmArguments`.

:dp:`fls_nfkbvs86d6kz`
Invoking :t:`macro` :std:`core::arch::global_asm` causes the related
:t:`assembly code block` to be emitted outside the :t:`function` where the
:t:`macro invocation` took place.

.. rubric:: Dynamic Semantics

:dp:`fls_98vyqh9bzigx`
The :t:`evaluation` of a :t:`macro invocation` to :t:`macro`
:std:`core::arch::asm` or :t:`macro` :std:`core::arch::global_asm` evaluates
:t:`[register argument]s` in declarative order.

:dp:`fls_ppnj8bcncdp9`
The :t:`execution` of an :t:`assembly code block` produced by
:t:`inline assembly` proceeds as follows:

#. :dp:`fls_wmay1vd8u0da`
   All :t:`[input register]s` are initialized to the :t:`[value]s` provided by
   the respective :t:`[register argument]s`, in an undefined order.

#. :dp:`fls_e613hpr50t9`
   The :t:`[assembly instruction]s` of the :t:`assembly code block` are executed
   in declarative order.

#. :dp:`fls_bic6iyd1nvfm`
   The :t:`[value]s` of all :t:`[output register]s` are assigned to the
   :t:`[register expression]s` provided by the respective
   :t:`[register argument]s`, in an undefined order.

.. rubric:: Examples

.. code-block:: rust

   fn asm_example() -> u32 {
       let basepri;
       unsafe {
           asm!("
               mrs {}, BASEPRI
           ",
           out(reg) basepri,
           options(nomem, nostack, preserves_flags))
       }
       basepri
   }

   global_asm!("
       do_nothing:
           push {r7, lr}
           mov  r7, sp
           pop  {r7, pc}
       ",
       options(raw)
   );

   fn global_asm_example() {
       extern "C" {
           fn do_nothing();
       }

       unsafe { do_nothing() }
   }
