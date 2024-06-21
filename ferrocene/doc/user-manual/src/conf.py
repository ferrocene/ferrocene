# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene User Manual"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    # ferrocene
    "ferrocene_intersphinx_support",
    "ferrocene_qualification",
    "ferrocene_toctrees",
    "ferrocene_domain_cli",
    "ferrocene_autoglossary",
    # 3rd-party
    "myst_parser",
    # sphinx.ext
    "sphinx.ext.intersphinx",
    "sphinx.ext.autosectionlabel",
]

# autosectionlabel unique names settings
autosectionlabel_prefix_document = True

ferrocene_id = "UM"

html_theme = "ferrocene"
html_title = "Ferrocene User Manual"
html_short_title = "User Manual"

# Do not generate the index pages. We don't need them, and they cause
# linkchecker to fail due to them including *all* glossary entries, including
# entries that were removed by autoglossary.
html_use_index = False

myst_heading_anchors = 7

suppress_warnings = ["myst.header", "autosectionlabel.*"]
