.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Customer Interactions
=====================

Interactions between Customers and the Ferrocene organization are done
through the following tools: 

.. list-table::
   :align: left
   :stub-columns: 1

   * - email
     - | - Get contract information;
       | - Contact our teams for support, questions or enhancement requests;
       | - Obtain release tarballs

.. end of table

In the following parts, the usage of the customer focused tools are described.

Opening Issues
--------------

Opening an issue is done through the email system. Customers will open issues by
sending an email to support@ferrocene.dev.

For further interactions related to the same issue, customers and Ferrocene
team members will reply to the email. This ensures the traceability of the
conversation.

Obtaining Ferrocene
-------------------

The Ferrocene toolchain is made available to customers through email
request. A tarball containing Ferrocene and the associated artifacts is
provided alongside the installation instructions. For details on the
installation process, refer to the :doc:`User Manual - Installation Process
<user-manual:install>`. Eventually, a dedicated Ferrocene tool will replace
the email system for downloading the Ferrocene toolchain.

The following channels are available to customers:

* **nightly:** Latest changes from the main branch which is published on a daily
  basis. There are no stability guarantees, no long term support, no warranty,
  and no qualification.

* **rolling:** Latest release made by upstream, with a new release roughly every
  six weeks. There are no stability guarantees, no long term support, no
  warranty, and no qualification.

* |channel_name| : Major release of Ferrocene.

.. note:: 
   For this qualification, only the |channel_name| release has been qualified 
   for the :doc:`specified environment <safety-manual:environment>`. 
   Consequently, only |channel_name| should be used in qualified development
   context.

Obtaining Documentation
-----------------------

Ferrocene comes with a comprehensive set of documentation. This
documentation is downloaded through the same tarball as the Ferrocene
compiler. This ensures that each release is accompanied by the corresponding
documentation.

The documentation contains the following information:

* Ferrocene documentation: the necessary documentation to understand
  Ferrocene, its scope, and safety-related material.

* Dev Log: a list of every new feature implemented in our products.

* Library AP documentation: description of associated libraries.

* Rust Project's documentation: documentation from the upstream rust project
  which provides the basis for understanding and using the Rust language.

Unless specified otherwise, customers have the freedom to copy any of the
documents, both electronically and in hard copy.

Consulting Known Problems and New Features
------------------------------------------

Whenever a noticeable defect is corrected or new functionality is implemented
in Ferrocene products, an impact entry is recorded in the
:doc:`known-problems`. Each impact entry is uniquely identified by a GitHub
Issue.

The up-to-date databases are regularly made accessible, via email or through
`problems.ferrocene.dev <https://problems.ferrocene.dev>`_, to supported users
and inform them of the current state of their product.
