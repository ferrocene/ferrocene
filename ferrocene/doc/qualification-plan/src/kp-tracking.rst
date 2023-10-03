.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Known Problems Tracking
=======================

Newly discovered KPs and previously fixed KPs in stable releases are documented
in dedicated KP files, which are then collated into a KP Manual. The KP Manual
is made available to customers either in stand-alone fashion, or as an addition
to the Safety Manual. Customers are expected to consult the KP Manual in order
to verify the applicability of the KPs for their versions of the toolchain.

Each KP file has the following format:

* ``title``: A short description of the KP.

* ``references``: A list of links to GitHub issues and other ticket management
  systems that track the KP.

* ``tag``: A designator of the KP severity. The tags are as follows:

  * ``wrong-compiler-code``: The KP causes the Ferrocene compiler to generate
    wrong code.

  * ``wrong-library-code``: The KP causes a Ferrocene library to generate
    wrong code.

  * ``wrong-linking``: The KP causes the linker to either incorrectly link a
    symbol or fail to link altogether.

  * ``accepts-invalid``: The KP causes the Ferrocene compiler to accept
    invalid code, and possibly generate wrong code.

  * ``diagnostic``: The KP causes the Ferrocene compiler to suppress a
    safety-related error diagnostic, or emit a spurious safety-related error
    diagnostic.

* ``introduced-commit``: The commit number when the KP was introduced.

* ``fixed-commit``: A set of commit numbers belonging to various releases that
  contain the fix for the KP.

* ``Description``: A detailed description of the KP, possibly accompanied by
  code examples.

* ``Workaround``: A mitigation strategy for avoiding the KP.

* ``Detection``: A textual description of methods for detecting the KP.

The following screenshot shows the contents of a KP.

.. figure:: figures/kp-example.png

   Known Problems Example

A KP may be identified through several sources:

* An upstream GitHub issue with label l-unsound is opened.

* A customer reports a bug.

* A Ferrocene developer reports a bug.

* Some other source, e.g. an article, mailing list, or newsgroup of interest
  observes a defect.

Regardless of its source, a KP receives a dedicated GitHub issue. Once a fix for
the KP has been found, the fix goes through the :ref:`development:Development
Process`. Any reproducers are retained as regression tests and are integrated
into the Ferrocene test suite. Finally, a KP file is created for the KP, and the
Safety Manual is updated.
