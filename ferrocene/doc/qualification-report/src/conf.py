# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import os
import sys

sys.path.append(os.path.abspath("../exts"))

project = "Ferrocene Qualification Report"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_toctrees",
    "ferrocene_qualification",
    "ferrocene_test_outcomes",

    "sphinx.ext.intersphinx",
    "ferrocene_intersphinx_support",
]

ferrocene_id = "QR"

html_theme = "ferrocene"
html_title = "Ferrocene Qualification Report"
html_short_title = "Qualification Report"
