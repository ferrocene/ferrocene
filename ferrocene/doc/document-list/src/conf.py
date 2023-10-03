# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import os
import sys

sys.path.append(os.path.abspath("../exts"))

project = "Ferrocene Document List"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_document_list",
    "ferrocene_toctrees",
    "ferrocene_qualification",

    "sphinx.ext.intersphinx",
    "ferrocene_intersphinx_support",
]

html_theme = "ferrocene"
html_title = "Ferrocene Document List"
html_short_title = "Document List"

ferrocene_id = "DL"
