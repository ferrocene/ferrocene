# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from sphinx.util.docutils import SphinxRole
import sphinx
import tomli


class TargetRole(SphinxRole):
    def run(self):
        target = self.text.strip()
        if target in self.env.ferrocene_target_names:
            return [nodes.Text(self.env.ferrocene_target_names[target])], []
        else:
            config = self.config["ferrocene_target_names_path"]
            logger = sphinx.util.logging.getLogger(__name__)
            logger.warning(
                f"missing target {target} in {config}", location=self.get_location()
            )
            return [nodes.problematic("", self.text)], []


def load_target_names(app, env, _docnames):
    with open(app.config["ferrocene_target_names_path"], "rb") as f:
        target_names = tomli.load(f)
    env.ferrocene_target_names = target_names


def setup(app):
    app.connect("env-before-read-docs", load_target_names)
    app.add_role("target", TargetRole())
