# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import os
import sys

project = "Ferrocene Safety Manual"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_toctrees",
    "ferrocene_qualification",
    "ferrocene_domain_cli",

    "sphinx.ext.intersphinx",
    "ferrocene_intersphinx_support",
    "sphinx.ext.autosectionlabel",
]

# autosectionlabel unique names settings
autosectionlabel_prefix_document = True

ferrocene_id = "SM"

html_theme = "ferrocene"
html_title = "Ferrocene Safety Manual"
html_short_title = "Safety Manual"
