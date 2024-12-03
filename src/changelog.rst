.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec
.. informational-page::

FLS Changelog
=============

This page describes the changes that have been applied to the FLS itself to
address changes and new features introduced in each Rust release. Every item
listed in the "Language" section of the release note is reproduced here, along
with the change that has been applied due to it.

.. caution::

   This page is **not** an exhaustive list of all of the changes in a release,
   just the language changes that had an impact to the FLS. See the `release
   notes`_ for a full list of changes.


Language changes in Rust 1.82.0
-------------------------------

* `Don't make statement nonterminals match pattern nonterminals <https://github.com/rust-lang/rust/pull/120221/>`_

  * No change: Exact parsing behavior of non-terminals within declarative macros is not specified

* `Patterns matching empty types can now be omitted in common cases <https://github.com/rust-lang/rust/pull/122792>`_

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

  * New paragraphs: :p:`fls_8ltVLtAfvy0m`, :p:`fls_WRpcVF1fLEpr`, :p:`fls_8ltVLtAfvy0m`

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
