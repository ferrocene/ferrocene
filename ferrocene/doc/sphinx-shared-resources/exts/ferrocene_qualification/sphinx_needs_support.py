# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import sphinx_needs


def configure_sphinx_needs(app, config):
    config.needs_types = [
        {
            "directive": "hazop-use",
            "title": "Use Case",
            "prefix": "USE_",
            "color": "",
            "style": "",
        },
        {
            "directive": "hazop-error",
            "title": "Potential error",
            "prefix": "ERR_",
            "color": "",
            "style": "",
        },
        {
            "directive": "hazop-mitigation",
            "title": "Mitigation",
            "prefix": "MIT_",
            "color": "",
            "style": "",
        },
        {
            "directive": "constraint",
            "title": "Constraint",
            "prefix": "CONSTR_",
            "color": "",
            "style": "",
        },
    ]

    config.needs_extra_links = [
        {
            "option": "caused_by",
            "incoming": "causes",
            "outgoing": "caused by",
        },
        {
            "option": "mitigates",
            "incoming": "mitigated by",
            "outgoing": "mitigates",
        },
    ]

    config.needs_title_optional = True
    config.needs_build_json = True


def setup(app):
    sphinx_needs.setup(app)

    app.connect("config-inited", configure_sphinx_needs, priority=100)
