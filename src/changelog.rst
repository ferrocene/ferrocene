.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec
.. informational-page::

Ferrocene Specification Changelog
=================================

Version 1.79.0 (2024-06-13)
---------------------------

* `Stabilize inline \`const {}\` expressions. <https://github.com/rust-lang/rust/pull/104087/>`_

  * New section: Const blocks :p:`fls_G59PiNQkVUnQ`

* `Prevent opaque types being instantiated twice with different regions within the same function. <https://github.com/rust-lang/rust/pull/116935/>`_

  * No change: This is already described in types and trait :p:`fls_hza5n5eb18ta`

* `Stabilize WebAssembly target features that are in phase 4 and 5. <https://github.com/rust-lang/rust/pull/117457/>`_

  * No change: `cfg` and `cfg_attr` configuration predicates are not part of the FLS.

* `Add the \`redundant_lifetimes\` lint to detect lifetimes which are semantically redundant. <https://github.com/rust-lang/rust/pull/118391/>`_

  * No change: Lints are not part of the FLS.

* `Stabilize the \`unnameable_types\` lint for public types that can't be named. <https://github.com/rust-lang/rust/pull/120144/>`_

  * No change: Lints are not part of the FLS.

* `Enable debuginfo in macros, and stabilize \`-C collapse-macro-debuginfo\` and \`#[collapse_debuginfo]\`. <https://github.com/rust-lang/rust/pull/120845/>`_

  * New section: Attribute `collapse_debuginfo` :p:`fls_qyudjGHZfyJH`

* `Propagate temporary lifetime extension into \`if\` and \`match\` expressions. <https://github.com/rust-lang/rust/pull/121346/>`_

  * New paragraphs:

    * :p:`fls_Rj9zhVutfQod`
    * :p:`fls_oodpp3LpXC13`
    * :p:`fls_xGThCPoTUSAi`

* `Restrict promotion of \`const fn\` calls. <https://github.com/rust-lang/rust/pull/121557/>`_

  * No change: This is already specified in values :p:`fls_3h5vr7xk2rrt`

* `Warn against refining impls of crate-private traits with \`refining_impl_trait\` lint. <https://github.com/rust-lang/rust/pull/121720/>`_

  * No change: Lints are not part of the FLS.

* `Stabilize associated type bounds (RFC 2289). <https://github.com/rust-lang/rust/pull/122055/>`_

  * New paragraph: :p:`fls_mcUMWsYcxzmZ`

* `Stabilize importing \`main\` from other modules or crates. <https://github.com/rust-lang/rust/pull/122060/>`_

  * Various changes:

    * New section: Program Entry Point :p:`fls_8JB3SJqamdpU`
    * New definition: :t:`crate type` :p:`fls_unxalgMqIr3v` :p:`fls_e7jGvXvTsFpC` :p:`fls_kQiJPwb2Hjcc`
    * Removal of definition `main function`
    * binary crate and proc-macro crate rewritten :p:`fls_9ub6ks8qrang` :p:`fls_Mf62VqAhoZ3c`
    * library crate removed
    * Introduction of `main function signature` functions :p:`fls_sbGnkm8Ephiu`

* `Check return types of function types for well-formedness <https://github.com/rust-lang/rust/pull/115538>`_

  * No change: This is a bug fix in the trait resolution which is not described in the FLS.

* `Rework \`impl Trait\` lifetime inference <https://github.com/rust-lang/rust/pull/116891/>`_

  * No change: Capturing of lifestime within \`impl Trait\` types is not described in the FLS.

* `Change inductive trait solver cycles to be ambiguous <https://github.com/rust-lang/rust/pull/122791>`_

  * No change: The trait solver is not part of the FLS.

Version 1.78.0 (2024-05-02)
---------------------------

* `Stabilize \`#[cfg(target_abi = ...)]\` <https://github.com/rust-lang/rust/pull/119590/>`_

  * No change: \`cfg\` and \`cfg_attr\` configuration predicates are not part of the FLS.

* `Stabilize the \`#[diagnostic]\` namespace and \`#[diagnostic::on_unimplemented]\` attribute <https://github.com/rust-lang/rust/pull/119888/>`_

  * No change: Tool attributes are not part of the FLS.

* `Make async-fn-in-trait implementable with concrete signatures <https://github.com/rust-lang/rust/pull/120103/>`_

  * No change: No paragraph in the FLS forbids this prior incompatability

* `Make matching on NaN a hard error, and remove the rest of \`illegal_floating_point_literal_pattern\` <https://github.com/rust-lang/rust/pull/116284/>`_

  * New paragraph: :p:`fls_JP8YSbxSN0Ym`

* `static mut: allow mutable reference to arbitrary types, not just slices and arrays <https://github.com/rust-lang/rust/pull/117614/>`_

  * No change: This lifted restriction is not described in the FLS.

* `Extend \`invalid_reference_casting\` to include references casting to bigger memory layout <https://github.com/rust-lang/rust/pull/118983/>`_

  * No change: Lints are not part of the FLS.

* `Add \`non_contiguous_range_endpoints\` lint for singleton gaps after exclusive ranges <https://github.com/rust-lang/rust/pull/118879/>`_

  * No change: Lints are not part of the FLS.

* `Add \`wasm_c_abi\` lint for use of older wasm-bindgen versions <https://github.com/rust-lang/rust/pull/117918/>`_

  * No change: Lints are not part of the FLS.

* `Update \`indirect_structural_match\` and \`pointer_structural_match\` lints to match RFC <https://github.com/rust-lang/rust/pull/120423/>`_

  * No change: Lints are not part of the FLS.

* `Make non-\`PartialEq\`-typed consts as patterns a hard error <https://github.com/rust-lang/rust/pull/120805/>`_

  * No change: This behavior is already described in the FLS in :p:`fls_zCswsyuitexI`

* `Split \`refining_impl_trait\` lint into \`_reachable\`, \`_internal\` variants <https://github.com/rust-lang/rust/pull/121720/>`_

  * No change: Lints are not part of the FLS.

* `Remove unnecessary type inference when using associated types inside of higher ranked \`where\`-bounds <https://github.com/rust-lang/rust/pull/119849>`_

  * No change: The FLS does not specify type inference to such a degree.

* `Weaken eager detection of cyclic types during type inference <https://github.com/rust-lang/rust/pull/119989>`_

  * No change: The FLS does not specify type inference to such a degree.

* `\`trait Trait: Auto {}\`: allow upcasting from \`dyn Trait\` to \`dyn Trait + Auto\` <https://github.com/rust-lang/rust/pull/119338>`_

Version 1.77.0 (2024-03-21)
---------------------------

* `Reveal opaque types within the defining body for exhaustiveness checking. <https://github.com/rust-lang/rust/pull/116821/>`_

  * No change: The introspection of the concrete type of the match expression scrutinee is not discussed in paragraph :p:`fls_e02um1gb89d0`

* `Stabilize C-string literals. <https://github.com/rust-lang/rust/pull/117472/>`_

  * New section: :p:`fls_U1gHCy16emVe`

* `Stabilize THIR unsafeck. <https://github.com/rust-lang/rust/pull/117673/>`_

  * No change: Not a language change.

* `Add lint \`static_mut_refs\` to warn on references to mutable statics. <https://github.com/rust-lang/rust/pull/117556/>`_

  * No change: Lints are not part of the FLS.

* `Support async recursive calls (as long as they have indirection). <https://github.com/rust-lang/rust/pull/117703/>`_

  * No change: This async restriction is not described in the FLS.

* `Undeprecate lint \`unstable_features\` and make use of it in the compiler. <https://github.com/rust-lang/rust/pull/118639/>`_

  * No change: Lints are not part of the FLS.

* `Make inductive cycles in coherence ambiguous always. <https://github.com/rust-lang/rust/pull/118649/>`_

  * No change: The trait solver in coherence resolution is not part of the FLS

* `Get rid of type-driven traversal in const-eval interning <https://github.com/rust-lang/rust/pull/119044/>`_, only as a `future compatibility lint <https://github.com/rust-lang/rust/pull/122204>`_ for now.

  * No change: This restriction of const promotion is not described in the FLS.

* `Deny braced macro invocations in let-else. <https://github.com/rust-lang/rust/pull/119062/>`_

  * New paragraph: :p:`fls_1s1UikGU5YQb`
