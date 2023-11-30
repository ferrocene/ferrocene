# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import os
import sys

project = "Ferrocene Evaluation Plan"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_toctrees",
    "ferrocene_qualification",

    "sphinx.ext.intersphinx",
    "ferrocene_intersphinx_support",
]

html_theme = "ferrocene"
html_title = "Ferrocene Evaluation Plan"
html_short_title = "Evaluation Plan"

ferrocene_id = "EP"
