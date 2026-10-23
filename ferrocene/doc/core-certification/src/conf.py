# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene Library Certifications"
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_autoglossary",
    "ferrocene_qualification",
    "ferrocene_toctrees",
    "sphinx.ext.autosectionlabel",
    "sphinxcontrib.mermaid",
]

# autosectionlabel unique names settings
autosectionlabel_prefix_document = True

ferrocene_id = "FLC"

html_theme = "ferrocene"
html_title = "Ferrocene Library Certifications"
html_short_title = "Library Certifications"

# Do not generate the index pages. We don't need them, and they cause
# linkchecker to fail due to them including *all* glossary entries, including
# entries that were removed by autoglossary.
html_use_index = False

mermaid_version = "12.2.0"

suppress_warnings = [
    "autosectionlabel.*",
    # suppress "Inline emphasis start-string without end-string."
    "docutils.*",
]
