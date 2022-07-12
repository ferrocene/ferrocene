.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

====================
FLS Sphinx Extension
====================

To enhance the Ferrocene Language Specification, and to facilitate its
authoring, updating, and testing it easier, we developed and use a custom
Sphinx extension that adds new roles and directives. The source code of the
extension is in the ``exts/ferrocene_spec`` directory.

.. contents:: In this document:

Definitions
===========

To aid users reading the FLS, we make extensive use of internal links. Since
Sphinx's native support for creating and linking to definitions did not suit
our needs we implemented custom roles for defining and linking to definitions.

Definition namespaces
---------------------

The extension includes support for multiple definition namespaces. In addition
to allowing the same name to be used as a definition in multiple namespaces,
each namespace has a different styling when rendered.

.. list-table::
   :header-rows: 1

   * - Namespace
     - Link role
     - Definition role
     - Styling

   * - Terms
     - ``:t:`foo```
     - ``:dt:`foo```
     - Normal text

   * - Programmatic constructs
     - ``:c:`foo```
     - ``:dc:`foo```
     - Monospace text

   * - Syntactic categories
     - ``:s:`Foo```
     - ``:ds:`Foo```
     - Monospace text

Both link roles and definition roles are standard reStructuredText roles, and
can be used inline inside a sentence. For example:

.. code-block:: rst

    An :dt:`associated item` is an :t:`item` that appears within an :dt:`implementation` or a :dt:`trait`.

Definitions are case insensitive: you can define and link to them with
whichever case suits the surrounding text the most.

Linking to definitions
----------------------

The standard syntax for link roles is to just include the name of the
definition inside the role, which will generate a link to the place the
definition is defined in. In addition to that, more advanced syntaxes are
available to handle more complex use cases.

Prefixes and suffixes
~~~~~~~~~~~~~~~~~~~~~

Sometimes you might need to add prefixes or suffixes to a word you're linking,
for example adding an ``s`` at the end to pluralize it. The native RST syntax
for that is quite clunky, as you need to put an "escaped space" between the
role and the rest of the word:

.. code-block:: rst

   Multiple :t:`value`\ s can be used here.

The snippet above will render as you'd expect ("Multiple values can be used
here" with a link on "value"), but that syntax is hard to read and annoying to
write. To ease adding prefixes and suffixes to links, the extension allows you
to include the prefix and suffix inside the role itself, by wrapping the actual
definition inside square brackets:

.. code-block:: rst

   Multiple :t:`[value]s` can be used here.

Arbitrary labels
~~~~~~~~~~~~~~~~

If you need to link to a definition but the link label doesn't fully contain
the definition name (for example if you're using a sentence as the link label,
or you need to pluralize a word with irregular plurals) you can put the
arbitrary label inside the role and put the definition name at the end of the
role within angle brackets:

.. code-block:: rst

   The :t:`criteria <criterion>` used for the change are:

Links to the Rust standard library
==================================

You can link to the documentation of items defined in the Rust standard library
(``core``, ``alloc``, ``std``, ``test`` and ``proc_macro``) by using the
``:std:`type``` role (even for types defined in other standard library crates)
with the fully qualified item path:

.. code-block:: rst

   The type needs to implement :std:`core::marker::Copy`.

Syntax blocks
=============

To ease the process of defining blocks of syntax definitions, the extension
implements the custom ``syntax`` directive, which parses the syntax contained
within it and automatically inserts definitions and links to the referenced
syntactic categories:

.. code-block:: rst

   .. syntax::

      ExpressionStatement ::=
          ExpressionWithBlock $$;$$?
        | ExpressionWithoutBlock $$;$$

In the directive above, the extension will automatically insert a syntactic
category definition for ``ExpressionStatement`` (since it's followed by ``::=``), and it
will insert syntactic category links for both ``ExpressionWithBlock`` and
``ExpressionWithoutBlock``.

Words and characters wrapped within ``$$`` are considered "literals": they will
be rendered differently than syntactic categories, and they won't be considered
by the extension when looking for syntactic categories.

Paragraph IDs
=============

Ferrocene's test suite needs each paragraph in the FLS to have a unique ID
attached to it. To ensure that this happens, the extension provides a way to
easily define the ID for each paragraph, and to use that ID to link to the
paragraph from other parts of the FLS.

Paragraph IDs can be added to a paragraph with the ``:dp:`id``` role. The role must
contain an unique random ID prefixed with ``fls_``, and the role must appear at
the start of a paragraph:

.. code-block:: rst

   :dp:`fls_qTgd9xuAY3n3`
   This is a paragraph with an ID.

You can generate a list of random IDs by running the following command::

   ./generate-random-ids.py

You can also link to an existing paragraph with the ``:p:`id``` role:

.. code-block:: rst

   See :p:`fls_qTgd9xuAY3n3` for a sample paragraph using IDs.

Note that paragraph IDs are also used to generate the human-readable paragraph
numbers generated by Sphinx: while IDs are supposed to be stable across FLS
revisions, the human-readable paragraph numbers can change between renderings.
