.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Requirements Management
=======================

Doc-comments
------------

Requirements are implemented as doc-comments.

The Doc-comments described below cover the single level of functional requirements for each function.

The documentation for each module covers the purpose and overview, and as such is suitable for the design requirement.

Doc-comments in general
~~~~~~~~~~~~~~~~~~~~~~~

Rust has a concept called "doc-comments” also known as documentation comments. They are denoted by triple-slashes, while normal comments are denoted by double-slashes. They support markdown, and code inside code blocks is automatically run as tests, to ensure the code and documentation do not get out of sync.

.. code-block:: rust
  :linenos:

  /// Add two `u32`s.
  ///
  /// ```
  /// assert_eq!(add(1, 5), 6);
  /// ```
  /// This is a doc-comment
  //
  // This is not a doc-comment
  fn add(x: u32, y: u32) -> u32 { /* */ }

In the example above, the function ``add(x: u32, y: u32) -> u32`` has a six line doc comment and directly after a two line comment which is not a doc-comment.

Those doc-comments are picked up by Rust tooling and used to generate documentation with the rustdoc tool. Every crate on `<http://crates.io/>`_, the standard Rust crate registry, automatically gets this documentation built for every release.

See `the heapless documentation <https://docs.rs/heapless/latest/heapless/>`_ as an example.

Read more about doc comments here: `<https://doc.rust-lang.org/rust-by-example/meta/doc.html>`_.

Doc-comments in the core library
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The core library makes heavy use of those doc-comments. Modules contain doc-comments that describe the functionality and structure in that module. Functions contain doc-comments that include a description of the behaviour, usage examples and safety comments where applicable. The Ferrocene compiler automatically enforces that all publicly exposed functions in the core library have a doc-comment through the ``missing_docs`` lint.

The doc-comments of both modules and functions are compiled, together with the function signatures, into the core library API docs.

Overall the doc-comments in the core library are very extensive, very high-quality and a lot of work has been and continues to be put into them.

That's why we want to rely on them for multiple purposes of the certification.

Requirements
------------

For requirements we will rely on the doc-comments plus the signature of the function. The signature describes the types of the input and output parameters, which are enforced by the compiler. The doc-comments describe the expected behaviour, which is tested by unit tests.

Doc-comments used as requirements must:

- Describe what the function does.
- Include one or more examples, which will be executed as doctests.
- Where applicable, include safety information.
- Where applicable, include panic information.

Architecture and software design
--------------------------------

The core library does not need a software architecture, due to its small size.

The core library uses the doc-comments of the modules as their module design.

Doc-comments used as module design must:

- Describe the purpose of the module.
- Describe the functionality included in it.

Quality of the doc-comments
---------------------------

The requirements for doc-comments used as requirements or module design are regularly checked. If gaps are found, the fixes will be upstreamed, which has the advantage of getting additional reviews by Rust experts and creating a consensus in the Rust community.

Tracing requirements to tests
-----------------------------

1. Firstly, the requirement of a function is the doc comment which is on top of that function. Therefore the requirements is traced to the function.
2. Secondly, the certification relies on code coverage to ensure that each function is sufficiently covered by tests.
3. Combining one and two, if all functions are covered by tests, also all requirements are covered by tests. Therefore tests do not need to be manually traced to requirements.

Requirement identifier
----------------------

Each function has one doc-comment aka. one requirement. The module path of a function is unique, which is ensured by the compiler, and can therefore be used as an identifier for that requirement. Doc comments might change between versions, so to ensure uniqueness across versions, that requirement id is the combination of the version of Ferrocene and the module path of the function.

Requirement status
------------------

A requirement is in one of three statuses: draft, approved, retired. If a requirement gets proposed via a pull request, it is in draft status. As soon as it is merged, the status is approved. If a pull request changes an existing requirement, the old requirement becomes retired. If a function gets marked as deprecated the requirement becomes retired as well.

Requirement verification
------------------------

All requirements must fulfill the basic properties of good requirements:

- Atomic
- Unambiguous
- Complete
- Accurate
- Free from vague terms like "some”, "several”, "many”, "sufficient”, "reasonable", "any” etc.
- Technically and logically feasible

Private functions
-----------------

Only public functions that are part of the certified subset must have an associated requirement. Functionality of a private function is usually included in the functionality described for the public function. Private function still must have full statement test coverage.

Assumed Requirements
--------------------

The ``core`` library API docs state:

  The Rust Core Library is the dependency-free[^1] foundation of The Rust Standard Library. It is the portable glue between the language and its libraries, defining the intrinsic and primitive building blocks of all Rust code. It links to no upstream libraries, no system libraries, and no libc.

  The core library is minimal: it isn't even aware of heap allocation, nor does it provide concurrency or I/O. These things require platform integration, and this library is platform-agnostic.

The Rust ``core`` library is developed as a general purpose library suitable to cater many different use cases. Therefore very few assumptions about the target platform and application are made.

All assumptions on how the core library is to be used are stated in the :doc:`user-manual:core/index` and must be strictly followed by users.
