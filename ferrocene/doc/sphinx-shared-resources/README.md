<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# Sphinx shared resources

This directory contains Sphinx extensions that are used across multiple documents, including our
documentation theme. Keep in mind that making a change here will propagate it across all documents,
so be mindful of side effects. There is no need to bump version numbers when making changes meant to
be applied in the monorepo.

## External repositories relying on the shared resources

Some external repositories, like `ferrocene/specification` or `ferrocene/problems`, rely on the
shared resources for their documentation purposes. To enable them to access the resources without
cloning the whole repository, the contents of this directory [are published on PyPI][pypi].

Publishing on new versions is done automatically by CI *only* when the version number in
`pyproject.toml` is updated. Bump the version number when you want to make the new changes available
to external repositories.

[pypi]: https://pypi.org/project/ferrocene-sphinx-shared-resources/
