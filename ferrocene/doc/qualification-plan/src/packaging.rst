.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Packaging Process
=================

Once the Development Process, the Documentation Process, and the Testing
Process for a PR are all completed, the Ferrocene CI infrastructure produces
full release artifacts for all supported platforms, and packages them up.


Package Organization
--------------------

The Packaging Process employs package definitions and subset definitions to
organize the various artifacts that are produced at the end of the
:ref:`testing:Testing Process`.

A package definition contains a set of attributes that uniquely identify
sources, binaries, documentation, libraries, build metrics, and metadata.
Each package belongs to a subset, effectively creating shallow hierarchies
of artifacts.


Pack Phase 1: Selection
-----------------------

This phase starts immediately after
:ref:`testing:Test Phase 2: Full Testing and Merge`.

During this phase, the Ferrocene CI infrastructure collects the following
artifacts in tarballs, tags them with the relevant subset, and archives them in
AWS S3 buckets:

- Rust compiler binaries, for each host platform.
- Rust libraries, for each compilation target.
- Additional unqualified developer tooling (Cargo), for each host platform.
- User-facing documentation.
- Qualification material.
- Test suite outcome information.


Pack Phase 2: Storage
---------------------

During this phase, the Ferrocene CI infrastructure copies the tarballs to
the ``s3://ferrocene-ci-artifacts`` storage location in Amazon S3. The
Ferrocene CI infrastructure uniquely identifies each tarball by the commit
hash of the PR that produced it. For example, if the commit number of the PR is
``abcdef``, then the artifact storage location is
``s3://ferrocene-ci-artifacts/ferrocene/dist/abcdef/``.
