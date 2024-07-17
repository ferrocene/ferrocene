# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene Safety Manual"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_toctrees",
    "ferrocene_qualification",
    "ferrocene_domain_cli",
    "ferrocene_autoglossary",
    "sphinx.ext.autosectionlabel",
]

# autosectionlabel unique names settings
autosectionlabel_prefix_document = True

ferrocene_id = "SM"

html_theme = "ferrocene"
html_title = "Ferrocene Safety Manual"
html_short_title = "Safety Manual"

# Do not generate the index pages. We don't need them, and they cause
# linkchecker to fail due to them including *all* glossary entries, including
# entries that were removed by autoglossary.
html_use_index = False
