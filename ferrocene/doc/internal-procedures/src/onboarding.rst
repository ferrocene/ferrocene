.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Onboarding process
==================

Engineers joining the Ferrocene project must be fully aware of the technical
environment and procedures, to be able to work effectively. This page assumes
you are fully onboarded inside of Ferrous Systems, and that the Ferrous Systems
infrastructure team has provisioned you access.

Required reading
----------------

During your onboarding, you should read and understand the following documents:

* :doc:`qualification-plan:index`
* :doc:`internal-procedures:index`

The documents describe the development workflow and engineering procedures of
Ferrocene, and the requirements they outline must be followed by every team
member.

Required software and tools
---------------------------

You will need access to the following tools to be able to work on Ferrocene:

.. list-table::
   :header-rows: 1

   * - Tool
     - Authentication
     - Purpose
   * - `GitHub <https://github.com/ferrocene>`_
     - GitHub + SSO
     - Source code tracking, issue tracking, code review, CI/CD
   * - `AWS <https://ferrous-systems.awsapps.com/start>`_
     - SSO
     - Access to pre-built CI artifacts
   * - `Zulip <https://ferrous.zulipchat.com>`_
     - SSO
     - Realtime chat
   * - `ClickUp <https://ferroussystems.clickup.com/4637546/v/o/s/90040220935>`_
     - SSO
     - Project management and task tracking
   * - `CircleCI <https://app.circleci.com/pipelines/github/ferrocene>`_
     - GitHub
     - CI/CD
   * - `Customer portal <https://customers.ferrocene.dev>`_
     - Individual account, ask the :doc:`Technical Lead <qualification-plan:organization>`
     - Access to `rendered documentation <https://docs.ferrocene.dev>`_ and `releases <https://releases.ferrocene.dev>`_

Depending on your tasks, you might also need access to:

.. list-table::
   :header-rows: 1

   * - Tool
     - Authentication
     - Purpose
   * - `HackMD <https://ferroussystems.hackmd.io/team/ferrocene>`_
     - SSO
     - Documents and notes storage
   * - `Google Drive <https://drive.google.com/drive/folders/0ANsMR0hRKNhAUk9PVA>`_
     - SSO
     - Documents storage

You should also ask the project manager to give you access to the relevant
recurring meetings.

CircleCI access
^^^^^^^^^^^^^^^

To authenticate with CircleCI, navigate to
`Login with GitHub <https://circleci.com/authentication/login?f=gho>`_.

Once you have done so, you can access the Ferrocene project on CircleCI `here
<https://app.circleci.com/pipelines/github/ferrocene/ferrocene>`_.

If you cannot access the page above, make sure you have **write** access to the
Ferrocene repository, then go to the `CircleCI user settings
<https://app.circleci.com/settings/user>`_ and click the "Refresh permissions"
button.
