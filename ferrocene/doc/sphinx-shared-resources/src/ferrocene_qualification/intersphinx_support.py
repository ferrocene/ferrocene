# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This module defines the "ferrocene_intersphinx_mappings" configuration, which is deserialized
# from JSON and then added to the intersphinx_mapping configuration. This is needed because the
# format of intersphinx_mapping is too complex to be provided with the -D flag, and our build system
# needs to provide it.
#
# It also enables intersphinx by default.

from sphinx.builders import Builder
from sphinx.builders.html import StandaloneHTMLBuilder
import json
import sphinx
import sphinx.ext.intersphinx


def inject_intersphinx_mappings(app, config):
    if config.ferrocene_intersphinx_mappings is not None:
        for inventory in json.loads(config.ferrocene_intersphinx_mappings):
            config.intersphinx_mapping[inventory["name"]] = (
                inventory["html_root"],
                inventory["inventory"],
            )


def setup(app):
    # Automatically enable the sphinx.ext.intersphinx extension without
    # requiring users to configure it in their conf.py.
    app.setup_extension("sphinx.ext.intersphinx")

    app.add_config_value("ferrocene_intersphinx_mappings", None, "env", [str])
    app.connect("config-inited", inject_intersphinx_mappings, priority=1)
