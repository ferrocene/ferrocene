# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

from collections import defaultdict
from dataclasses import dataclass
from docutils import nodes
from sphinx.directives import SphinxDirective
from sphinx.environment.collectors import EnvironmentCollector
from .informational import is_document_informational, is_section_informational
from sphinx.transforms import SphinxTransform
import sphinx


class ItemsWithRubricMarkerNode(nodes.Element):
    def __init__(self, rubric, location):
        super().__init__()
        self["rubric"] = rubric

        # Make the node's source location be the same as the directive, so that
        # later transforms can retrieve the location for error reporting.
        self.source = location[0]
        self.line = location[1]


class ItemsWithRubricDirective(SphinxDirective):
    has_content = True

    def run(self):
        if len(self.content) != 1:
            warn(
                "items-with-rubric accepts only the name of the rubric",
                self.get_location(),
            )
            return []

        return [ItemsWithRubricMarkerNode(self.content[0], self.get_source_info())]


class RubricCollector(EnvironmentCollector):
    def clear_doc(self, app, env, docname):
        storage = get_storage(env)
        for rubric, items in storage.items():
            # This makes a copy of the list (with `list(items)`) to be able to
            # remove items from it without affecting the iteration.
            for i, item in enumerate(list(items)):
                if item.document == docname:
                    items.pop(i)

    def merge_other(self, app, env, docnames, other):
        current = get_storage(env)
        other = get_storage(other)
        for rubric, items in other.items():
            for item in items:
                if item.document in docnames:
                    current[rubric].append(item)

    def process_doc(self, app, document):
        for rubric in document.findall(nodes.rubric):
            self.process_rubric(app, document, rubric)

    def process_rubric(self, app, document, rubric):
        rubric_name = rubric.astext()

        if not isinstance(rubric.parent, nodes.section):
            warn("rubric is not directly inside a section", rubric)
            return

        title_node = next(rubric.parent.findall(nodes.title))
        if title_node is None:
            warn(
                "section containing this rubric doesn't have a title",
                rubric,
            )
            return
        section_title = title_node.astext()

        section_id = rubric.parent["ids"][-1]
        get_storage(app.env)[rubric_name].append(
            StoredRubric(
                document=app.env.docname,
                section_id=section_id,
                section_title=section_title,
            )
        )


class InjectContentTransform(SphinxTransform):
    default_priority = 500

    def apply(self):
        for node in self.document.findall(ItemsWithRubricMarkerNode):
            self.replace_node(node)

    def replace_node(self, node):
        items = get_storage(self.env)[node["rubric"]]
        if not items:
            warn(f"no items with rubric {node['rubric']}", node)
            node.parent.remove(node)
            return

        # Ensure the items are in a sorted order to guarantee reproducibility.
        items.sort(key=lambda item: (item.section_title, item.section_id))

        bullet_list = nodes.bullet_list()
        for item in items:
            if is_document_informational(self.env, item.document):
                continue
            if is_section_informational(self.env, item.document, f"#{item.section_id}"):
                continue

            paragraph = nodes.paragraph()
            paragraph += sphinx.util.nodes.make_refnode(
                self.app.builder,
                self.env.docname,
                item.document,
                item.section_id,
                nodes.Text(item.section_title),
            )
            paragraph += nodes.Text(" (in ")
            paragraph += sphinx.util.nodes.make_refnode(
                self.app.builder,
                self.env.docname,
                item.document,
                "",
                nodes.Text(self.env.titles[item.document].astext()),
            )
            paragraph += nodes.Text(")")

            list_item = nodes.list_item()
            list_item += paragraph

            bullet_list += list_item

        node.replace_self(bullet_list)


def get_storage(env):
    key = "spec_rubrics"
    if not hasattr(env, key):
        setattr(env, key, defaultdict(list))
    return getattr(env, key)


@dataclass
class StoredRubric:
    document: str
    section_id: str
    section_title: str


def warn(message, location):
    logger = sphinx.util.logging.getLogger(__name__)
    logger.warning(message, location=location)


def setup(app):
    app.add_env_collector(RubricCollector)
    app.add_node(ItemsWithRubricMarkerNode)
    app.add_post_transform(InjectContentTransform)
