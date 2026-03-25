.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Rust Project Contributors

.. default-domain:: spec

.. informational-page::

Developer Guide
===============

This document outlines the various formats, processes, and procedures associated with the maintenance of the FLS.

Changelog maintenance
---------------------

The Changelog is located in ``src/changelog.rst``. It should be updated using one of the sentence patterns outlined below. Note that this is not an exhaustive list, and special cases would need to use their own wording.

No change
~~~~~~~~~

When an entry in the Rust Release Notes does not affect the FLS, provide the reason as to why this is the case. Below are a few examples of such justifications::

    - Lints are outside the scope of the FLS
    - Target XYZ is outside the scope of the FLS
    - The exact mechanics of the borrow checker are outside the scope of the FLS
    - The restriction was not specified in the FLS
    - Configuration options are environment-specific and not exhaustive

Glossary entries
~~~~~~~~~~~~~~~~

When working with glossary entries, use the following sentence pattern for a single glossary entry::

    <Action> glossary entry: :t:`term`

Use the following sentence pattern for multiple paragraphs::

    <Action> glossary entries:
    - :t:`term`
    - :t:`term`

``<Action>`` must denote either ``Changed``, ``New``, or ``Removed``.

Paragraphs
~~~~~~~~~~

When working with paragraphs, use the following sentence pattern for a single paragraph::

    <Action> paragraph: :p:`fls_paragraph_id`

Use the following sentence pattern for multiple paragraphs::

    <Action> paragraphs:
    - :p:`fls_paragraph_id`
    - :p:`fls_paragraph_id`

``<Action>`` must denote either ``Changed``, ``Moved``, ``New``, or ``Removed``.

Sections
~~~~~~~~

When working with sections, use the following sentence pattern for a single section::

    <Action> section :ref:`fls_section_id`

Use the following sentence pattern for multiple sections::

    <Action> sections:
    - :ref:`fls_section_id`
    - :ref:`fls_section_id`

``<Action>`` must denote either ``Moved``, ``New``, or ``Removed``.

Syntax
~~~~~~

When working with syntax, use the following sentence pattern for a single syntax category::

    <Action> syntax: :s:`syntax_category`

Use the following sentence pattern for multiple syntax categories::

    <Action> syntax:
    - :s:`syntax_category`
    - :s:`syntax_category`

``<Action>`` must denote either ``Changed``, ``New``, or ``Removed``.
