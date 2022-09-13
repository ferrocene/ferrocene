# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

from . import utils
from collections import defaultdict
from docutils import nodes
from sphinx import addnodes
from sphinx.directives import SphinxDirective
from sphinx.environment.collectors import EnvironmentCollector
from sphinx.transforms import SphinxTransform
import sphinx


# Marker to use inside the informational storage to signal the whole page, and
# not just a section, is informational
WHOLE_PAGE = "{{{whole-page}}}"


def build_directive(kind):
    class InformationalDirective(SphinxDirective):
        has_content = False

        def run(self):
            paragraph = nodes.paragraph()
            paragraph += nodes.Text(f"The contents of this {kind} are informational.")

            note = nodes.note()
            note += paragraph

            return [note, InformationalMarkerNode(kind)]

    return InformationalDirective


class InformationalMarkerNode(nodes.Element):
    def __init__(self, kind):
        super().__init__()
        self["kind"] = kind


class InformationalPagesCollector(EnvironmentCollector):
    def clear_doc(self, app, env, docname):
        storage = get_storage(env)
        if docname in storage:
            del storage[docname]

    def merge_other(self, app, env, docnames, other):
        storage = get_storage(env)
        for document, contents in get_storage(other).items():
            storage[document] = storage[document].union(contents)

    def process_doc(self, app, document):
        storage = get_storage(app.env)
        for node in document.findall(InformationalMarkerNode):
            if node["kind"] == "page":
                self.process_page(app, storage, node)
            elif node["kind"] == "section":
                self.process_section(app, storage, node)
            else:
                raise RuntimeError("unknown directive kind: " + node["kind"])

    def process_page(self, app, storage, node):
        if type(node.parent) != addnodes.document:
            warn("informational-page must be at the top of the document", node)
            return

        storage[app.env.docname].add(WHOLE_PAGE)

    def process_section(self, app, storage, node):
        if type(node.parent) != nodes.section:
            warn("informational-section must be inside a section", node)
            return

        try:
            _id, anchor = utils.section_id_and_anchor(node.parent)
        except utils.NoSectionIdError:
            warn(
                "informational-section must be inside a section with an ID "
                "starting with fls_"
            )
            return
        storage[app.env.docname].add(anchor)


class RemoveInformationalMarkerNodesTransform(SphinxTransform):
    default_priority = 500

    def apply(self):
        for node in self.document.findall(InformationalMarkerNode):
            node.parent.remove(node)


def get_storage(env):
    key = "spec_informational"
    if not hasattr(env, key):
        setattr(env, key, defaultdict(set))
    return getattr(env, key)


def warn(message, location):
    logger = sphinx.util.logging.getLogger(__name__)
    logger.warning(message, location=location)


def setup(app):
    app.add_node(InformationalMarkerNode)
    app.add_env_collector(InformationalPagesCollector)
    app.add_post_transform(RemoveInformationalMarkerNodesTransform)
