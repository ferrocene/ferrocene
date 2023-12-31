# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene Qualification Plan"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_toctrees",
    "ferrocene_qualification",

    "sphinx.ext.intersphinx",
    "ferrocene_intersphinx_support",
    "sphinx.ext.autosectionlabel",
]

# autosectionlabel unique names settings
autosectionlabel_prefix_document = True

ferrocene_id = "QP"

html_theme = "ferrocene"
html_title = "Ferrocene Qualification Plan"
html_short_title = "Qualification Plan"
