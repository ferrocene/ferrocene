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

The rendered version will be available in `build/html/`.

You can also start a local server on port 8000 with automatic rebuild and
reload whenever you change a file by passing the `-s` flag:

```
./make.py -s
```

## Checking links consistency

It's possible to run Rust's linkchecker tool on the rendered documentation, to
see if there are broken links. To do so, pass the `--check-links` flag:

```
./make.py --check-links
```

This will clone the source code of the tool, build it, and execute it on the
rendered documentation.

## Updating build dependencies

FLS uses `pip-tools` to manage the Python dependencies used for builds, as it
allows pinning hashes for the dependencies. While it doesn't add any additional
burden when installing dependencies (the format it outputs is understood by
`pip`), you have to install it when regenerating the `requirements.txt` file.

To install `pip-tools`, we recommend first installing [pipx], and then running:

```
pipx install pip-tools
```

Once that's done, you can change the list of desired dependencies in the
`requirements.in` file, and run this command to regenerate the
`requirements.txt` file:

```
pip-compile --generate-hashes
```

[pipx]: https://pypa.github.io/pipx/
