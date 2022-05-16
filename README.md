<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: Critical Section GmbH -->

# Ferrocene Language Specification

TODO: description of what FLS is

The Ferrocene Language Specification is licensed under either the `MIT` or
`Apache-2.0`, at your option. Licensing metadata is present in each file, and
the full license text is present in the `LICENSES/` directory.

## Building the specification

FLS uses [Sphinx](https://www.sphinx-doc.org) to build a rendered version of
the specification. To simplify building the rendered version, we created a
script called `make.py` that takes care of installing the expected Sphinx
release and invoking it with the right flags.

You can build the rendered version by running:

```
./make.py
```

By default Sphinx uses incremental rebuilds to only generate the content that
changed since the last invocation. If you notice a problem with incremental
rebuilds, you can pass the `-c` flag to clear the existing artifacts before
building:

```
./make.py -c
```

The rendered version will be available in `build/dirhtml/`.
