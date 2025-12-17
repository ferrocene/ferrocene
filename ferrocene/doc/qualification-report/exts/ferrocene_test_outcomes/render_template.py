# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from docutils.statemachine import StringList
from sphinx.directives import SphinxDirective
from sphinx.transforms import SphinxTransform
import docutils
import jinja2
import sphinx


class OutcomesPageNode(nodes.Element):
    def __init__(self, host, target, tested_target, upcoming):
        super().__init__()
        self["host"] = host
        self["target"] = target
        self["tested_target"] = tested_target
        self["upcoming"] = upcoming


class RenderOutcomesTemplate(SphinxDirective):
    required_arguments = 1
    option_spec = {
        "host": docutils.parsers.rst.directives.unchanged_required,
        "target": docutils.parsers.rst.directives.unchanged_required,
        "upcoming": docutils.parsers.rst.directives.unchanged,
        "bare_metal_test_target": docutils.parsers.rst.directives.unchanged,
        "certified_target": docutils.parsers.rst.directives.unchanged,
        "remote_testing": docutils.parsers.rst.directives.flag,
    }

    def run(self):
        # Grab the outcomes for the bare metal test target if specified:
        tested_target = self.options.get(
            "bare_metal_test_target",
            self.options["target"],
        )

        page_node = OutcomesPageNode(
            self.options["host"],
            self.options["target"],
            tested_target,
            self.options["upcoming"] if "upcoming" in self.options else None,
        )
        content_node = render_template(
            self,
            self.arguments[0],
            {
                "host": self.options["host"],
                "target": self.options["target"],
                "upcoming": self.options["upcoming"] if "upcoming" in self.options else None,
                "bare_metal_test_target": self.options.get("bare_metal_test_target"),
                "certified_target": self.options.get("certified_target"),
                "remote_testing": "remote_testing" in self.options,
                # Can be None if test outcomes were not injected.
                "platform_outcomes": self.env.ferrocene_test_outcomes.platform(
                    self.options["host"],
                    tested_target,
                )
                if self.env.ferrocene_test_outcomes is not None
                else None,
            },
        )
        return [page_node] + content_node


def render_template(directive, template, context):
    path = f"{directive.env.srcdir}/{template}"
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    directive.env.note_dependency(path)

    env = jinja2.Environment(autoescape=False)  # Autoescape is for HTML, not rst
    template = env.from_string(content)

    rendered = template.render(context)
    lines = StringList(rendered.splitlines(), source="")

    node = nodes.Element()
    node.document = directive.state.document
    sphinx.util.nested_parse_with_titles(directive.state, lines, node)

    return node.children


def null_fn(*args):
    pass


def setup(app):
    app.add_directive("render-outcomes-template", RenderOutcomesTemplate)
    app.add_node(OutcomesPageNode, html=(null_fn, null_fn))
