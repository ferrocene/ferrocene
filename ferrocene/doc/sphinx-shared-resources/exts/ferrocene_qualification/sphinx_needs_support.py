# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import sphinx_needs
import json


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

    if config.ferrocene_external_needs is not None:
        config.needs_external_needs = json.loads(config.ferrocene_external_needs)

    config.needs_title_optional = True
    config.needs_build_json = True
    config.needs_reproducible_json = True


def setup(app):
    sphinx_needs.setup(app)

    app.add_config_value("ferrocene_external_needs", None, "env", str)
    app.connect("config-inited", configure_sphinx_needs, priority=100)
