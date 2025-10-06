.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Digital signatures
==================

All the qualification documents we send to the assessor have to be digitally
signed by the responsible parties, to attest they were reviewed and to prevent
accidental changes to the documents (which would require the assessor to review
the documents again).

.. caution::

   Signing documents will store your work email address in the `Rekor
   transparency log <https://docs.sigstore.dev/logging/overview/>`_. Do not
   attempt to sign documents if you don't want your work email address to be
   published in it.


.. _signing-all-documents:

Signing all documents
---------------------

.. note::

   While everyone can invoke the command to begin signing, after a signature is
   made, the tooling will error out if you are not listed as an authorized
   signer.

.. caution::

   If this is the first time you contribute to Ferrocene from this machine,
   remember to :doc:`setup your local Ferrocene environment
   <../setup-local-env>` before signing.

We developed a small wrapper around the "cosign" tool to make it easy to
sign our qualification documents. Cosign is fully managed by the Ferrocene build
system: you don't need to install it manually.

To sign all of the Ferrocene documents, :ref:`authenticate with AWS <aws-auth>`
(if you haven't done so today) and run:

.. code-block:: text

   ./x sign

Running the command will build all the documents locally, download the correct
version of cosign, and start the signing process for each document.

The signing process will open the signature page in the browser (or, if the
build system cannot open your browser, show you the link to the page). In the
signature page, you will need to select **Microsoft** as your identity provider,
and authenticate with your work credentials. The tool will then perform the
signature, and save the signature files to disk.

The signing process will be automatically repeated for each document you need to
sign. Each signature will repeat the whole signing process. Once ``./x sign``
finishes you can commit and push the changes.

.. tip::

   The ``./x sign`` command will only sign documents that don't have a valid
   signature. If you make some changes to the documents and then run ``./x
   sign`` again, it will only sign the changed ones.

Signing a single document
-------------------------

It's possible to provide one or more paths to ``./x sign`` to sign just those
documents. For example, if you just want to sign the internal procedures, you
can run:

.. code-block::

   ./x sign ferrocene/doc/internal-procedures

Signing a single document is only useful if your role requires signing a single
document.

Verifying signatures
--------------------

You can verify that all the present signatures are still valid with this
command:

.. code-block:: text

   ./x test ferrocene-check-document-signatures

This will emit warnings if some documents are not signed.

.. Note::
   When running the command locally, you might get signature verification errors
   if some of the cached pages built locally are out of date. To fix them,
   remove the ``build/`` directory.

Debugging signatures differences locally compared to CI
-------------------------------------------------------

The signature process relies on the documentation produced locally and the
documentation produced in CI to be bit-by-bit identical. If there is any
difference, the signature will not match.

Debugging why signatures pass locally but fail on CI require looking at the
differences between the two. To aid generating such a diff, we created the
``ferrocene/tools/document-signatures/diff.py`` script. The script is supposed
to be run by the person who signed the document.

Before running the script, make sure you have a valid `GitHub Personal Access
Token (classic) <https://github.com/settings/tokens>`_ authorized with SSO to
access the Ferrocene organization, and place it in the ``GITHUB_TOKEN``
environment variable by running `$ export GITHUB_TOKEN=your_token`. Then run
this command, replacing ``PR_NUMBER`` with the number of the PR that failed CI:

.. code-block::

   AWS_PROFILE=ferrocene-ci ferrocene/tools/document-signatures/diff.py PR_NUMBER

The command will fetch the documentation from CI, build a fresh copy of the
documentation locally, and then compare the two. At the end, the script will
show the path to the diff: copy that file and send it to the engineer in charge
of debugging the difference.

Configuring the roles allowed to sign
-------------------------------------

The signature tool requires that people authorized to sign documents are listed
in the signature config file for each document, located at
``$doc/signature/config.toml`` (where ``$doc`` is the path to the document, for
example ``ferrocene/doc/internal-procedures``).

Each role is represented by its own TOML table:

.. code-block:: toml

   [roles.ROLE_ID]
   role-name = "ROLE_NAME"
   name = "PERSON_NAME"
   email = "PERSON_EMAIL"

The ``ROLE_NAME`` and ``PERSON_NAME`` placeholders are freeform text fields,
whose only purpose is to be displayed in the rendered HTML. The ``ROLE_ID``
placeholder can also be set to any value, and is used internally to identify the
role. The ``PERSON_EMAIL`` placeholder is the load-bearing one, and **must** be
the company email address of the person authorized to sign.

The signature tool will try to match the email of the person who signed the
document with a role in that document's configuration file, and error out if the
email is not listed in the configuration file.

Inspecting the signature contents
---------------------------------

.. note::

   This is *not* part of the signature process. This is only relevant when
   debugging.

It is possible to inspect the ephemeral code signing certificate of a signature
with this command:

.. code-block:: text

   jq .cert $path/signature/$role.cosign-bundle -r | base64 -d | openssl x509 -text
