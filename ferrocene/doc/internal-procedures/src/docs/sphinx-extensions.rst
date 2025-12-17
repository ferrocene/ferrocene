.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Custom Sphinx extensions
========================

To ease the effort required to keep the documentation up to date, and improve
the writing experience, we developed a few custom Sphinx extensions. This page
documents the features of each extension.

.. contents:: Table of contents:
   :local:
   :backlinks: none

``ferrocene_qualification`` extension
-------------------------------------

This extension implements "global" features that are not tied to any specific
document, and should be imported in each document.

The extension also enables ``sphinx.ext.intersphinx`` without the need to
manually enable it in ``conf.py``.


Defining and linking to IDs
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Our qualification documents contain IDs defining every concept and item
referenced across our qualification material for example - ``RUSTC_ERR_DRIVER_04``.
The ``ferrocene_qualification`` Sphinx extension provides a directive and a role
to respectively declare and link to an ID.

To declare a new ID, you can use the ``id`` directive:

.. code-block:: text

   .. id:: FOO

That directive will define an ID named ``FOO``, and it will render it as
"**Identifier:** ``FOO``" (inside of tables it will just render it as "``FOO``"
to reduce clutter).

To link to an ID, you can use the ``id`` role:

.. code-block:: text

   :id:`FOO`

This will create a link pointing to the definition you created earlier.
Note that linking to IDs works transparently across all of our qualification
material, without having to mention the document the ID was defined in. You can
just reference an ID defined in a different document like you would link to an
ID defined in the same document.

.. note::

   The ``id`` role and directive are both defined in the ``qualification``
   `Sphinx domain`_. To use them, you must either prefix their name with
   ``qualification:``, or put this at the top of the file:

   .. code-block:: rst

      .. default-domain:: qualification

Using substitutions
~~~~~~~~~~~~~~~~~~~

Across our documentation there are terms or phrases referenced multiple times.
Substitutions allow you to create "aliases" for common phrases you can include
in any qualification document.

Substitutions are defined in the ``ferrocene/doc/sphinx-substitutions.toml``
file. There are also the following substitutions, which are calculated
dynamically:

* ``doc_title``: the name of the document (e.g. Safety Manual)

* ``doc_short_title``: the acronym of the document (e.g. SM)

* ``ferrocene_version``: the Ferrocene version number (e.g. 23.06.0)

* ``rust_version``: the Rust version number (e.g. 1.68.2)

* ``channel``: the Ferrocene channel of this release (e.g. stable-23.06)

You can refer to substitutions by surrounding the substitution name with ``|``:

.. code-block:: text

   |doc_title|

Mentioning targets
~~~~~~~~~~~~~~~~~~

When you need to refer to targets across the documentation, it's better to use
a human-readable name (like ":target:`aarch64-unknown-none`") than the target
tuple, as the latter is often inconsistent between similar targets and could
be confusing to customers.

To keep the target names consistent, you can use the ``:target:`` role with the
target tuple as its content, which will be rendered as the human-readable
name:

.. code-block:: rst

   :target:`x86_64-unknown-linux-gnu`

The ``:target-with-tuple:`` role will also add the tuple following the
human-readable name, which is best used when customers then need to copy/paste
the tuple:

.. code-block:: rst

   :target-with-tuple:`aarch64-unknown-none`

The human-readable names are stored in ``ferrocene/doc/target-names.toml``, and
referring to a target not defined in that file will emit a warning.

Marking changes as upcoming
~~~~~~~~~~~~~~~~~~~~~~~~~~~

When we add documentation about an upcoming feature to our documentation, the
documentation will be immediately available on our `public documentation
<https://public-docs.ferrocene.dev>`_. In those cases, you should mark those
changes as "upcoming" by adding a note in the paragraph describing them:

.. code-block:: rst

   :upcoming:`25.02`

The upcoming block must contain the version number in which the feature will be
available. The badge will be shown in any branch not belonging to a specific
release (like nightly or rolling) and in all releases with a version lower than
the one you defined. It will automatically disappear from that version onwards.

.. note::

   You can include the ``:upcoming:`` role in page or section titles as well.
   Doing so will result in the badge being displayed along the title:

   .. code-block:: rst

      New exciting feature :upcoming:`25.02`
      ======================================

.. _document-id:

Document ID
~~~~~~~~~~~

The extension is responsible for generating the ID of the document, which is
displayed at the bottom of every page. The ID contains the acronym of the
document and a hash of the content, uniquely identifying the revision of the
document. This is enabled by default and requires no maintainer action.

Signature outcome page
~~~~~~~~~~~~~~~~~~~~~~

The extension is responsible for generating the dynamic page showing the
signature status of the document (see :doc:`signatures` for more information
about signatures). This is enabled by default and requires no maintainer
action.

``ferrocene_domain_cli`` extension
----------------------------------

The extension adds a custoom ``cli`` `Sphinx domain`_ that can be used to
document the CLI options of a program. Compared to Sphinx's builtin domain, our
``cli`` domain supports options with spaces in their name (like ``-C option``),
and emits metadata compatible with our traceability matrix tooling.

The extension only needs to be added to documents either defining or linking to
CLI options.

Defining CLI options
~~~~~~~~~~~~~~~~~~~~

CLI options can be defined with the ``cli:program`` directive, whose argument
is the name of the binary being documented.

Within it, you need to define one or more ``cli:option`` directives. The
argument of the directive is the name of the option, with the user-provided
value wrapped between ``<>``. The body of the directive is the documentation of
the CLI option:

.. code-block:: rst

   .. cli::program: rustc

      .. cli::option: --target <name>

         Used to specify the target tuple.

      .. cli::option: --help

         Show the help message.

Linking to CLI options
~~~~~~~~~~~~~~~~~~~~~~

It's possible to link to CLI options (even defined in other documents) with the
``:cli:option:`` role. Options are identified with both the program name and
the option name, separated by a space. It works like a normal link role, so you
can either put the option name as the role value, or arbitrary text (with the
option name between ``<>``).

.. code-block:: rst

   :cli:option:`rustc --help`
   :cli:option:`the target flag <rustc --target <name>>`

``ferrocene_toctrees`` extension
--------------------------------

This extension tweaks Sphinx's table of contents support to better suit our
style of documents. It must be enabled in all of our documents.

Continuous numbering of sections
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

By default Sphinx numbers pages within a ``toctree`` directive without
considering other directives. This means adding multiple ``toctree``\ s (for
example to group some pages together) would reset the numbering, potentially
having multiple pages numbered "1".

The extension patches Sphinx to fix this, and ensure that the page numbers
constantly increase even when multiple ``toctree`` directives are present.

Appendices support
~~~~~~~~~~~~~~~~~~

The extension adds a new ``appendices`` directive, with the same syntax and
functionality as the ``toctree`` directive. The only difference is that pages
are numbered with letters rather than digits.

``ferrocene_autoglossary`` extension
------------------------------------

This extension eases the maintenance of the glossary, and should be enabled for
each document containing a glossary. It serves two purposes:

* Prunes from ``glossary`` directives all items that are not mentioned anywhere
  in the current document. This allows sharing the same glossary file across
  documents without adding extra content to documents not referring to some
  terms.

* Automatically adds links to terms defined in ``glossary`` directives across
  the document, without the need to manually use the builtin ``:term:`` role.

``ferrocene_document_list`` extension
-------------------------------------

This extension is specific to the :doc:`document-list:index` document. It
provides the ``document-id`` role, which injects the
:ref:`document-id` of the requested document:

.. code-block:: rst

   The ID of the qualification plan is :document-id:`qualification-plan`.

``ferrocene_test_outcomes`` extension
-------------------------------------

This extension is specific to the :doc:`qualification-report:index`.

Rendering a template with the test outcomes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The ``render-outcomes-template`` directive allows rendering a `Jinja`_ template
with the :ref:`test outcomes <test-outcomes>` of a tested target. The
directive accepts a single argument, the path to the template to render. It
also accepts multiple options:

* ``host`` (required): the target tuple of the host platform
* ``target`` (required): the target tuple of the compilation target
* ``bare_metal_test_target`` (optional): the target tuple of the special
  target used for bare metal testing; it should be omitted if no special target
  was used
* ``certified_target`` (optional): the target tuple that has a certified standard library
* ``remote_testing`` (optional): whether the tests were executed on CI or on a
  remote machine/emulator; its presence without a value means ``true``, while
  its absence means ``false``

.. code-block:: rst

   .. render-outcomes-template:: templates/tests.jinja2
      :host: x86_64-unknown-linux-gnu
      :target: x86_64-unknown-linux-gnu

   .. render-outcomes-template:: templates/tests.jinja2
      :host: x86_64-unknown-linux-gnu
      :target: aarch64-unknown-none
      :bare_metal_test_target: aarch64-unknown-ferrocene.facade
      :remote_testing:

Rendering a summary of all test outcome pages
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The ``render-summary`` directive renders a table with all
``render-outcomes-template`` invocations present in the rest of the document.
It accepts no arguments nor options.

``ferrocene_relnotes`` extension
--------------------------------

This extension is specific to the :doc:`release-notes:index`.

Marking releases as upcoming
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The extension allows marking a page as upcoming by adding
``:upcoming-release:`` *before* any content of the page (except for comments).
This adds an "upcoming" badge next to the title, and a caution message just
below the title.

Including Rust changelogs
~~~~~~~~~~~~~~~~~~~~~~~~~

It is possible to inject the Rust changelog for a range of versions with the
``rust-changelog`` directive. The directive requires the ``:from:`` and
``:to:`` options to define the (inclusive) range of releases to include:

.. code-block:: rst

   .. rust-changelog::
      :from: 1.68.2
      :to: 1.76.0

.. _Sphinx domain: https://www.sphinx-doc.org/en/master/usage/domains/index.html
.. _Jinja: https://palletsprojects.com/p/jinja/
