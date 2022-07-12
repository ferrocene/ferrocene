# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

# -- Path setup --------------------------------------------------------------

import os
import sys

sys.path.insert(0, os.path.abspath("../exts"))


# -- Project information -----------------------------------------------------

project = "Ferrocene Language Specification"
copyright = "Critical Section GmbH"
author = "Critical Section GmbH"


# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    "ferrocene_spec",
]

# Add any paths that contain templates here, relative to this directory.
templates_path = []

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = []

rst_prolog = """
.. caution::

   You're reading a pre-release draft of the Ferrocene Language Specification.
   Some parts of this document might be missing, incomplete or incorrect.
"""


# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#

html_theme = "ferrocene"
html_theme_path = ["../themes"]

html_theme_options = {
    "license": "MIT or Apache 2.0",
}

html_title = "Ferrocene Language Specification"
html_short_title = "Language Specification"
