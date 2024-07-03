.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec
.. informational-page::

.. _fls_grvqHf0SUaiN:

Ferrocene Specification Changelog
=================================

.. _fls_6TRrFesJWnoL:

Version 1.79.0 (2024-06-13)
---------------------------

* :dp:`fls_tPldXLk59Axx`
  `Stabilize inline \`const {}\` expressions. <https://github.com/rust-lang/rust/pull/104087/>`_

  * :dp:`fls_sLrh1GVvUGPn`
    New section: Const blocks :p:`fls_G59PiNQkVUnQ`

* :dp:`fls_juWXW4VMwAs7`
  `Prevent opaque types being instantiated twice with different regions within the same function. <https://github.com/rust-lang/rust/pull/116935/>`_

  * :dp:`fls_cpxs2P41vApm`
    No change: This is already described in types and trait :p:`fls_hza5n5eb18ta`

* :dp:`fls_HbxNaRTX5XoS`
  `Stabilize WebAssembly target features that are in phase 4 and 5. <https://github.com/rust-lang/rust/pull/117457/>`_

  * :dp:`fls_Dbufw279W10c`
    No change: `cfg` and `cfg_attr` configuration predicates are not part of the FLS.

* :dp:`fls_Dbufw279W10c`
  `Add the \`redundant_lifetimes\` lint to detect lifetimes which are semantically redundant. <https://github.com/rust-lang/rust/pull/118391/>`_

  * :dp:`fls_OuDLgCAVNzjU`
    No change: Lints are not part of the FLS.

* :dp:`fls_mdAd1wCv1hMI`
  `Stabilize the \`unnameable_types\` lint for public types that can't be named. <https://github.com/rust-lang/rust/pull/120144/>`_

  * :dp:`fls_7gae0TTCRc3v`
    No change: Lints are not part of the FLS.

* :dp:`fls_sH93ZYSVlOOv`
  `Enable debuginfo in macros, and stabilize \`-C collapse-macro-debuginfo\` and \`#[collapse_debuginfo]\`. <https://github.com/rust-lang/rust/pull/120845/>`_

  * :dp:`fls_7gae0TTCRc3v`
    New section: Attribute `collapse_debuginfo` :p:`fls_qyudjGHZfyJH`

* :dp:`fls_0bG3GNvXbmiS`
  `Propagate temporary lifetime extension into \`if\` and \`match\` expressions. <https://github.com/rust-lang/rust/pull/121346/>`_

  * :dp:`fls_t42bsw6HdHCS`
    New paragraphs

    * :dp:`fls_exj3aCyohLLd`
      :p:`fls_Rj9zhVutfQod`

    * :dp:`fls_XzTrtPQRZYbg`
      :p:`fls_oodpp3LpXC13`

    * :dp:`fls_h7crW9ZLee4J`
      :p:`fls_xGThCPoTUSAi`

* :dp:`fls_jnWGDInGZLL9`
  `Restrict promotion of \`const fn\` calls. <https://github.com/rust-lang/rust/pull/121557/>`_

  * :dp:`fls_Ics3st856gPP`
    No change: This is already specified in values :p:`fls_3h5vr7xk2rrt`

* :dp:`fls_zKkF8t5P9LnS`
  `Warn against refining impls of crate-private traits with \`refining_impl_trait\` lint. <https://github.com/rust-lang/rust/pull/121720/>`_

  * :dp:`fls_EjcpyoBxaI8Z`
    No change: Lints are not part of the FLS.

* :dp:`fls_NT29OnE6BTsu`
  `Stabilize associated type bounds (RFC 2289). <https://github.com/rust-lang/rust/pull/122055/>`_

  * :dp:`fls_ZuyjKP2KfCAV`
    New paragraph: :p:`fls_mcUMWsYcxzmZ`

* :dp:`fls_L5lm8jUfg3ju`
  `Stabilize importing \`main\` from other modules or crates. <https://github.com/rust-lang/rust/pull/122060/>`_

  * :dp:`fls_FE6Rujg0JzpG`
    Various changes:

    * :dp:`fls_FE6Rujg0JzpG`
      New section: Program Entry Point :p:`fls_8JB3SJqamdpU`

    * :dp:`fls_gSXmkIYYhTfP`
      New definition: :t:`crate type` :p:`fls_unxalgMqIr3v` :p:`fls_e7jGvXvTsFpC` :p:`fls_kQiJPwb2Hjcc`

    * :dp:`fls_b3x3YUEfjKZy`
      Removal of definition `main function`

    * :dp:`fls_b3x3YUEfjKZy`
      binary crate and proc-macro crate rewritten :p:`fls_9ub6ks8qrang` :p:`fls_Mf62VqAhoZ3c`

    * :dp:`fls_Xtl4nwOusufp`
      library crate removed

    * :dp:`fls_b9cUKnX6rji2`
      Introduction of `main function signature` functions :p:`fls_sbGnkm8Ephiu`

* :dp:`fls_uvV5kAESyCEp`
  `Check return types of function types for well-formedness <https://github.com/rust-lang/rust/pull/115538>`_

  * :dp:`fls_vTQTiu4TVDdz`
    No change: This is a bug fix in the trait resolution which is not described in the FLS.

* :dp:`fls_KNfRdLR5TCJl`
  `Rework \`impl Trait\` lifetime inference <https://github.com/rust-lang/rust/pull/116891/>`_

  * :dp:`fls_0z5Tru7S30qq`
    No change: Capturing of lifestime within \`impl Trait\` types is not described in the FLS.

* :dp:`fls_l87kw1tZR9Op`
  `Change inductive trait solver cycles to be ambiguous <https://github.com/rust-lang/rust/pull/122791>`_

  * :dp:`fls_w1EhKv4rXFx8`
    No change: The trait solver is not part of the FLS.

.. _fls_L2xtE9W3fzl6:

Version 1.78.0 (2024-05-02)
---------------------------

* :dp:`fls_r1ypVWRwRO3R`
  `Stabilize \`#[cfg(target_abi = ...)]\` <https://github.com/rust-lang/rust/pull/119590/>`_

  * :dp:`fls_PFmLCi644GtO`
    No change: \`cfg\` and \`cfg_attr\` configuration predicates are not part of the FLS.

* :dp:`fls_dgJ261l3EW21`
  `Stabilize the \`#[diagnostic]\` namespace and \`#[diagnostic::on_unimplemented]\` attribute <https://github.com/rust-lang/rust/pull/119888/>`_

  * :dp:`fls_NsIMHy9kDXYw`
    No change: Tool attributes are not part of the FLS.

* :dp:`fls_K642a6dH3DOE`
  `Make async-fn-in-trait implementable with concrete signatures <https://github.com/rust-lang/rust/pull/120103/>`_

  * :dp:`fls_KA0eXH2moxGW`
    No change: No paragraph in the FLS forbids this prior incompatability

* :dp:`fls_mGo1ObKqvauG`
  `Make matching on NaN a hard error, and remove the rest of \`illegal_floating_point_literal_pattern\` <https://github.com/rust-lang/rust/pull/116284/>`_

  * :dp:`fls_rerCWvAc0vU4`
    New paragraph: :p:`fls_JP8YSbxSN0Ym`

* :dp:`fls_rerCWvAc0vU4`
  `static mut: allow mutable reference to arbitrary types, not just slices and arrays <https://github.com/rust-lang/rust/pull/117614/>`_

  * :dp:`fls_rerCWvAc0vU4`
    No change: This lifted restriction is not described in the FLS.

* :dp:`fls_J2h9pGT9tGP0`
  `Extend \`invalid_reference_casting\` to include references casting to bigger memory layout <https://github.com/rust-lang/rust/pull/118983/>`_

  * :dp:`fls_grfw95iCYdZa`
    No change: Lints are not part of the FLS.

* :dp:`fls_sPzJMf8GbyoC`
  `Add \`non_contiguous_range_endpoints\` lint for singleton gaps after exclusive ranges <https://github.com/rust-lang/rust/pull/118879/>`_

  * :dp:`fls_etIqYVoecqrE`
    No change: Lints are not part of the FLS.

* :dp:`fls_QFXTGAxM46Cx`
  `Add \`wasm_c_abi\` lint for use of older wasm-bindgen versions <https://github.com/rust-lang/rust/pull/117918/>`_

  * :dp:`fls_lWhlYmqX8WAf`
    No change: Lints are not part of the FLS.

* :dp:`fls_7pkPlAfcIDes`
  `Update \`indirect_structural_match\` and \`pointer_structural_match\` lints to match RFC <https://github.com/rust-lang/rust/pull/120423/>`_

  * :dp:`fls_VxtpIUFcaOaD`
    No change: Lints are not part of the FLS.

* :dp:`fls_iyJF0c5sYNgt`
  `Make non-\`PartialEq\`-typed consts as patterns a hard error <https://github.com/rust-lang/rust/pull/120805/>`_

  * :dp:`fls_8xkvaTWDsjnu`
    No change: This behavior is already described in the FLS in :p:`fls_zCswsyuitexI`

* :dp:`fls_N7Bc0yYbybZV`
  `Split \`refining_impl_trait\` lint into \`_reachable\`, \`_internal\` variants <https://github.com/rust-lang/rust/pull/121720/>`_

  * :dp:`fls_9MCZ2zPE4rGq`
    No change: Lints are not part of the FLS.

* :dp:`fls_onjwaXIGx3iR`
  `Remove unnecessary type inference when using associated types inside of higher ranked \`where\`-bounds <https://github.com/rust-lang/rust/pull/119849>`_

  * :dp:`fls_qBYEI3IahJMh`
    No change: The FLS does not specify type inference to such a degree.

* :dp:`fls_OD9Icyu1uWjS`
  `Weaken eager detection of cyclic types during type inference <https://github.com/rust-lang/rust/pull/119989>`_

  * :dp:`fls_0DUycmb1P2Os`
    No change: The FLS does not specify type inference to such a degree.

* :dp:`fls_0DUycmb1P2Os`
  `\`trait Trait: Auto {}\`: allow upcasting from \`dyn Trait\` to \`dyn Trait + Auto\` <https://github.com/rust-lang/rust/pull/119338>`_

.. _fls_rDta70LeEpBl:

Version 1.77.0 (2024-03-21)
---------------------------

* :dp:`fls_QgJ561lpEWUi`
  `Reveal opaque types within the defining body for exhaustiveness checking. <https://github.com/rust-lang/rust/pull/116821/>`_

  * :dp:`fls_oQuyRstkj66J`
    No change: The introspection of the concrete type of the match expression scrutinee is not discussed in paragraph :p:`fls_e02um1gb89d0`

* :dp:`fls_QDMaabnEQ73n`
  `Stabilize C-string literals. <https://github.com/rust-lang/rust/pull/117472/>`_

  * :dp:`fls_qVDTWYszGfym`
    New section: :p:`fls_U1gHCy16emVe`

* :dp:`fls_b4OXjAKJACyj`
  `Stabilize THIR unsafeck. <https://github.com/rust-lang/rust/pull/117673/>`_

  * :dp:`fls_JgalEDgK0Bsz`
    No change: Not a language change.

* :dp:`fls_YU2GiEPMgtTs`
  `Add lint \`static_mut_refs\` to warn on references to mutable statics. <https://github.com/rust-lang/rust/pull/117556/>`_

  * :dp:`fls_XM7iKLpkqHSW`
    No change: Lints are not part of the FLS.

* :dp:`fls_fbhpU8aeBdi6`
  `Support async recursive calls (as long as they have indirection). <https://github.com/rust-lang/rust/pull/117703/>`_

  * :dp:`fls_UOGatsGucUFV`
    No change: This async restriction is not described in the FLS.

* :dp:`fls_EhBt0aHL7deR`
  `Undeprecate lint \`unstable_features\` and make use of it in the compiler. <https://github.com/rust-lang/rust/pull/118639/>`_

  * :dp:`fls_scU89ha8iV7z`
    No change: Lints are not part of the FLS.

* :dp:`fls_1YbvTHuZaB20`
  `Make inductive cycles in coherence ambiguous always. <https://github.com/rust-lang/rust/pull/118649/>`_

  * :dp:`fls_6iBlIG98rAwR`
    No change: The trait solver in coherence resolution is not part of the FLS

* :dp:`fls_p6qz5mUXFdjX`
  `Get rid of type-driven traversal in const-eval interning <https://github.com/rust-lang/rust/pull/119044/>`_, only as a `future compatibility lint <https://github.com/rust-lang/rust/pull/122204>`_ for now.

  * :dp:`fls_8xQA9b8ItnNo`
    No change: This restriction of const promotion is not described in the FLS.

* :dp:`fls_RIZfxwq10lkv`
  `Deny braced macro invocations in let-else. <https://github.com/rust-lang/rust/pull/119062/>`_

  * :dp:`fls_OAopLKUtBleB`
    New paragraph: :p:`fls_1s1UikGU5YQb`
