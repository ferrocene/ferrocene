# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene Evaluation Plan"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_toctrees",
    "ferrocene_qualification",
    "ferrocene_autoglossary",
]

html_theme = "ferrocene"
html_title = "Ferrocene Evaluation Plan"
html_short_title = "Evaluation Plan"

ferrocene_id = "EP"

# Do not generate the index pages. We don't need them, and they cause
# linkchecker to fail due to them including *all* glossary entries, including
# entries that were removed by autoglossary.
html_use_index = False
