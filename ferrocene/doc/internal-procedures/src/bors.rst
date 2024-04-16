.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Interacting with bors
=====================

Bors is the merge queue bot we use in the Ferrocene monorepo (and smaller
adjacent repositories), as described :ref:`in the qualification plan
<qualification-plan:bors>`. This page explains how developers can interact with
it.

Pull request comments
---------------------

The primary way to interact with bors is by including "bors commands" in your
pull request comments, on their own line. The following commands should be
used:

.. list-table::
   :header-rows: 1

   * - Command
     - Description

   * - ``bors merge``
     - Approve the current PR and put it into the queue.

   * - ``bors cancel``
     - Remove the current PR from the queue and cancel the merge. Does **not** affect try builds.

   * - ``bors try``
     - Start a try build of the CI pipeline, without merging.

   * - ``bors try-``
     - Cancel the try build.

   * - ``bors retry``
     - Execute the previous command again.

   * - ``bors p=NUMBER``
     - Set the priority of the PR to ``NUMBER`` (must be an integer).

   * - ``bors single on``
     - Merge this PR without grouping it with other PRs.

   * - ``bors single off``
     - Merge this PR by grouping it with other PRs.

   * - ``bors ping``
     - Check whether bors is active.

.. caution::

   The behavior of ``bors retry`` differs from the behavior of upstream's bors
   implementation (called "homu"). As explained in the table above, the retry
   command re-executes the previous command, it doesn't attempt to merge the PR
   again.

   In practice, if you first type ``bors merge`` and then ``bors retry`` bors
   will attempt to merge the PR again, but just because it ends up executing
   ``bors merge`` twice. If instead you run ``bors merge``, ``bors p=10``
   and then ``bors retry``, the priority would be set twice.

Inspecting the queue
--------------------

The bors queue can be inspected at `bors-ferrocene.herokuapp.com/repositories
<https://bors-ferrocene.herokuapp.com/repositories>`_ (after selecting the
repository you want to see). Note that the queue is only available to Ferrous
Systems employees.
