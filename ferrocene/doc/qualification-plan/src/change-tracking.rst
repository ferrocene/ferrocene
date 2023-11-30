.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Change Tracking
===============

The general compliance document can only justify the identification and
traceability of the Ferrocene toolchain up to the point of the delivery to
the customer; it is the responsibility of the customer to complete it with
project-specific evidence for the post-delivery period.

All Ferrocene-related artifacts, including the code base, test suites,
documentation, qualification material, and releases are hosted on GitHub and
Amazon S3. As a result, every change to any of these artifacts is tracked, and
every commit yields a complete and validated snapshot of Ferrocene.

The source code of the Ferrocene toolchain and the source code of our validation
infrastructure are placed under configuration management using ``git``. Each
release of the Ferrocene toolchain is uniquely identified by a symbolic name. In
order to manage the complexity of the development process, different branches
are used (see :ref:`development:Merge Strategy` for full details). It is
therefore possible to know precisely the contents of each of the releases, and
to have parallel development on new targets or with specialized requirements.

The Ferrocene toolchain maintains several artifacts that track the changes
between **stable releases**. New features that have been introduced in a stable
release are documented in features files. New KPs that have been identified and
existing KPs that have been fixed in stable releases are documented in KP
files. Once a new stable release is produced, the Ferrocene team sends a mass
email to existing customers, notifying them of the release, its new features
and KPs. In addition, customers and interested parties may view these artifacts
online on the Ferrocene landing page.

As per the “Customer Centric Objectives” of the “Ferrocene Quality Objectives”
document, the entire Ferrocene toolchain release process is designed to ensure
that the delivered product is exactly what was validated internally. The
Ferrocene toolchain is made available to customers as a tarball supplied by
Ferrocene engineers.
