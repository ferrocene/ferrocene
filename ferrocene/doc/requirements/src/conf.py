# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene Requirements"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_autoglossary",
    "ferrocene_qualification",
    "ferrocene_toctrees",
]

ferrocene_id = "FR"

html_theme = "ferrocene"
html_title = project
html_short_title = project

# Do not generate the index pages. We don't need them, and they cause
# linkchecker to fail due to them including *all* glossary entries, including
# entries that were removed by autoglossary.
html_use_index = False
