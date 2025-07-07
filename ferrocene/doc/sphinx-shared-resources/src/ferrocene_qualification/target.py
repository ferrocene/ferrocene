# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from sphinx.util.docutils import SphinxRole
import sphinx
import tomli


class TargetRole(SphinxRole):
    def __init__(self, *, include_tuple=False):
        super().__init__()
        self.include_tuple = include_tuple

    def run(self):
        target = self.text.strip()
        return [
            render_target_name(
                self.env,
                self.config,
                target,
                include_tuple=self.include_tuple,
                location=self.get_location(),
            )
        ], []


def render_target_name(env, config, target, *, include_tuple=False, location=None):
    if target in env.ferrocene_target_names:
        inline = nodes.inline()
        inline += nodes.Text(env.ferrocene_target_names[target])
        if include_tuple:
            inline += nodes.Text(" (")
            inline += nodes.literal("", target)
            inline += nodes.Text(")")
        return inline
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
    app.add_role("target-with-tuple", TargetRole(include_tuple=True))
