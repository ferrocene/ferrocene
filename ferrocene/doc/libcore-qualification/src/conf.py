# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

project = "Ferrocene Libcore Qualification "
copyright = "The Ferrocene Developers"
author = "The Ferrocene Developers"

extensions = [
    "ferrocene_autoglossary",
    "ferrocene_qualification",
    "ferrocene_toctrees",
    "sphinx.ext.autosectionlabel",
]

# autosectionlabel unique names settings
autosectionlabel_prefix_document = True

ferrocene_id = "LQ"

html_theme = "ferrocene"
html_title = "Ferrocene Libcore Qualification"
html_short_title = "Libcore Qualification"
html_static_path = ["_static"]

# Do not generate the index pages. We don't need them, and they cause
# linkchecker to fail due to them including *all* glossary entries, including
# entries that were removed by autoglossary.
html_use_index = False

suppress_warnings = ["autosectionlabel.*"]
