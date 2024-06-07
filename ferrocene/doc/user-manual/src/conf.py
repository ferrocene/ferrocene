# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene User Manual"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_intersphinx_support",
    "ferrocene_qualification",
    "ferrocene_toctrees",
    "ferrocene_domain_cli",
    "sphinx.ext.intersphinx",
    "sphinx.ext.autosectionlabel",
]

# autosectionlabel unique names settings
autosectionlabel_prefix_document = True

ferrocene_id = "UM"

html_theme = "ferrocene"
html_title = "Ferrocene User Manual"
html_short_title = "User Manual"
