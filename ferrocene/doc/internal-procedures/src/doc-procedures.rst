.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Documentation procedures
========================

The following procedures describe the means of building, updating, validating,
and signing documents as per the appropriate level of quality.

Document process
----------------

The document process has been described in the :doc:`qualification-plan:index`.
The procedures describe how documentation is imported to GitHub as well as the
verification process used to verify the buildability and correctness of the
content.

See the following chapters for the detailed process:

* :doc:`qualification-plan:documentation`

Working on the documentation
----------------------------

Qualification and QMS documents are built with Sphinx, and their source code is
located in the ``ferrocene/doc`` folder inside the Ferrocene repository. For
example, the source of the Safety Manual is located in
``ferrocene/doc/safety-manual``.

The Ferrocene build system takes care of building and cross-linking the
qualification and QMS documents, including downloading all the required
dependencies. Building the documents outside of the Ferrocene build system
(including building them with the standard Sphinx CLI) is not supported and will
likely not work.

Building documents
~~~~~~~~~~~~~~~~~~

You can build a qualification document by passing its path to the Ferrocene
build system:

.. code-block:: text

   ./x doc ferrocene/doc/$name

You can also build multiple documents at the same time by passing all of their
paths:

.. code-block:: text

   ./x doc ferrocene/doc/$foo ferrocene/doc/$bar ferrocene/doc/$baz

If you want to build all the documentation generated with Sphinx (for example
if you need to ensure cross-document links work, or you need to test an
extension change across all documents) you can run:

.. code-block:: text

   ./x doc ferrocene/doc

If you want to build all of the available documentation, including the standard
library docs and documentation from upstream, you can omit the path:

.. code-block:: text

   ./x doc

The build system will output the path containing the resulting documents. You
can also open the built documents in your browser by passing the ``--open``
flag:

.. code-block:: text

   ./x doc ferrocene/doc/$foo --open

If you are making frequent changes to the contents of a document, the
``--serve`` flag will continuously watch for changes in the file system and
rebuild content as needed. It also starts a local web server with automatic
reloading of the changes in the browser:

.. code-block:: text

   ./x doc ferrocene/doc/$foo --serve

.. caution::

   The ``--serve`` flag is only available for documents built with Sphinx, and
   it will not do anything for other kinds of documentation (like mdBook or
   rustdoc). It also only supports serving a *single* Sphinx document: if you
   attempt to serve more than one at the time it will error out.

When you are working on Sphinx extensions, the ``--debug-sphinx`` flag will
change the configuration to aid debugging, by running only one builder job at
the time and showing exception tracebacks:

.. code-block:: text

   ./x doc ferrocene/doc/$foo --debug-sphinx

.. note::

   Sphinx features automatic dependency tracking and should rebuild only the
   files you actually changed. Dependency tracking doesn't work when the
   implementation of extensions is changed: in those cases you will want to
   pass the ``--fresh`` flag, which does a fresh build ignoring caches:

   .. code-block:: text

      ./x doc ferrocene/doc/$foo --fresh

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

You can refer to substitutions across all of our documentation  by surrounding
the substitution name with ``|``:

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

Signing documents
-----------------

All the qualification documents we send to TUV have to be digitally signed by
the responsible parties, to attest they were reviewed and to prevent accidental
changes to the documents (which would require TUV to review the documents
again).

The Ferrocene QMS documentation also needs to be digitally signed by
responsible parties to attest that their contents are up to date and represent
the latest applicable procedures to be applied throughout the organization.

We developed a small wrapper around the "cosign" tool to make it easy to
sign our qualification documents.

To sign a document, you need to run this command inside of
``ferrocene/ferrocene``:

.. code-block:: text

   ./x sign $path

For example:

.. code-block:: text

   ./x sign ferrocene/doc/evaluation-report

Running the command will:

* Build the document locally to calculate its document ID and the hash of the
  generated files.

* Download the expected version of cosign, if it was not downloaded before.

* Invoke cosign to digitally sign the document metadata gathered earlier.

When cosign is invoked, you will need to:

* Agree to your company email address being published in the Rektor transparency
  log.

* In the browser window that opened, select "Microsoft".

* Authenticate with your company email credentials.

This will generate an ephemeral code signing certificate for your company email
address and sign the contents of the document with it. Once that's done, commit
the new files generated by the signing tool.

Verifying signatures
--------------------

You can verify that all the present signatures are still valid with this
command:

.. code-block:: text

   ./x test ferrocene-check-document-signatures

.. Note::
   When running the command locally, you might get signature verification errors
   if some of the cached pages built locally are out of date. To fix them,
   remove the ``build/`` directory.

It is also possible to inspect the ephemeral code signing certificate of a
signature with this command:

.. code-block:: text

   jq .cert $path/signature/$role.cosign-bundle -r | base64 -d | openssl x509 -text

Test outcomes
-------------

Some of our documents, like the :doc:`qualification-report:tests/index` page,
need to know which tests were executed and ignored to generate parts of the
content. We call this information "test outcomes". While not strictly required
to build the docs, not providing them will result in some information being
omitted, and warnings being rendered in the generated content mentioning the
lack of test outcomes.

Test outcomes consist of a collection of JSON files produced by the build
system's "build metrics" feature. They are usually generated by CI, but it is
possible to also generate them locally by setting the ``build.metrics = true``
option in ``config.toml``.

The easiest way to inject test outcomes into the built documentation is to
instruct the build system to automatically download the latest available copy
of the test outcomes built by CI. People with access to the Ferrocene AWS account can do so
by adding this to their ``config.toml``:

.. code-block:: toml

   ferrocene.test-outcomes = "download-ci"

Another way is to manually download a copy of the test outcomes from `the
releases download portal <https://releases.ferrocene.dev>`_ (look for a tarball
named ``ferrocene-test-outcomes`` in the release you care about), extract the
tarball in a directory on disk, and add this snippet to ``config.toml``:

.. code-block:: toml

   ferrocene.test-outcomes = "custom"
   ferrocene.test-outcomes-dir = "path/to/extracted/tarball/share/ferrocene/test-outcomes"

.. note::

   When configuring a custom path for the test outcomes, make sure you choose
   the path actually containing the JSON files. In downloaded tarballs, that is
   the ``share/ferrocene/test-outcomes`` directory inside the tarball.
