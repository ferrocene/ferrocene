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
        path = f"{self.env.srcdir}/{self.arguments[0]}"
        with open(path) as f:
            content = f.read()
        self.env.note_dependency(path)

        env = jinja2.Environment(autoescape=False)  # Autoescape is for HTML, not rst
        template = env.from_string(content)

        rendered = template.render(
            host=self.options["host"],
            target=self.options["target"],
            bare_metal_test_target=self.options.get("bare_metal_test_target"),
            remote_testing="remote_testing" in self.options,
        )
        rendered = StringList(rendered.splitlines(), source="")

        node = nodes.Element()
        node.document = self.state.document
        sphinx.util.nested_parse_with_titles(self.state, rendered, node)

        return node.children


def setup(app):
    app.add_directive("render-outcomes-template", RenderOutcomesTemplate)
