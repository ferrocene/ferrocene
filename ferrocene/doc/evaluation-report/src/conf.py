# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene Evaluation Report"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_toctrees",
    "ferrocene_qualification",
    "ferrocene_autoglossary",

    "sphinx.ext.intersphinx",
    "ferrocene_intersphinx_support",
]

html_theme = "ferrocene"
html_title = "Ferrocene Evaluation Report"
html_short_title = "Evaluation Report"

ferrocene_id = "ER"
