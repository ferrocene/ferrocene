.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Custom Sphinx extensions
========================

To ease the effort required to keep the documentation up to date, and improve
the writing experience, we developed a few custom Sphinx extensions. This page
documents the features of each extension.

``ferrocene_qualification`` extension
-------------------------------------

This extension implements "global" features that are not tied to any specific
document, and should be imported in each document.

Defining and linking to IDs
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Our qualification documents contain IDs defining every concept and item
referenced across our qualification material for example - ``ERR_DRIVER_04``.
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
   domain. To use them, you must either prefix their name with
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
triple, as the latter is often inconsistent between similar targets and could
be confusing to customers.

To keep the target names consistent, you can use the ``:target:`` role with the
target triple as its content, which will be rendered as the human-readable
name:

.. code-block:: rst

   :target:`x86_64-unknown-linux-gnu`

The ``:target-with-triple:`` role will also add the triple following the
human-readable name, which is best used when customers then need to copy/paste
the triple:

.. code-block:: rst

   :target-with-triple:`aarch64-unknown-none`

The human-readable names are stored in ``ferrocene/doc/target-names.toml``, and
referring to a target not defined in that file will emit a warning.

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
