# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

from docutils import nodes
from sphinx.directives import SphinxDirective
from sphinx.environment.collectors import EnvironmentCollector
from sphinx.transforms import SphinxTransform


class InformationalPageDirective(SphinxDirective):
    has_content = False

    def run(self):
        paragraph = nodes.paragraph()
        paragraph += nodes.Text("The contents of this page are informational.")

        note = nodes.note()
        note += paragraph

        return [note, InformationalMarkerNode()]


class InformationalMarkerNode(nodes.Element):
    pass


class InformationalPagesCollector(EnvironmentCollector):
    def clear_doc(self, app, env, docname):
        storage = get_storage(env)
        if docname in storage:
            storage.remove(docname)

    def merge_other(self, app, env, docnames, other):
        storage = get_storage(env)
        for element in get_storage(other):
            storage.add(element)

    def process_doc(self, app, document):
        storage = get_storage(app.env)
        for _ in document.findall(InformationalMarkerNode):
            storage.add(app.env.docname)
            break


class RemoveInformationalMarkerNodesTransform(SphinxTransform):
    default_priority = 500

    def apply(self):
        for node in self.document.findall(InformationalMarkerNode):
            node.parent.remove(node)


def get_storage(env):
    key = "spec_informational"
    if not hasattr(env, key):
        setattr(env, key, set())
    return getattr(env, key)


def setup(app):
    app.add_node(InformationalMarkerNode)
    app.add_env_collector(InformationalPagesCollector)
    app.add_post_transform(RemoveInformationalMarkerNodesTransform)
