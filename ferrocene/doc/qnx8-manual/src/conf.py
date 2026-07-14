# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import os
import sys

sys.path.append(os.path.abspath("../../qualification-report/exts"))

project = "QNX8 Manual"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_autoglossary",
    "ferrocene_toctrees",
    "ferrocene_qualification",
    "ferrocene_test_outcomes",
    "ferrocene_domain_cli",
    "myst_parser",
    "sphinx.ext.autosectionlabel",
]

# autosectionlabel unique names settings
autosectionlabel_prefix_document = True

ferrocene_id = "QNXEM"

html_theme = "ferrocene"
html_title = "QNX8 Manual"
html_short_title = "QNX8 Manual"

# Do not generate the index pages. We don't need them, and they cause
# linkchecker to fail due to them including *all* glossary entries, including
# entries that were removed by autoglossary.
html_use_index = False

myst_heading_anchors = 7

suppress_warnings = ["myst.header", "autosectionlabel.*"]
