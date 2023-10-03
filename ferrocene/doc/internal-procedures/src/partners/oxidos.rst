.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

OxidOS
======

One of the partner projects of Ferrocene is `OxidOS <https://oxidos.io/>`_, an
embedded OS designed for automotive use. This page contains the information
relevant to Ferrous Systems employees working on Ferrocene.

Prebuilt OxidOS
---------------

One of the Ferrocene components we ship contains pre-built OxidOS library
files, so that customers can start using it in their projects without having to
manually compile it.

We're doing this because OxidOS relies on unstable features, and customers
cannot enable them on the compilers we ship. The Ferrocene build system can
enable them though, allowing us to pre-build it.

Inside of the component tarball there are two different copies of OxidOS: a
production copy and a development copy. The difference is the set of Cargo
features enabled as part of the build. The pre-built ``.rlib`` files are
installed in ``lib/rustlib/$target/lib/builtin/oxidos{,-dev}``, inside of the
sysroot.

Building OxidOS locally
~~~~~~~~~~~~~~~~~~~~~~~

OxidOS can be built with the ``ferrocene-oxidos`` dist step, which generates a
tarball that needs to be installed in the sysroot. Since the tarball contains
pre-built ``.rlib``\ s, you also need to build the standard library and the
compiler at the same time, and install everything together::

   ./x dist rustc rust-std
   ./x dist rust-std ferrocene-oxidos --target $target

Building OxidOS requires downloading the source code of the project. You can
let the build system do that by setting the ``ferrocene.oxidos-src`` key in
your ``config.toml``, pointing to a source code tarball. It can be a local
path, an HTTP URL, or an S3 URL.

Updating OxidOS
---------------

When we receive a new source code tarball from OxidOS, we need to convert it to
the format our build system expects, and upload it to S3. We need to make sure
that:

* The tarball is in the ``tar.xz`` format, as other compression formats (like
  ``zip``, ``tar.gz`` or ``tar.zst``) are not supported. If the tarball format
  is wrong you'll need to convert it.

* The tarball only contains one top-level directory, and no top-level files.
  The Cargo workspace must be located within the top-level directory. The name
  of the top-level directory doesn't matter.

To upload the new tarball, authenticate with AWS and run the following
command::

   aws --profile PROFILE s3 cp TARBALL s3://ferrocene-ci-mirrors/manual/oxidos/oxidos-source-YYYY-MM-DD.tar.xz

Make sure to replace ``PROFILE`` with the AWS profile name, ``TARBALL`` with
the local path to the tarball you prepared, and ``YYYY-MM-DD`` with the date
you received the source code from OxidOS on.

Once that's done, open a PR to the monorepo updating the
``ferrocene/ci/configure.sh`` file. You need to change the
``ferrocene.oxidos-src`` key to point to the ``s3://`` URL you just uploaded.
That will instruct CI to use the new sources.

Debugging build failures on CI
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Since OxidOS is built inside of the Ferrocene build system, there are a few
cases that could result in build failures not present upstream.

``check-cfg``
^^^^^^^^^^^^^

The Ferrocene build system enables ``check-cfg``, an `unstable compiler flag
<https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/check-cfg.html>`_
validating which ``cfg(..)``\ s can be used. This implies OxidOS must be able
to build with ``check-cfg`` enabled.

Since OxidOS does not support that unstable feature upstream, the Ferrocene
build system hardcodes the list of ``cfg(..)``\ s used by OxidOS. In
``src/bootstrap/ferrocene/partners/oxidos.rs`` there is a constant at the top
configuring ``check-cfg``. If you get errors, you should check whether the
contents of the constant need to be extended.

Allowed unstable features
^^^^^^^^^^^^^^^^^^^^^^^^^

OxidOS relies on some unstable features, which is why it's built as part of the
Ferrocene build system. We want to be careful about which features it enables,
to prevent it from depending on new unstable features as we upgrade it.

In ``src/bootstrap/ferrocene/partners/oxidos.rs`` there is a constant at the
top listing which unstable features are allowed. If the build fails due to a
new unstable feature being used, **vet that the feature can be safely used
by OxidOS** and add it to the list.
