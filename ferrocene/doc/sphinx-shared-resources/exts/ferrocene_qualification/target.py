# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from sphinx.util.docutils import SphinxRole
import sphinx
import tomli


class TargetRole(SphinxRole):
    def run(self):
        target = self.text.strip()
        return [
            render_target_name(
                self.env, self.config, target, location=self.get_location()
            )
        ], []


def render_target_name(env, config, target, *, location=None):
    if target in env.ferrocene_target_names:
        return nodes.Text(env.ferrocene_target_names[target])
    else:
        config = config["ferrocene_target_names_path"]
        logger = sphinx.util.logging.getLogger(__name__)
        logger.warning(f"missing target {target} in {config}", location=location)
        return nodes.problematic("", target)


def load_target_names(app, env, _docnames):
    with open(app.config["ferrocene_target_names_path"], "rb") as f:
        target_names = tomli.load(f)
    env.ferrocene_target_names = target_names


def setup(app):
    app.connect("env-before-read-docs", load_target_names)
    app.add_role("target", TargetRole())
