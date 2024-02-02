# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from docutils.statemachine import StringList
from sphinx.directives import SphinxDirective
import docutils
import jinja2
import sphinx


class RenderOutcomesTemplate(SphinxDirective):
    required_arguments = 1
    option_spec = {
        "host": docutils.parsers.rst.directives.unchanged_required,
        "target": docutils.parsers.rst.directives.unchanged_required,
        "bare_metal_test_target": docutils.parsers.rst.directives.unchanged,
        "remote_testing": docutils.parsers.rst.directives.flag,
    }

    def run(self):
        return render_template(
            self,
            self.arguments[0],
            {
                "host": self.options["host"],
                "target": self.options["target"],
                "bare_metal_test_target": self.options.get("bare_metal_test_target"),
                "remote_testing": "remote_testing" in self.options,
                # Can be None if test outcomes were not injected.
                "platform_outcomes": self.env.ferrocene_test_outcomes.platform(
                    self.options["host"],
                    # Grab the outcomes for the bare metal test target if specified:
                    self.options["bare_metal_test_target"]
                    if "bare_metal_test_target" in self.options
                    else self.options["target"],
                )
                if self.env.ferrocene_test_outcomes is not None
                else None,
            },
        )


def render_template(directive, template, context):
    path = f"{directive.env.srcdir}/{template}"
    with open(path) as f:
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


def setup(app):
    app.add_directive("render-outcomes-template", RenderOutcomesTemplate)
