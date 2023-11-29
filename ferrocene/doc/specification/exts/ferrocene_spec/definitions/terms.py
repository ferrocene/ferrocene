# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

from docutils import nodes


ROLE = "term"
NAME = "term"
PRETTY_NAME = "term"


class Term:
    def __init__(self, id, document):
        self.id = id
        self.document = document

    def anchor(self):
        return f"term_{self.id}"

    def include_in_search(self):
        return True

    def display_name(self, env):
        return self.id


def collect_items_in_document(app, nodes):
    for node in nodes:
        yield Term(node["def_id"], app.env.docname)


def replace_id_node(app, node, term):
    new = nodes.emphasis("", node["def_text"])
    new["ids"].append(term.anchor())
    node.replace_self(new)


def create_ref_node(env, text, item):
    return nodes.Text(text)
