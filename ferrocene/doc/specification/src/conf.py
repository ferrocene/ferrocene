# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# -- Path setup --------------------------------------------------------------

import os
import sys

sys.path.append(os.path.abspath("../exts"))
sys.path.append(os.path.abspath("../shared/exts"))


# -- Project information -----------------------------------------------------

project = "Ferrocene Language Specification"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"


# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    "ferrocene_intersphinx_support",
    "ferrocene_qualification",
    "ferrocene_spec",
    "ferrocene_spec_lints",
    "ferrocene_toctrees",
    "sphinx.ext.intersphinx",
]

# Add any paths that contain templates here, relative to this directory.
templates_path = []

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = []

ferrocene_id = "FLS"

# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#

html_theme = "ferrocene"
html_theme_path = ["../shared/themes"]

html_theme_options = {
    "license": "MIT or Apache 2.0",
}

html_title = "Ferrocene Language Specification"
html_short_title = "Language Specification"

# -- Options for linting -----------------------------------------------------

lint_alphabetical_section_titles = ["glossary"]

lint_no_paragraph_ids = ["index", "changelog"]
