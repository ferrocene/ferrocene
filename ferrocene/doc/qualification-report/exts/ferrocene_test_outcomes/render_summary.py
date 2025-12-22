# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from .render_template import OutcomesPageNode
from .utils import RenderTable, paragraph
from dataclasses import dataclass
from docutils import nodes
from ferrocene_qualification.target import render_target_name
from sphinx.directives import SphinxDirective
from sphinx.environment.collectors import EnvironmentCollector
from sphinx.transforms import SphinxTransform
from sphinx.util.nodes import make_refnode


class PendingSummaryNode(nodes.Element):
    def __init__(self, location):
        super().__init__()
        self.source = location[0]
        self.line = location[1]


class SummaryDirective(SphinxDirective):
    has_content = False

    def run(self):
        return [PendingSummaryNode(self.get_source_info())]


class InjectSummaryTransform(SphinxTransform):
    default_priority = 500

    def apply(self):
        for node in self.document.findall(PendingSummaryNode):
            self.replace_node(node)

    def replace_node(self, node):
        pages = get_storage(self.env).copy()
        pages.sort()

        table = RenderTable(7)
        table.add_row(
            paragraph("Host"),
            paragraph("Target"),
            paragraph("Passed tests"),
            paragraph("Failed tests"),
            paragraph("Ignored tests"),
            paragraph(""),
            head=True,
        )
        for page in pages:
            platform = (
                self.env.ferrocene_test_outcomes.platform(page.tested_target)
                if self.env.ferrocene_test_outcomes is not None
                else None
            )

            def render_sum(field):
                if platform is not None:
                    content = sum(getattr(inv, field) for inv in platform.invocations)
                    css_class = "align-right"
                else:
                    content = "-"
                    css_class = "align-center"
                container = paragraph(str(content))
                container["classes"].append(css_class)
                return container

            table.add_row(
                paragraph(self.render_target(node, page.host)),
                paragraph(self.render_target(node, page.target)),
                render_sum("passed_tests"),
                render_sum("failed_tests"),
                render_sum("ignored_tests"),
                paragraph(
                    make_refnode(
                        self.app.builder,
                        self.env.docname,
                        page.document,
                        "",
                        nodes.Text("Detailed results »"),
                    )
                ),
            )

        node.replace_self(table.finalize())

    def render_target(self, node, target):
        return render_target_name(
            self.env,
            self.config,
            target,
            location=(node.source, node.line),
        )


class OutcomePagesCollector(EnvironmentCollector):
    def clear_doc(self, app, env, docname):
        storage = get_storage(env)
        storage[:] = [page for page in storage if page.document != docname]

    def merge_other(self, app, env, docnames, other):
        storage = get_storage(env)
        other_storage = get_storage(other)
        for page in other_storage:
            if page.document in docnames:
                storage.append(page)

    def process_doc(self, app, document):
        storage = get_storage(app.env)
        for node in document.findall(OutcomesPageNode):
            storage.append(
                OutcomesPage(
                    document=app.env.docname,
                    host=node["host"],
                    target=node["target"],
                    tested_target=node["tested_target"],
                )
            )


def get_storage(env):
    key = "ferrocene_test_outcome_pages"
    if not hasattr(env, key):
        setattr(env, key, [])
    return getattr(env, key)


@dataclass(order=True)
class OutcomesPage:
    host: str
    target: str
    tested_target: str
    document: str


def setup(app):
    app.add_node(PendingSummaryNode)
    app.add_directive("render-summary", SummaryDirective)
    app.add_post_transform(InjectSummaryTransform)
    app.add_env_collector(OutcomePagesCollector)
