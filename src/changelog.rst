.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec
.. informational-page::

Changelog
=========

This page describes the changes that have been applied to the FLS itself to
address changes and new features introduced in each Rust release. Every item
listed in the "Language" section of the release note is reproduced here, along
with the change that has been applied due to it.

.. caution::

   This page is **not** an exhaustive list of all of the changes in a release,
   just the language changes that had an impact to the FLS. See the `release
   notes`_ for a full list of changes.

Language changes in Rust 1.91.1
-------------------------------

This release has no language changes.

Language changes in Rust 1.91.0
-------------------------------

- `Lower pattern bindings in the order they're written and base drop order on primary bindings' order <https://github.com/rust-lang/rust/pull/143764>`_

- `Stabilize declaration of C-style variadic functions for 'sysv64', 'win64', 'efiapi', and 'aapcs' ABIs <https://github.com/rust-lang/rust/pull/144066>`_.
  This brings these ABIs in line with the C ABI: variadic functions can be declared in extern blocks but not defined.

- `Add dangling_pointers_from_locals lint to warn against dangling pointers from local variables <https://github.com/rust-lang/rust/pull/144322>`_

  - No change: lints are not part of the FLS

- `Upgrade semicolon_in_expressions_from_macros from warn to deny <https://github.com/rust-lang/rust/pull/144369>`_

  - No change: lints are not part of the FLS

- `Stabilize LoongArch32 inline assembly <https://github.com/rust-lang/rust/pull/144402>`_

  - No change: the target is outside the scope of the FLS

- `Add warn-by-default integer_to_ptr_transmutes lint against integer-to-pointer transmutes <https://github.com/rust-lang/rust/pull/144531>`_

  - No change: lints are not part of the FLS

- `Stabilize 'sse4a' and 'tbm' target features <https://github.com/rust-lang/rust/pull/144542>`_

- `Add 'target_env = "macabi"' and 'target_env = "sim"' cfgs <https://github.com/rust-lang/rust/pull/139451>`_ as replacements for the `target_abi` cfgs with the same values.

Language changes in Rust 1.90.0
-------------------------------

- `Split up the unknown_or_malformed_diagnostic_attributes lint <https://github.com/rust-lang/rust/pull/140717>`_

  - No change: lints are not part of the FLS

- `Allow constants whose final value has references to mutable/external memory, but reject such constants as patterns <https://github.com/rust-lang/rust/pull/140942>`_

  - This lifted restriction was not specified in the FLS
  - The restriction on patterns is documented in :p:`fls_wJ9f906BlBvg`
  - New paragraph: :p:`fls_zyuxqty09SDO`

    - Above paragraph replaces :p:`fls_6g7c1kjrmfnr` and :p:`fls_hkbwa8xx2fwx`

  * New paragraphs:

    - :p:`fls_ooOYxhVh8hZo`

    - :p:`fls_zkNFeBLy80UA`

    - :p:`fls_VhzGfnWg7YrG`

    - :p:`fls_ibYKKQdB2tDn`

    - :p:`fls_dQdSxf8kOgbi`

    - :p:`fls_qC6L0km0ZMFI`

- `Allow volatile access to non-Rust memory, including address 0 <https://github.com/rust-lang/rust/pull/141260>`_

  - No change: lints are not part of the FLS

Language changes in Rust 1.89.0
-------------------------------

- `Stabilize explicitly inferred const arguments (feature(generic_arg_infer)) <https://github.com/rust-lang/rust/pull/141610>`_

  - New paragraphs:

    - :p:`fls_reASCId0i117`
    - :p:`fls_Ft5rGeL7QwJM`
    - :p:`fls_7epZNsTYNmgE`
    - :p:`fls_TvuMMQnR0drL`

  - Changed paragraph: :p:`fls_imr2jx6cbuzq`

- `Add a warn-by-default mismatched_lifetime_syntaxes lint <https://github.com/rust-lang/rust/pull/138677>`_.
  This lint detects when the same lifetime is referred to by different syntax categories between function arguments and return values, which can be confusing to read, especially in unsafe code.
  This lint supersedes the warn-by-default ``elided_named_lifetimes`` lint.

  - No change: lints are not part of the FLS

- `Expand unpredictable_function_pointer_comparisons to also lint on function pointer comparisons in external macros <https://github.com/rust-lang/rust/pull/134536>`_

  - No change: lints are not part of the FLS

- `Make the dangerous_implicit_autorefs lint deny-by-default <https://github.com/rust-lang/rust/pull/141661>`_

  - No change: lints are not part of the FLS

- `Stabilize the avx512 target features <https://github.com/rust-lang/rust/pull/138940>`_

  - Changed syntax: :s:`Feature`

  - New paragraphs:

    - :p:`fls_8HPcvIaahaB9`
    - :p:`fls_lxcfj6XCKyMh`
    - :p:`fls_fh7i1Pcmqjdm`
    - :p:`fls_M9ne2sqhRB5L`
    - :p:`fls_xWcGvl83bVWC`
    - :p:`fls_5CVTkOoLHAvW`
    - :p:`fls_hThb8qn820wJ`
    - :p:`fls_1T0VYMtG3loD`
    - :p:`fls_AlBD9ckWKecQ`
    - :p:`fls_72cwAExPDMQ8`
    - :p:`fls_pSSIwmUAnUly`
    - :p:`fls_YOW8lcBQ5lNI`
    - :p:`fls_8PrvVL6dkr31`
    - :p:`fls_hKvaSExdlKgd`
    - :p:`fls_r5KEicrmEGJt`
    - :p:`fls_Qnx9nNiJeIAX`
    - :p:`fls_ZrcPWy4pxlTX`
    - :p:`fls_YW7otDxAliSj`
    - :p:`fls_G4aOoX8a7i7r`
    - :p:`fls_HGKiXfM5fGVo`
    - :p:`fls_gRf8F9PIGySt`
    - :p:`fls_NNj4H6A9VTR8`

- `Stabilize these target features for x86: kl, widekl <https://github.com/rust-lang/rust/pull/140766>`_

  - Changed syntax: :s:`Feature`

  - New paragraphs:

    - :p:`fls_gtoLNHFmmBzd`
    - :p:`fls_qZO82VdU5Iz9`

- `Stabilize these target features for x86: sha512, sm3, sm4 <https://github.com/rust-lang/rust/pull/140767>`_

  - Changed syntax: :s:`Feature`

  - New paragraphs:

    - :p:`fls_UPhNSZS89sYr`
    - :p:`fls_HoNwaLtDzwZi`
    - :p:`fls_VqjfIihpRe9m`

- `Stabilize these LoongArch target features: f, d, frecipe, lasx, lbt, lsx, lvz <https://github.com/rust-lang/rust/pull/135015>`_

  - No change: the target is outside the scope of the FLS

- `Remove i128 and u128 from improper_ctypes_definitions lint <https://github.com/rust-lang/rust/pull/137306>`_

  - No change: lints are not part of the FLS

- `Stabilize repr128 (#[repr(u128), #repr(i128)]) <https://github.com/rust-lang/rust/pull/138285>`_

  - No change: this was erroneously documented as if it were already true

- `Allow #![doc(test(attr(..))) everywhere <https://github.com/rust-lang/rust/pull/140560>`_

  - No change: documentation tests are outside the scope of the FLS

- `Extend temporary lifetime extension to also go through tuple struct and tuple variant constructors <https://github.com/rust-lang/rust/pull/140593>`_

  - Changed paragraphs:

    - :p:`fls_DQaCUkskfXzk`
    - :p:`fls_wyzau8hhq74d`

  - New paragraphs:

    - :p:`fls_YeeZWqTdMivX`
    - :p:`fls_eeaJtK4w5gVK`

- `extern "C" functions on the wasm32-unknown-unknown target now have a standards compliant ABI <https://blog.rust-lang.org/2025/04/04/c-abi-changes-for-wasm32-unknown-unknown/>`_

  - No change: the target is outside the scope of the FLS

Language changes in Rust 1.88.0
-------------------------------

- `Stabilize let_chains feature in the 2024 edition <https://github.com/rust-lang/rust/pull/132833>`_

  - No change: the FLS only supports the 2021 edition

- `Stabilize naked_functions feature <https://github.com/rust-lang/rust/pull/134213>`_

  - New sections: :ref:`fls_Sd6rUmpEb355`

  - New paragraphs:

    - :p:`fls_eOJS3mxa9xgu`

    - :p:`fls_2oP2nbDPtUg7`

    - :p:`fls_y2wCBvXDtQK2`

    - :p:`fls_PEoOGTBjuEQc`

    - :p:`fls_vcB5xwgD6Ign`

  - Changed paragraphs:

    - :p:`fls_3fg60jblx0xb`

    - :p:`fls_ecteot716j8j`

    - :p:`fls_tgzga1lanfuo`

    - :p:`fls_g09kmp2a04g9`

    - :p:`fls_nszx1gllufi2`

  - New syntax: :s:`SymPathExpression`

  - Changed syntax:

    - :s:`RegisterArgument`

    - :s:`RegisterExpression`

    - :s:`BuiltinAttributeContent`

- `Stabilize cfg_boolean_literals feature <https://github.com/rust-lang/rust/pull/138632>`_

  - Changed syntax: :s:`ConfigurationPredicate`

- `Fully de-stabilize the #[bench] attribute <https://github.com/rust-lang/rust/pull/134273)>`_

  - No change: this was an unstable feature and was therefore not specified in the FLS

- `Add warn-by-default dangerous_implicit_autorefs lint against implicit autoref of raw pointer dereference <https://github.com/rust-lang/rust/pull/123239>`_

  - No change: lints are not part of the FLS

- `Add invalid_null_arguments lint to prevent invalid usage of null pointers <https://github.com/rust-lang/rust/pull/119220>`_

  - No change: lints are not part of the FLS

- `Change trait impl candidate preference for builtin impls and trivial where-clauses. <https://github.com/rust-lang/rust/pull/138176>`_

  - No change: trait impl candidate preference is not specified in the FLS

- `Check types of generic const parameter defaults <https://github.com/rust-lang/rust/pull/139646>`_

  - No change: this bug was not documented in the FLS

Language changes in Rust 1.87.0
-------------------------------

- `Stabilize asm_goto feature <https://github.com/rust-lang/rust/pull/133870>`_

  - New section: :ref:`fls_MW7mtH5oOeQ1`

- `Allow parsing open beginning ranges (..EXPR) after unary operators: ! - * <https://github.com/rust-lang/rust/pull/134900>`_

  - No change: this lifted restriction was not specified in the FLS

- `Don't require method impls for methods with "Self: Sized" bounds in impls for unsized types <https://github.com/rust-lang/rust/pull/135480>`_

  - No change: this lifted restriction was not specified in the FLS

- `Stabilize feature(precise_capturing_in_traits) allowing use<...> bounds on return position impl Trait in traits (RPITIT) <https://github.com/rust-lang/rust/pull/138128>`_

  - No change: this lifted restriction was not specified in the FLS

Language changes in Rust 1.86.0
-------------------------------

- `Stabilize upcasting trait objects to supertraits. <https://github.com/rust-lang/rust/pull/134367>`_

  - New paragraph: :p:`fls_QB4c6FNKxaPl`

- `Allow safe functions to be marked with the #[target_feature] attribute. <https://github.com/rust-lang/rust/pull/134090>`_

  - Changed paragraph: :p:`fls_3qj3jvmtxvx6`

- `The missing_abi lint now warns-by-default. <https://github.com/rust-lang/rust/pull/132397>`_

  - No change: Lints are not part of the FLS

- `Rust now lints about double negations, to catch cases that might have intended to be a prefix decrement operator (--x) as written in other languages. This was previously a clippy lint, clippy::double_neg, and is now available directly in Rust as double_negations. <https://github.com/rust-lang/rust/pull/126604>`_

  - No change: Lints are not part of the FLS

- `More pointers are now detected as definitely not-null based on their alignment in const eval. <https://github.com/rust-lang/rust/pull/133700>`_

  - No change: The concrete semantics of constant evaluation is not described within the FLS

- `Empty repr() attribute applied to invalid items are now correctly rejected. <https://github.com/rust-lang/rust/pull/133925>`_

  - No change: this bug was not documented

- `Inner attributes, #![test] and #![rustfmt::skip], are no longer accepted in more places than intended. <https://github.com/rust-lang/rust/pull/134276>`_

  - No change: These attributes are not part of the FLS

Language changes in Rust 1.85.0
-------------------------------

* `The 2024 Edition is now stable. <https://github.com/rust-lang/rust/pull/133349>`_

  * No change: The FLS currently qualifies only the 2021 Edition

* `Stabilize async closures <https://github.com/rust-lang/rust/pull/132706>`_

  * New paragraphs: :p:`fls_My6pMgpeFCFg`, :p:`fls_DSy7bPKGzyov`

  * Changed syntax: :s:`ClosureExpression`

* `Stabilize \`#[diagnostic::do_not_recommend]\` <https://github.com/rust-lang/rust/pull/132056>`_

  * No change: tool attributes are not part of the FLS

* `Add \`unpredictable_function_pointer_comparisons\` lint to warn against function pointer comparisons <https://github.com/rust-lang/rust/pull/118833>`_

  * No change: Lints are not part of the FLS

* `Lint on combining \`#[no_mangle]\` and \`#[export_name]\` attributes. <https://github.com/rust-lang/rust/pull/131558>`_

  * No change: Lints are not part of the FLS

Language changes in Rust 1.84.0
-------------------------------

* `Allow \`#[deny]\` inside \`#[forbid]\` as a no-op <https://github.com/rust-lang/rust/pull/121560/>`_

  * No change: Lints are not part of the FLS

* `Show a warning when \`-Ctarget-feature\` is used to toggle features that can lead to unsoundness due to ABI mismatches <https://github.com/rust-lang/rust/pull/129884>`_

  * No change: `target-feature` is outside the scope of the Ferrocene qualification

* `Use the next-generation trait solver in coherence <https://github.com/rust-lang/rust/pull/130654>`_

  * No change: the exact trait solver is not part of the FLS

* `Allow coercions to drop the principal of trait objects <https://github.com/rust-lang/rust/pull/131857>`_

  * Changed paragraph: :p:`fls_SYnFJBhi0IWj`

* `Support \`/\` as the path separator for \`include!()\` in all cases on Windows <https://github.com/rust-lang/rust/pull/125205>`_

  * No change: This past restriction of the `include` macro is not specified by the FLS

* `Taking a raw ref (\`raw (const|mut)\`) of a deref of a pointer (\`*ptr\`) is now safe <https://github.com/rust-lang/rust/pull/129248>`_

  * Changed paragraph: :p:`fls_8i4jzksxlrw0`

* `Stabilize s390x inline assembly <https://github.com/rust-lang/rust/pull/131258>`_

  * No change: These changes affect content that is informational

* `Stabilize Arm64EC inline assembly <https://github.com/rust-lang/rust/pull/131781>`_

  * No change: These changes affect content that is informational

* `Lint against creating pointers to immediately dropped temporaries <https://github.com/rust-lang/rust/pull/128985>`_

  * No change: Lints are not part of the FLS

* `Execute drop glue when unwinding in an \`extern "C"\` function <https://github.com/rust-lang/rust/pull/129582>`_

  * No change: This lifted restriction was not specified in the FLS

Language changes in Rust 1.83.0
-------------------------------

* `Stabilize \`&mut\`, \`*mut\`, \`&Cell\`, and \`*const Cell\` in const. <https://github.com/rust-lang/rust/pull/129195>`_

  * Changed paragraphs: :p:`fls_to4e7imq2c0w`, :p:`fls_6g7c1kjrmfnr`, :p:`fls_hkbwa8xx2fwx`

  * Removed paragraph: :p:`fls_ox6sgl9n43g2`

* `Allow creating references to statics in \`const\` initializers. <https://github.com/rust-lang/rust/pull/129759>`_

  * No change: This previous restriction is not specified in the FLS

* `Implement raw lifetimes and labels (\`'r#ident\`). <https://github.com/rust-lang/rust/pull/126452>`_

  * Changed syntax: :s:`Lifetime`

* `Define behavior when atomic and non-atomic reads race. <https://github.com/rust-lang/rust/pull/128778>`_

  * No change: Already covered by the definition of :t:`data race`.

* `Non-exhaustive structs may now be empty. <https://github.com/rust-lang/rust/pull/128934>`_

  * Removed paragraph: :p:`fls_2CWUWbYT9KcT`

  * Changed paragraph: :p:`fls_fSNrRsgzLd0E`, :p:`fls_S9QL6yVF5LFI`

* `Disallow implicit coercions from places of type \`!\` <https://github.com/rust-lang/rust/pull/129392>`_

  * No change: The FLS does not specify type inference to such a degree

* `\`const extern\` functions can now be defined for other calling conventions. <https://github.com/rust-lang/rust/pull/129753>`_

  * No change: This previous restriction is not specified in the FLS

* `Stabilize \`expr_2021\` macro fragment specifier in all editions. <https://github.com/rust-lang/rust/pull/129972>`_

  * Changed syntax: :s:`MacroFragmentSpecifier`

  * Changed paragraphs: :p:`fls_k00bck2k8tde`, :p:`fls_PxR9vNHsaFnI`

* `The \`non_local_definitions\` lint now fires on less code and warns by default. <https://github.com/rust-lang/rust/pull/127117>`_

  * No change: Lints are not part of the FLS

Language changes in Rust 1.82.0
-------------------------------

* `Don't make statement nonterminals match pattern nonterminals <https://github.com/rust-lang/rust/pull/120221/>`_

  * No change: Exact parsing behavior of non-terminals within declarative macros is not specified

* `Patterns matching empty types can now be omitted in common cases <https://github.com/rust-lang/rust/pull/122792>`_

  * New section: :ref:`fls_mcxF9y5u66sZ`

  * Changed paragraphs: :p:`fls_9fjspnefoyvz`, :p:`fls_uq7ftuuq1sig`, :p:`fls_cfoy86mkmqa4`, :p:`fls_rnppz6y5z8pi`, :p:`fls_x0bmzl1315gq`, :p:`fls_MK83WE0iDqNf`

* `Enforce supertrait outlives obligations when using trait impls <https://github.com/rust-lang/rust/pull/124336>`_

  * No change: the concrete type inference resolution is not part of the FLS

* `\`addr_of(_mut)!\` macros and the newly stabilized \`&raw (const|mut)\` are now safe to use with all static items <https://github.com/rust-lang/rust/pull/125834>`_

  * No change: `addr_of` is not specified as it is a library defined macro, `&raw (const|mut)` appears as a new feature separately below

* `size_of_val_raw: for length 0 this is safe to call <https://github.com/rust-lang/rust/pull/126152/>`_

  * No change: `size_of_val_raw` is a library defined function

* `Reorder trait bound modifiers *after* \`for<...>\` binder in trait bounds <https://github.com/rust-lang/rust/pull/127054/>`_

  * Changed syntax: :s:`TraitBound`

* `Stabilize opaque type precise capturing (RFC 3617) <https://github.com/rust-lang/rust/pull/127672>`_

  * Changed syntax: :s:`ImplTraitTypeSpecification`, :s:`ImplTraitTypeSpecificationOneBound`

  * New syntax: :s:`UseCaptures`, :s:`UseCapturesGenericArgs`, :s:`UseCapturesGenericArg`

  * New paragraphs: :p:`fls_69hqMjvNno9u`, :p:`fls_OnyR0Wsfk7KI`, :p:`fls_KgH6c5cC4S0G`, :p:`fls_iT9WCNfUZQnC`

* `Stabilize \`&raw const\` and \`&raw mut\` operators (RFC 2582) <https://github.com/rust-lang/rust/pull/127679>`_

  * New section: :ref:`fls_vXGuvRWOLbEE`

  * New paragraphs: :p:`fls_K7SbApHPmwjM`

* `Stabilize unsafe extern blocks (RFC 3484) <https://github.com/rust-lang/rust/pull/127921>`_

  * New syntax: :s:`ItemSafety`

  * Changed syntax: :s:`WeakKeyword`, :s:`FunctionQualifierList`, :s:`StaticDeclaration`

  * New paragraphs: :p:`fls_8ltVLtAfvy0m`, :p:`fls_WRpcVF1fLEpr`, :p:`fls_nUADhgcfvvGC`

  * Changed paragraphs: :p:`fls_g0JEluWqBpNc`, :p:`fls_7ucwmzqtittv`, :p:`fls_4dje9t5y2dia`, :p:`fls_l88r9fj82650`, :p:`fls_fo9with6xumo`

  * Removed paragraph :p:`fls_iaimuqcclstl`

* `Stabilize nested field access in \`offset_of!\` <https://github.com/rust-lang/rust/pull/128284>`_

  * No change: `offset_of` is a library defined macro

* `Do not require \`T\` to be live when dropping \`[T; 0]\` <https://github.com/rust-lang/rust/pull/128438>`_

  * No change: The drop interaction with the borrow checker is not specified as the borrow checker is not specified in the FLS

* `Stabilize \`const\` operands in inline assembly <https://github.com/rust-lang/rust/pull/128570>`_

  * Note: These changes affect content that is informational.

  * New syntax: :s:`ConstRegisterExpression`

  * New paragraphs: :p:`fls_81Ju1TEqJ48K`, :p:`fls_j9XOoXDmN5Dq`, :p:`fls_jU8zg4k8dFsY`

* `Stabilize floating-point arithmetic in \`const fn\` <https://github.com/rust-lang/rust/pull/128596>`_

  * New paragraph: :p:`fls_lSxXWxJn0vMO`

  * Removed paragraph: :p:`fls_9mrrosm8jnn7`

* `Stabilize explicit opt-in to unsafe attributes <https://github.com/rust-lang/rust/pull/128771>`_

  * New section: :ref:`fls_19LnTi3WabFd`

* `Document NaN bit patterns guarantees <https://github.com/rust-lang/rust/pull/129559>`_

  * New paragraph: :p:`fls_nuFAwLHOdQBx`

Language changes in Rust 1.81.0
-------------------------------

* `Abort on uncaught panics in \`extern "C"\` functions. <https://github.com/rust-lang/rust/pull/116088/>`_

  * No change: unwinding is not specified in the FLS

* `Fix ambiguous cases of multiple \`&\` in elided self lifetimes. <https://github.com/rust-lang/rust/pull/117967/>`_

  * Changed paragraph: :p:`fls_crb6m6b3cdwh`

  * New paragraph: :p:`fls_d4u3y82hdadc`

* `Stabilize \`#[expect]\` for lints (RFC 2383), like \`#[allow]\` with a warning if the lint is _not_ fulfilled. <https://github.com/rust-lang/rust/pull/120924/>`_

  * New paragraph: :p:`fls_NrTL2FruARAv`

* `Change method resolution to constrain hidden types instead of rejecting method candidates. <https://github.com/rust-lang/rust/pull/123962/>`_

  * No change: the concrete type inference resolution is not part of the FLS

* `Bump \`elided_lifetimes_in_associated_constant\` to deny. <https://github.com/rust-lang/rust/pull/124211/>`_

  * No change: lints are not part of the FLS

* `\`offset_from\`: always allow pointers to point to the same address. <https://github.com/rust-lang/rust/pull/124921/>`_

  * No change: this previous restriction is not specified in the FLS

* `Allow constraining opaque types during subtyping in the trait system. <https://github.com/rust-lang/rust/pull/125447/>`_

  * No change: the concrete type inference resolution is not part of the FLS

* `Allow constraining opaque types during various unsizing casts. <https://github.com/rust-lang/rust/pull/125610/>`_

  * No change: the concrete type inference resolution is not part of the FLS

* `Deny keyword lifetimes pre-expansion. <https://github.com/rust-lang/rust/pull/126762/>`_

  * No change: the FLS already specifies this restriction in :s:`Lifetime`

Language changes in Rust 1.80.0
-------------------------------

* `Document maximum allocation size <https://github.com/rust-lang/rust/pull/116675/>`_

  * New paragraphs: :p:`fls_CUJyMj0Sj8NS`, :p:`fls_kaomYy0Ml4Nh`, :p:`fls_B5cmkWfD5GNt`, :p:`fls_oqhQ62mDLckN`, :p:`fls_uhwpuv6cx4ip`, :p:`fls_xuuFKmm181bs`

* `Allow zero-byte offsets and ZST read/writes on arbitrary pointers <https://github.com/rust-lang/rust/pull/117329/>`_

  * No change: this previous restriction is not specified in the FLS

* `Support C23's variadics without a named parameter <https://github.com/rust-lang/rust/pull/124048/>`_

  * No change: this previous restriction is not specified in the FLS

* `Stabilize \`exclusive_range_pattern\` feature <https://github.com/rust-lang/rust/pull/124459/>`_

  * Changed syntax: :s:`RangePattern`

  * New syntax: :s:`ExclusiveRangePattern`

  * Changed paragraph: :p:`fls_8Q6NfRx4j5V7`

  * New paragraphs: :p:`fls_3PyquOKjA7SI`, :p:`fls_8bdOqkO1NuJW`, :p:`fls_EDL1Pi56KQ2H`

Language changes in Rust 1.79.0
-------------------------------

* `Stabilize inline \`const {}\` expressions. <https://github.com/rust-lang/rust/pull/104087/>`_

  * New section: :ref:`fls_G59PiNQkVUnQ`

* `Prevent opaque types being instantiated twice with different regions within the same function. <https://github.com/rust-lang/rust/pull/116935/>`_

  * No change: already described in :p:`fls_hza5n5eb18ta`

* `Stabilize WebAssembly target features that are in phase 4 and 5. <https://github.com/rust-lang/rust/pull/117457/>`_

  * No change: ``cfg`` and ``cfg_attr`` configuration predicates are not part of the FLS

* `Add the \`redundant_lifetimes\` lint to detect lifetimes which are semantically redundant. <https://github.com/rust-lang/rust/pull/118391/>`_

  * No change: lints are not part of the FLS

* `Stabilize the \`unnameable_types\` lint for public types that can't be named. <https://github.com/rust-lang/rust/pull/120144/>`_

  * No change: lints are not part of the FLS

* `Enable debuginfo in macros, and stabilize \`-C collapse-macro-debuginfo\` and \`#[collapse_debuginfo]\`. <https://github.com/rust-lang/rust/pull/120845/>`_

  * New section: :ref:`fls_qyudjGHZfyJH`

* `Propagate temporary lifetime extension into \`if\` and \`match\` expressions. <https://github.com/rust-lang/rust/pull/121346/>`_

  * New paragraphs: :p:`fls_Rj9zhVutfQod`, :p:`fls_oodpp3LpXC13`, :p:`fls_xGThCPoTUSAi`

* `Restrict promotion of \`const fn\` calls. <https://github.com/rust-lang/rust/pull/121557/>`_

  * No change: already described in :p:`fls_3h5vr7xk2rrt`

* `Warn against refining impls of crate-private traits with \`refining_impl_trait\` lint. <https://github.com/rust-lang/rust/pull/121720/>`_

  * No change: lints are not part of the FLS

* `Stabilize associated type bounds (RFC 2289). <https://github.com/rust-lang/rust/pull/122055/>`_

  * New paragraph: :p:`fls_mcUMWsYcxzmZ`

* `Stabilize importing \`main\` from other modules or crates. <https://github.com/rust-lang/rust/pull/122060/>`_

  * No change: this lifted restriction was not previously described in the FLS

  * While updating the FLS to account for this feature, we realized that the
    way the FLS described crate types was incorrect. We rectified this:

    * New section: :ref:`fls_8JB3SJqamdpU`
    * New glossary entry: :t:`crate type`
    * New paragraphs: :p:`fls_unxalgMqIr3v`, :p:`fls_e7jGvXvTsFpC`, :p:`fls_kQiJPwb2Hjcc`, :p:`fls_OyFwBtDGVimT`
    * Updated glossary entries: :t:`binary crate`, :t:`library crate`, :t:`proc-macro crate`
    * Updated paragraphs: :p:`fls_9ub6ks8qrang`, :p:`fls_Mf62VqAhoZ3c`, :p:`fls_d9nn4yuiw1ja`
    * Moved paragraph: :p:`fls_sbGnkm8Ephiu`

* `Check return types of function types for well-formedness <https://github.com/rust-lang/rust/pull/115538>`_

  * No change: the exact trait resolution implementation is not part of the FLS

* `Rework \`impl Trait\` lifetime inference <https://github.com/rust-lang/rust/pull/116891/>`_

  * New paragraphs: :p:`fls_3aKZB0ILIkZw`, :p:`fls_Xo1ODwOyX7Vm`, :p:`fls_kTGFLFymTWch`

* `Change inductive trait solver cycles to be ambiguous <https://github.com/rust-lang/rust/pull/122791>`_

  * No change: the exact trait solver is not part of the FLS

Language changes in Rust 1.78.0
-------------------------------

* `Stabilize \`#[cfg(target_abi = ...)]\` <https://github.com/rust-lang/rust/pull/119590/>`_

  * No change: ``cfg`` and ``cfg_attr`` configuration predicates are not part of the FLS

* `Stabilize the \`#[diagnostic]\` namespace and \`#[diagnostic::on_unimplemented]\` attribute <https://github.com/rust-lang/rust/pull/119888/>`_

  * No change: tool attributes are not part of the FLS

* `Make async-fn-in-trait implementable with concrete signatures <https://github.com/rust-lang/rust/pull/120103/>`_

  * No change: no paragraph in the FLS forbids this prior incompatability

* `Make matching on NaN a hard error, and remove the rest of \`illegal_floating_point_literal_pattern\` <https://github.com/rust-lang/rust/pull/116284/>`_

  * New paragraph: :p:`fls_JP8YSbxSN0Ym`

* `static mut: allow mutable reference to arbitrary types, not just slices and arrays <https://github.com/rust-lang/rust/pull/117614/>`_

  * No change: this lifted restriction was not previously described in the FLS

* `Extend \`invalid_reference_casting\` to include references casting to bigger memory layout <https://github.com/rust-lang/rust/pull/118983/>`_

  * No change: lints are not part of the FLS

* `Add \`non_contiguous_range_endpoints\` lint for singleton gaps after exclusive ranges <https://github.com/rust-lang/rust/pull/118879/>`_

  * No change: lints are not part of the FLS

* `Add \`wasm_c_abi\` lint for use of older wasm-bindgen versions <https://github.com/rust-lang/rust/pull/117918/>`_

  * No change: lints are not part of the FLS

* `Update \`indirect_structural_match\` and \`pointer_structural_match\` lints to match RFC <https://github.com/rust-lang/rust/pull/120423/>`_

  * No change: lints are not part of the FLS

* `Make non-\`PartialEq\`-typed consts as patterns a hard error <https://github.com/rust-lang/rust/pull/120805/>`_

  * No change: already described in :p:`fls_zCswsyuitexI`

* `Split \`refining_impl_trait\` lint into \`_reachable\`, \`_internal\` variants <https://github.com/rust-lang/rust/pull/121720/>`_

  * No change: lints are not part of the FLS

* `Remove unnecessary type inference when using associated types inside of higher ranked \`where\`-bounds <https://github.com/rust-lang/rust/pull/119849>`_

  * No change: the FLS does not specify type inference to such a degree

* `Weaken eager detection of cyclic types during type inference <https://github.com/rust-lang/rust/pull/119989>`_

  * No change: the FLS does not specify type inference to such a degree

* `\`trait Trait: Auto {}\`: allow upcasting from \`dyn Trait\` to \`dyn Trait + Auto\` <https://github.com/rust-lang/rust/pull/119338>`_

  * New paragraph: :p:`fls_SYnFJBhi0IWj`

language changes in Rust 1.77.0
-------------------------------

* `Reveal opaque types within the defining body for exhaustiveness checking. <https://github.com/rust-lang/rust/pull/116821/>`_

  * No change: the FLS does not specify introspection of the concrete type of the match expression scrutinee to such a degree

* `Stabilize C-string literals. <https://github.com/rust-lang/rust/pull/117472/>`_

  * New section: :ref:`fls_U1gHCy16emVe`

* `Stabilize THIR unsafeck. <https://github.com/rust-lang/rust/pull/117673/>`_

  * No change: not a language change

* `Add lint \`static_mut_refs\` to warn on references to mutable statics. <https://github.com/rust-lang/rust/pull/117556/>`_

  * No change: lints are not part of the FLS

* `Support async recursive calls (as long as they have indirection). <https://github.com/rust-lang/rust/pull/117703/>`_

  * No change: this lifted restriction was not previously described in the FLS

* `Undeprecate lint \`unstable_features\` and make use of it in the compiler. <https://github.com/rust-lang/rust/pull/118639/>`_

  * No change: lints are not part of the FLS

* `Make inductive cycles in coherence ambiguous always. <https://github.com/rust-lang/rust/pull/118649/>`_

  * No change: the FLS does not describe the trait solver to such a degree

* `Get rid of type-driven traversal in const-eval interning <https://github.com/rust-lang/rust/pull/119044/>`_, only as a `future compatibility lint <https://github.com/rust-lang/rust/pull/122204>`_ for now.

  * No change: this lifted restriction was not previously described in the FLS

* `Deny braced macro invocations in let-else. <https://github.com/rust-lang/rust/pull/119062/>`_

  * New paragraph: :p:`fls_1s1UikGU5YQb`

.. Note: for the publicly rendered version of the FLS we want to link to
   upstream's release notes. In the Ferrocene subtree this should be replaced
   to the link to the Ferrocene release notes!
.. _release notes: https://doc.rust-lang.org/releases.html
