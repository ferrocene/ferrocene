# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

from docutils import nodes


ROLE = "syntax"
NAME = "syntaxes"
PRETTY_NAME = "syntactic category"


class Syntax:
    def __init__(self, id, document):
        self.id = id
        self.document = document

    def anchor(self):
        return f"syntax_{self.id}"

    def include_in_search(self):
        return True

    def display_name(self, env):
        return self.id


def collect_items_in_document(app, nodes):
    for node in nodes:
        yield Syntax(node["def_id"], app.env.docname)


def replace_id_node(app, node, syntax):
    new = nodes.literal("", node["def_text"])
    new["ids"].append(syntax.anchor())
    node.replace_self(new)


def create_ref_node(env, text, item):
    return nodes.literal("", text)
