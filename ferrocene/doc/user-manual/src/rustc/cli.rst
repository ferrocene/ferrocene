.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

``rustc`` Command-Line Interface
================================

.. cli:program:: rustc

   .. cli:option:: -A <lints>
      :category: narrow

      Compiler argument ``-A`` indicates which lints are suppressed. A
      suppressed lint is never checked and does not emit a diagnostic.

      ``<lints>`` must be a comma-separated list of lint names and lint
      category names. The list must contain at least one element.

      Example:

      .. code-block::

         $ rustc -A warnings,absolute-paths-not-starting-with-crate

      The effects of compiler argument ``-A`` depend on its position within the
      sequence of compiler arguments and the presence of other lint-related
      compiler arguments, where precedence increases from left to right.

      A list of size ``N`` is equivalent to ``N`` compiler arguments ``-A``,
      one for each element.

      Example:

      .. code-block::

         $ rustc -A warnings -A absolute-paths-not-starting-with-crate

   .. cli:option:: -C codegen-units=<number>
      :category: narrow

      Code generation option ``codegen-units`` instructs the compiler to
      split the crate being compiled into multiple units. This may increase
      parallelism, but may also produce slower code.

      ``<number>`` must be a positive number.

      Example:

      .. code-block::

         $ rustc -C codegen-units=4 my_program.rs

      Multiple ``codegen-units`` code generation options are allowed on the
      command line, where precedence increases from left to right.

   .. cli:option:: -C debug-assertions=<flag>
      :category: narrow

      Code generation option ``debug-assertions`` enables (or disables)
      conditional compilation that is predicated on attribute
      ``#[cfg(debug_assertions)]``.

      ``<flag>`` must be either ``off``, ``on``, ``n``, ``no``, ``y``, or
      ``yes``, or can be altogether missing.

      The effects of ``<flag>`` are as follows:

      ================================ ==========================
      ``<flag>``                       effects
      ================================ ==========================
      ``on``, ``y``, ``yes``, no value Enables debug assertions.
      ``off``, ``n``, ``no``           Disables debug assertions.
      ================================ ==========================

      Example:

      .. code-block::

         $ rustc -C debug-assertions=yes my_program.rs

      Multiple ``debug-assertions`` code generation options are allowed on the
      command line, where precedence increases from left to right.

   .. cli:option:: -C debuginfo=<level>
      :category: narrow

      Code generation option ``debuginfo`` indicates the level of detail of
      the debug information produced by the compiler.

      ``<level>`` must be either ``0``, ``1``, or ``2``.

      The effects of ``<level>`` are as follows:

      =========== ===================================
      ``<level>`` effects
      =========== ===================================
      ``0``       No debug information is produced.
      ``1``       Only line tables are produced.
      ``2``       Full debug information is produced.
      =========== ===================================

      Example:

      .. code-block::

         $ rustc -C debuginfo=2 my_program.rs

      Multiple ``debuginfo`` code generation options are allowed on the command
      line, where precedence increases from left to right.

   .. cli:option:: -C extra-filename=<suffix>
      :category: narrow

      Code generation option ``extra-filename`` appends a suffix to the name
      of each output file.

      ``<suffix>`` must be a string literal.

      Example:

      .. code-block::

         $ rustc -C extra-filename="_backup" my_program.rs

      Multiple ``extra-filename`` code generation options are allowed on the
      command line, where precedence increases from left to right.

   .. cli:option:: -C llvm-args=<args>
      :category: unqualified

      Code generation option ``llvm-args`` can be used to pass a list of arguments
      directly to LLVM.
      The list must be separated by spaces.

      Example:

      .. code-block::

         $ rustc -C llvm-args=--inline-threshold=123 my_program.rs

   .. cli:option:: -C link-arg=<arg>
      :category: wide

      .. caution::

         Only the values listed in :doc:`Compilation targets </targets/index>`
         section for each target can be used in a safety critical context.

      Code generation option ``link-arg`` appends a single extra argument to
      the invocation of the linker.

      ``<arg>`` must be a valid linker argument.

      Example:

      .. code-block::

         $ rustc -C link-arg=--arch=sm_60 my_program.rs

      The effects of code generation option ``link-arg`` depend on its position
      within the sequence of linker argument-related code generation options,
      where precedence increases from left to right.

      Multiple ``link-arg`` code generation options are allowed on the command
      line.

   .. cli:option:: -C link-args=<args>
      :category: wide

      .. caution::

         Only the values listed in :doc:`Compilation targets </targets/index>`
         section for each target can be used in a safety critical context.

      Code generation option ``link-args`` appends multiple extra arguments to
      the invocation of the linker.

      ``<args>`` must be a space-separated list of valid linker arguments. The
      list must contain at least one element.

      Example:

      .. code-block::

         $ rustc -C link-args="-pie --relax" my_program.rs

      The effects of code generation option ``link-args`` depend on its
      position within the sequence of linker argument-related code generation
      options, where precedence increases from left to right.

      Multiple ``link-args`` code generation options are allowed on the command
      line.

   .. cli:option:: -C link-dead-code=<flag>
      :category: narrow

      Code generation option ``link-dead-code`` indicates whether dead code
      is linked.

      ``<flag>`` must be either ``off``, ``on``, ``n``, ``no``, ``y``, or
      ``yes``, or can be altogether missing.

      The effects of ``<flag>`` are as follows:

      ================================ ===============================
      ``<flag>``                       effects
      ================================ ===============================
      ``on``, ``y``, ``yes``, no value Links dead code.
      ``off``, ``n``, ``no``           Default. Do not link dead code.
      ================================ ===============================

      Example:

      .. code-block::

         $ rustc -C link-dead-code=yes my_program.rs

      Multiple ``link-dead-code`` code generation options are allowed on the
      command line, where precedence increases from left to right.

   .. cli:option:: -C linker=<path>
      :category: wide

      .. caution::

         This can only be used for targets that require a linker driver,
         in which case the linker driver must adhere to the
         :ref:`linker options <linker-options>` restrictions.

      Code generation option ``linker`` indicates the path to the linker. If
      this compiler argument is not specified, then the linker is inferred
      based on the target.

      ``<path>`` must denote a valid path.

      Example:

      .. code-block::

         $ rustc -C linker=/usr/local/bin my_program.rs

      Multiple ``linker`` code generation options are allowed on the command
      line, where precedence increases from left to right.

   .. cli:option:: -C linker-flavor=<flavor>
      :category: wide

      .. caution::

         Only the flavors listed in :doc:`Compilation targets </targets/index>`
         section for each target can be used in a safety critical context.

      Code generation option ``linker-flavor`` indicates the flavor of the
      linker.

      If the linker is specified using compiler argument ``-C linker``, then
      the flavor is inferred.

      If no linker is specified, then the flavor is used to select the linker
      to use.

      ``<flavor>`` must be either ``bpf-linker``, ``em``, ``gcc``, ``ld``,
      ``ld.lld``, ``ld64.lld``, ``lld-link``, ``msvc``, ``ptx-linker``,
      ``wasm-ld``.

      The effects of ``<flavor>`` are as follows:

      ============== =======================================================
      ``<flavor>``   effects
      ============== =======================================================
      ``bpf-linker`` Use ``bpf-linker`` for eBPF support.
      ``em``         Use Emscripten ``emcc``.
      ``gcc``        Use ``cc``.
      ``ld``         Use ``ld``.
      ``ld.lld``     Use LLVM ``lld`` with the ``-flavor gnu`` flag for GNU
                     binutils' ``ld``.
      ``ld64.lld``   Use LLVM ``lld`` with the ``-flavor darwin`` flag for
                     Apple's ``ld``.
      ``lld-link``   Use LLVM ``lld`` with the ``-flavor link`` flag for
                     Microsoft's ``link``.
      ``mscv``       Use ``link`` from Microsoft Visual Studio.
      ``ptx-linker`` Use ``rust-ptx-linker`` for Nvidia NVPTX GPGPU support.
      ``wasm-ld``    Use ``wasm-ld``.
      ============== =======================================================

      Example:

      .. code-block::

         $ rustc -C linker-flavor=gcc my_program.rs

      Multiple ``linker-flavor`` code generation options are allowed on the
      command line.

   .. cli:option:: -C metadata=<data>
      :category: narrow

      Code generation option ``metadata`` enhances symbol mangling by supplying
      additional data used in the hashed suffixes of symbols.

      ``<data>`` must be a comma-separated list of string literals. The list
      must contain at least one element.

      Example:

      .. code-block::

         $ rustc -C metadata=prod,arm32 my_program.rs

      Multiple ``metadata`` code generation options are allowed on the command
      line, where precedence increases from left to right.

   .. cli:option:: -C no-vectorize-loops
      :category: unqualified

      Code generation option ``no-vectorize-loops`` disables loop
      vectorization.

      Example:

      .. code-block::

         $ rustc -C no-vectorize-loops my_program.rs

      Multiple ``no-vectorize-loops`` code generation options are allowed on
      the command line, where precedence increases from left to right.

   .. cli:option:: -C opt-level=<level>
      :category: unqualified

      Code generation option ``opt-level`` indicates the optimization level in
      effect.

      ``<level>`` must be either ``0``, ``1``, ``2``, ``3``, ``s``, or ``z``.

      The effects of ``<level>`` are as follows:

      =========== ==========================================================
      ``<level>`` effects
      =========== ==========================================================
      ``0``       No optimizations.
      ``1``       Basic optimizations.
      ``2``       Some optimizations.
      ``3``       All optimizations.
      ``s``       Optimize for binary size.
      ``z``       Optimize for binary size with disabled loop vectorization.
      =========== ==========================================================

      .. code-block::

         $ rustc -C opt-level=2 my_program.rs

      Multiple ``opt-level`` code generation options are allowed on the command
      line, where precedence increases from left to right.

   .. cli:option:: -C overflow-checks=<flag>
      :category: narrow

      Code generation option ``overflow-checks`` enables (or disables)
      checks on runtime integer overflow. If overflow checks are enabled
      and integer overflow occurs, then the code panics.

      ``<flag>`` must be either ``off``, ``on``, ``n``, ``no``, ``y``, or
      ``yes``, or can be altogether missing.

      The effects of ``<flag>`` are as follows:

      ================================ =========================
      ``<flag>``                       effects
      ================================ =========================
      ``on``, ``y``, ``yes``, no value Enables overflow checks.
      ``off``, ``n``, ``no``           Disables overflow checks.
      ================================ =========================

      Example:

      .. code-block::

         $ rustc -C overflow-checks=yes my_program.rs

      Multiple ``overflow-checks`` code generation options are allowed on the
      command line, where precedence increases from left to right.

   .. cli:option:: -C panic=<behavior>
      :category: narrow

      Code generation option ``panic`` indicates the behavior of panics.

      ``<behavior>`` must be either ``abort`` or ``unwind``.

      The effects of ``<behavior>`` are as follows:

      ============== ==================================
      ``<behavior>`` effects
      ============== ==================================
      ``abort``      The process terminates upon panic.
      ``unwind``     The stack unwinds upon panic.
      ============== ==================================

      Example:

      .. code-block::

         $ rustc -C panic=abort my_program.rs

      Multiple ``panic`` code generation options are allowed on the command
      line, where precedence increases from left to right.

   .. cli:option:: -C prefer-dynamic=<flag>
      :category: narrow

      Code generation option ``prefer-dynamic`` indicates whether dynamic
      linking is preferable when both a static and a dynamic version of a
      library are available.

      ``<flag>`` must be either ``off``, ``on``, ``n``, ``no``, ``y``, or
      ``yes``, or can be altogether missing.

      The effects of ``<flag>`` are as follows:

      ================================ ============================
      ``<flag>``                       effects
      ================================ ============================
      ``on``, ``y``, ``yes``, no value Use dynamic linking.
      ``off``, ``n``, ``no``           Default. Use static linking.
      ================================ ============================

      Example:

      .. code-block::

         $ rustc -C prefer-dynamic=y my_program.rs

      Multiple ``prefer-dynamic`` code generation options are allowed on the
      command line, where precedence increases from left to right.

   .. cli:option:: -C relocation-model=<model>
      :category: unqualified

      Code generation option ``relocation-model`` enables the generation of
      position-independent code.

      ``<model>`` is set to ``static`` on ARM architectures.

      Multiple ``relocation-model`` code generation options are allowed on the
      command line, where precedence increases from left to right.

   .. cli:option:: -C rpath=<flag>
      :category: narrow

      Code generation option ``rpath`` indicates whether the run-time search
      path is enabled (or disabled).

      ``<flag>`` must be either ``off``, ``on``, ``n``, ``no``, ``y``, or
      ``yes``, or can be altogether missing.

      The effects of ``<flag>`` are as follows:

      ================================ =====================================
      ``<flag>``                       effects
      ================================ =====================================
      ``on``, ``y``, ``yes``, no value Enables the run-time search path.
      ``off``, ``n``, ``no``           Default. Disables the run-time search
                                       path.
      ================================ =====================================

      Example:

      .. code-block::

         $ rustc -C rpath=no my_program.rs

      Multiple ``rpath`` code generation options are allowed on the command
      line, where precedence increases from left to right.

   .. cli:option:: -C target-cpu=<cpu>
      :category: unqualified

      Code generation option ``target-cpu`` indicates the CPU of the target to
      generate code for.

      ``<cpu>`` must be one of the CPU kinds reported by compiler argument
      ``--print target-cpus``.

      Example:

      .. code-block::

         $ rustc -C target-cpu=x86-64 my_program.rs

      Multiple ``target-cpu`` code generation options are allowed on the
      command line, where precedence increases from left to right.

   .. cli:option:: -C target-feature=<features>
      :category: unqualified

      Code generation option ``target-feature`` enables (or disables) a
      feature of the target.

      This code generation option is **unsafe** and may result in undefined
      behavior.

      ``<features>`` must be a comma-separated list of feature details. The
      list must contain at least one element.

      The valid feature details and their effects are as follows:

      ============== =====================
      feature detail effects
      ============== =====================
      ``+FEATURE``   Enables the feature.
      ``-FEATURE``   Disables the feature.
      ============== =====================

      ``FEATURE`` must be one of the features reported by compiler argument
      ``--print target-features``.

      The effects of compiler argument ``-C target-feature`` depends on its
      position within the sequence of compiler arguments and the presence of
      other such compiler arguments, where precedence increases from left to
      right.

      Examples:

      .. code-block::

         $ rustc -C target-feature=adx,f16c my_program.rs

      Multiple ``target-feature`` code generation options are allowed on the
      command line.

   .. cli:option:: --cap-lints <level>
      :category: narrow

      Compiler argument ``--cap-lints`` indicates the overall diagnostic level
      of all lints.

      ``<level>`` must be either ``allow``, ``deny``, ``forbid``, or ``warn``.

      Example:

      .. code-block::

         $ rustc --cap-lints warn my_program.rs

      Specifying a particular ``level`` enacts the effects of the following
      lint-related compiler arguments for all lints:

      =========== =================
      ``<level>`` compiler argument
      =========== =================
      ``allow``   ``-A``
      ``deny``    ``-D``
      ``forbid``  ``-F``
      ``warn``    ``-W``
      =========== =================

      The effects of compiler argument ``--cap-lints`` depend on its position
      within the sequence of compiler arguments and the presence of other
      lint-related compiler arguments, where precedence increases from left to
      right.

      Multiple ``--cap-lints`` compiler arguments are allowed on the command
      line.

   .. cli:option:: --cfg <option>
      :category: narrow

      Compiler argument ``--cfg`` specifies conditional compilation option
      keys and values.

      ``<option>`` must either denote a key of the form ``key`` or a key-value
      pair of the form ``key="value"``.

      Example:

      .. code-block::

         $ rustc --cfg verbose my_program.rs
         $ rustc --cfg feature="serde" my_program.rs

      A compiler argument of the form ``--cfg key`` corresponds to attribute
      ``#[cfg(key)]``.

      A compiler argument of the form ``--cfg key="value"`` corresponds to
      attribute ``#[cfg(key = "value")]``.

      Multiple ``--cfg`` compiler arguments are allowed on the command line.

   .. cli:option:: --color <option>
      :category: informational

      Compiler argument ``--color`` sets the color of the output.

      ``<option>`` must denote either ``always``, ``auto``, or ``never``,
      where ``auto`` is the default.

      The effects of ``<option>`` are as follows:

      ============ ======================================================
      ``<option>`` effects
      ============ ======================================================
      ``always``   Always use color in the output.
      ``auto``     Default. Use color only when the output goes to a TTY.
      ``never``    Never use color in the output.
      ============ ======================================================

      Example:

      .. code-block::

         $ rustc --color always my_program.rs

      Only one ``--color`` compiler argument is allowed on the command line.

   .. cli:option:: --crate-name <name>
      :category: narrow

      Compiler argument ``--crate-name`` specifies the name of the crate to
      build.

      ``<name>`` must be a string literal.

      Example:

      .. code-block::

         $ rustc --crate-name my_crate_name my_program.rs

      If the crate root module is subject to attribute ``crate_name``, then
      ``<name>`` and the name specified by the attribute must be the same.

      Only one ``--crate-name`` compiler argument is allowed on the command
      line.

   .. cli:option:: --crate-type <types>
      :category: narrow

      .. caution::

         Only the ``bin``, ``lib``, ``rlib`` and ``staticlib`` crate types can
         be used in a safety critical context.

      Compiler argument ``--crate-type`` specifies the type of crate to build.

      ``<types>`` must be a comma-separated list of crate types. The list must
      contain at least one element.

      The valid crate types are:

      ============== ======================================================
      ``<types>``
      ============== ======================================================
      ``bin``        A runnable executable.
      ``cdylib``     A native dynamic library.
      ``dylib``      A Rust dynamic library.
      ``lib``        A compiler-favored library kind, defaults to ``rlib``.
      ``proc-macro`` A procedural macro library.
      ``rlib``       A Rust static library.
      ``staticlib``  A native static library.
      ============== ======================================================

      Example:

      .. code-block::

         $ rustc --crate-type dylib,rlib my_library.rs

      If the crate root module is subject to attribute ``crate_type``, then
      ``<types>`` overrides the types specified by the attribute.

      A list of size ``N`` is equivalent to ``N`` compiler arguments
      ``--crate-type``, one for each element.

      Example:

      .. code-block::

         $ rustc --crate-type dylib --crate-type rlib my_library.rs

   .. cli:option:: -D <lints>
      :category: narrow

      Compiler argument ``-D`` indicates which lints emit their diagnostics as
      errors. This compiler argument may be used to treat warnings as errors.
      An error stops compilation while a warning does not.

      ``<lints>`` must be a comma-separated list of lint names and lint
      category names. The list must contain at least one element.

      Example:

      .. code-block::

         $ rustc -D warnings,absolute-paths-not-starting-with-crate

      The effects of compiler argument ``-D`` depend on its position within the
      sequence of compiler arguments and the presence of other lint-related
      compiler arguments, where precedence increases from left to right.

      A list of size ``N`` is equivalent to ``N`` compiler arguments ``-D``,
      one for each element.

      Example:

      .. code-block::

         $ rustc -D warnings -D absolute-paths-not-starting-with-crate

   .. cli:option:: --edition <edition>
      :category: wide

      .. caution::

         Only edition ``2021`` is within the scope of the Ferrocene
         qualification.

      Compiler argument ``--edition`` indicates the edition of the Rust
      programming language used during compilation.

      ``<edition>`` must denote either ``2015``, ``2018``, or ``2021``, where
      ``2015`` is the default.

      Example:

      .. code-block::

         $ rustc --edition 2021 my_program.rs

      Only one ``--edition`` compiler argument is allowed on the command line.

   .. cli:option:: --emit <kinds>
      :category: narrow

      Compiler argument ``--emit`` indicates which kind of output to emit.

      ``<kinds>`` must be a comma-separated list of output kinds. The list must
      contain at least one element.

      The valid output kinds and their effects are as follows:

      ============ ========================================================
      ``<kinds>``  effects
      ============ ========================================================
      ``dep-info`` Generate file ``CRATE_NAME.d`` which indicates the files
                   loaded when generating the crate, using Makefile syntax.
      ``metadata`` Generate file ``libCRATE_NAME.rmeta`` which contains
                   metadata about the crate.
      ============ ========================================================

      Example:

      .. code-block::

         $ rustc --emit dep-info,metadata my_program.rs

      A list of size ``N`` is equivalent to ``N`` compiler arguments
      ``--emit``, one for each element.

      Example:

      .. code-block::

         $ rustc --emit dep-info --emit metadata my_program.rs

   .. cli:option:: --error-format <kind>
      :category: informational

      Indicate which kind of error format to use.

      Compiler argument ``--error-format`` indicates which error format to use
      when emitting diagnostics.

      ``<kind>`` must denote either ``human``, ``json``, or ``short``, where
      ``human`` is the default.

      The effects of ``<kind>`` are as follows:

      ========== ========================================
      ``<kind>`` effects
      ========== ========================================
      ``human``  Default. Generate human-readable output.
      ``json``   Generate structured JSON output.
      ``short``  Generate one-line diagnostics.
      ========== ========================================

      Example:

      .. code-block::

         $ rustc --error-format short my_program.rs

      Only one ``--error-format`` compiler argument is allowed on the command
      line.

   .. cli:option:: --explain <code>
      :category: informational

      Compiler argument ``--explain`` outputs a verbose explanation of an error
      denoted by a code.

      ``<code>`` must have the form ``Ennnn``, where each ``n`` denotes a digit
      from 0 to 9.

      Example:

      .. code-block::

         $ rustc --explain E0708

      Only one ``--explain`` compiler argument is allowed on the command line.

   .. cli:option:: --extern <details>
      :category: narrow

      Compiler argument ``--extern`` indicates the name and location of an
      external crate. The crate is added to the external prelude, and will
      be linked only when used.

      ``<details>`` must be a comma-separated list of crate details. The list
      must contain at least one element.

      The valid crate details and their effects are as follows:

      =================== =================================================
      crate details       effects
      =================== =================================================
      ``CRATE_NAME``      Indicates the name of the crate within the search
                          path.
      ``CRATE_NAME=PATH`` Indicates the name and path of the crate.
      =================== =================================================

      ``CRATE_NAME`` does not have to be unique within the union of all
      ``<details>``. If the same ``CRATE_NAME`` is specified with and without a
      ``PATH``, the version with ``PATH`` takes precedence.

      Example:

      .. code-block::

         $ rustc --extern \
                 my_library, \
                 my_other_library=/usr/lib/libmy_other_library.so \
                 my_program.rs

      A list of size ``N`` is equivalent to ``N`` compiler arguments
      ``--extern``, one for each element.

      Example:

      .. code-block::

         $ rustc --extern my_library \
                 --extern my_other_library=/usr/lib/libmy_other_library.so \
                 my_program.rs

   .. cli:option:: -F <lints>
      :category: narrow

      Compiler argument ``-F`` indicates which lints always emit their
      diagnostics as errors, regardless of whether other lint-related compiler
      arguments are present. This compiler argument may be used to treat
      warnings as errors. An error stops compilation while a warning does not.

      ``<lints>`` must be a comma-separated list of lint names and lint
      category names. The list must contain at least one element.

      Example:

      .. code-block::

         $ rustc -F warnings,absolute-paths-not-starting-with-crate

      The effects of compiler argument ``-F`` depend on its position within the
      sequence of compiler arguments and the presence of other lint-related
      compiler arguments, where precedence increases from left to right.

      A list of size ``N`` is equivalent to ``N`` compiler arguments ``-W``,
      one for each element.

      Example:

      .. code-block::

         $ rustc -F warnings -F absolute-paths-not-starting-with-crate

   .. cli:option:: -g
      :category: narrow

      Compiler argument ``-g`` is identical to compiler argument
      ``-C debuginfo=2``.

   .. cli:option:: -h
      :category: informational

      Compiler option ``-h`` is identical to compiler argument ``--help``.

   .. cli:option:: --help
      :category: informational

      Compiler argument ``--help`` emits usage and help information.

      Example:

      .. code-block::

         $ rustc -help

   .. cli:option:: --json <format>
      :category: informational

      Compiler argument ``--json`` indicates the format of the JSON output when
      compiler argument ``--error-format json`` is in effect.

      ``<format>`` must denote either ``artifacts``, ``diagnostic-short``,
      ``diagnostic-rendered-ansi``, or ``future-incompat``.

      The effects of ``<format>`` are as follows:

      ============================ =============================================
      ``<format>``                 effects
      ============================ =============================================
      ``artifacts``                Output a JSON blob for each artifact produced
                                   by compiler argument ``--emit``.
      ``diagnostic-short``         Output a JSON blob as if compiler argument
                                   ``--error-format short`` is in effect.
      ``diagnostic-rendered-ansi`` Output a JSON blob with ANSI color codes.
      ``future-incompat``          Output a JSON blob indicating future
                                   incompatibilities in the code.
      ============================ =============================================

      Example:

      .. code-block::

         $ rustc --error-format json --json artifacts my_program.rs

      Only one ``--json`` compiler argument is allowed on the command line.

   .. cli:option:: -L <details>
      :category: narrow

      Compiler argument ``-L`` indicates the kind and location of a search
      path.

      ``<details>`` must be a comma-separated list of path details. The list
      must contain at least one element.

      The valid path details and their effects are as follows:

      =================== ===================================================
      path details        effects
      =================== ===================================================
      ``PATH``            Identical to ``all=PATH``.
      ``all=PATH``        Add a search path for all kinds of dependencies and
                          libraries.
      ``crate=PATH``      Add a search path for direct dependencies only.
      ``dependency=PATH`` Add a search path for transitive dependencies only.
      ``native=PATH``     Add a search path for native libraries only.
      =================== ===================================================

      Example:

      .. code-block::

         $ rustc -L libraries,native=include/c_libraries my_program.rs

      A list of size ``N`` is equivalent to ``N`` compiler arguments ``-L``,
      one for each element.

      Example:

      .. code-block::

         $ rust -L libraries -L native=include/c_libraries my_program.rs

   .. cli:option:: -l <details>
      :category: narrow

      Compiler argument ``-l`` indicates a native library to link.

      ``<details>`` must be a comma-separated list of library details. The list
      must contain at least one element.

      The valid library details and their effects are as follows:

      =============== =====================================================
      library details effects
      =============== =====================================================
      ``NAME``        Link a native library denoted by its name.
      ``KIND=NAME``   Link a specific kind of native library denoted by its
                      name.
      =============== =====================================================

      A valid ``NAME`` is as follows:

      ===================== =================================================
      ``NAME``
      ===================== =================================================
      ``ACTUAL_NAME``       The actual name of the native library.
      ``ALIAS:ACTUAL_NAME`` The actual name of the native library bound to an
                            alias.
      ===================== =================================================

      A valid ``KIND`` is as follows:

      ====================== ===============================================
      ``KIND``
      ====================== ===============================================
      ``LIB_KIND``           The kind of the native library, ``dylib`` for a
                             dynamic native library, ``static`` for a static
                             native library.
      ``LIB_KIND:MODIFIERS`` The kind of the native library subject to
                             modifiers.
      ====================== ===============================================

      ``MODIFIERS`` is a comma-separated list of library modifiers. The list
      must contain at least one element. Specifying multiple identical library
      modifiers is not supported.

      Library modifiers are only compatible with the ``static`` ``KIND``. The
      valid library modifiers and their effects are as follows:

      ================== ======================================================
      library modifier   effects
      ================== ======================================================
      ``+bundle``        Default. Causes the static native library to be packed
                         into an rlib or a static archive, and then retrieved
                         from there during the linking of the final binary.
      ``-bundle``        Causes the static native library to be registered as a
                         dependency of an rlib "by name", and then retrieved
                         "by name" during the linking of the final binary.
      ``+whole-archive`` Causes the static native library to be linked as a
                         whole archive. This library modifier translates to
                         ``--whole-archive`` for ``ld``-like linkers, to
                         ``/WHOLEARCHIVE`` for ``link.exe``, and to
                         ``-force-load`` for ``ld64``.
      ``-whole-archive`` Default.
      ================== ======================================================

      Library modifiers ``+bundle`` and ``+whole-archive`` are mutually
      exclusive.

      Examples:

      .. code-block::

         $ rustc -l libmy_library my_program.rs
         $ rustc -l mine:libmy_library my_program.rs
         $ rustc -l static=libmy_static_library my_program.rs
         $ rustc -l static=mine:libmy_static_library my_program.rs
         $ rustc -l static+bundle=libmy_static_library my_program.rs
         $ rustc -l static+bundle=mine:libmy_static_library my_program.rs

   .. cli:option:: -O
      :category: wide

      Enable optimizations.

   .. cli:option:: -o <name>
      :category: narrow

      Compiler argument ``-o`` indicates the name of the compilation output
      file.

      ``<name>`` must be a string literal.

      Example:

      .. code-block::

         $ rustc -o driver my_program.rs

      Compiler argument ``-o`` causes compiler argument ``--out-dir`` to be
      ignored.

      Only one ``-o`` compiler argument is allowed on the command line.

   .. cli:option:: --out-dir <directory>
      :category: narrow

      Compiler argument ``--out-dir`` indicates the output directory.

      ``<directory>`` must be a valid path. It is not necessary for the path to
      exist prior to compilation.

      Example:

      .. code-block::

         $ rustc --out-dir my_project/bin my_program.rs

      Compiler argument ``--out-dir`` is ignored when compiler argument ``-o``
      is in effect.

      Only one ``--out-dir`` compiler argument is allowed on the command line.

   .. cli:option:: --print <option>
      :category: informational

      Compiler argument ``--print`` emits information about the compiler.

      ``<option>`` must denote either ``cfg``, ``crate-name``, ``link-args``,
      ``native-static-lib``, ``sysroot``, ``target-libdir``, ``target-list``.

      The effects of ``<option>`` are as follows:

      ===================== ===================================================
      ``<option>``          effects
      ===================== ===================================================
      ``cfg``               Outputs all keys and key-value pairs related to
                            conditional compilation that are in effect.
      ``crate-name``        Outputs the name of the crate.
      ``link-args``         Outputs the full linker invocation.
      ``native-static-lib`` Outputs the linker flags used when linking a static
                            library.
      ``sysroot``           Outputs the path to the compiler installation root.
      ``target-libdir``     Outputs the path to the target libdir.
      ``target-list``       Outputs a list of known targets.
      ===================== ===================================================

      Example:

      .. code-block::

         $ rustc --print file-names my_program.rs

      Multiple ``--print`` compiler arguments are allowed on the command line.

   .. cli:option:: --remap-path-prefix <from>=<to>
      :category: narrow

      Remap source path prefixes in all output.
      Compiler argument ``--remap-path-prefix`` causes path prefixes in all
      output to be replaced with another prefix.

      ``<from>`` must be a string literal.

      ``<to>`` must be a string literal, but exclude character 0x3D (equals
      sign).

      Example:

      .. code-block::

         $ rustc --remap-path-prefix /usr/bin=/my_project/executables

      Multiple ``--remap-path-prefix`` compiler arguments are allowed on the
      command line.

   .. cli:option:: --sysroot
      :category: narrow

      Compiler option ``--sysroot`` indicates the compiler installation root.

      Example:

      .. code-block::

         $ rustc --sysroot my_project/bin my_program.rs

      Only one ``--sysroot`` compiler argument is allowed on the command line.

   .. cli:option:: --target <target>
      :category: wide

      Compiler argument ``--target`` indicates the target tuple to build.

      ``<target>`` must denote **what is it for ARM?**

      Example:

      .. code-block::

         $ rustc --target ??? my_program.rs

      Only one ``--target`` compiler argument is allowed on the command line.

   .. cli:option:: --test
      :category: narrow

      Compiler argument ``--test`` builds a test harness in the form of an
      executable binary.

      Example:

      .. code-block::

         $ rustc --test my_program.rs

      Only one ``--test`` compiler argument is allowed on the command line.

   .. cli:option:: -V
      :category: informational

      Compiler argument ``-V`` is identical to compiler argument ``--version``.

   .. cli:option:: -v
      :category: informational

      Compiler argument ``-v`` is identical to compiler argument ``--verbose``.

   .. cli:option:: --verbose
      :category: informational

      Compiler argument ``--verbose`` emits verbose output.

      Example:

      .. code-block::

         $ rustc --verbose my_program.rs

      Only one ``--verbose`` compiler argument is allowed on the command line.

   .. cli:option:: --version
      :category: informational

      Compiler argument ``--version`` emits the compiler version.

      Example:

      .. code-block::

         $ rustc --version

      Only one ``--version`` compiler argument is allowed on the command line.

   .. cli:option:: -W <lints>
      :category: narrow

      Compiler argument ``-W`` indicates which lints emit their diagnostics as
      warnings. This compiler argument may be used to treat errors as warnings.
      An error stops compilation while a warning does not.

      ``<lints>`` must be a comma-separated list of lint names and lint
      category names. The list must contain at least one element.

      Example:

      .. code-block::

         $ rustc -W warnings,absolute-paths-not-starting-with-crate

      The effects of compiler argument ``-W`` depend on its position within the
      sequence of compiler arguments and the presence of other lint-related
      compiler arguments, where precedence increases from left to right.

      A list of size ``N`` is equivalent to ``N`` compiler arguments ``-W``,
      one for each element.

      Example:

      .. code-block::

         $ rustc -W warnings -W absolute-paths-not-starting-with-crate
