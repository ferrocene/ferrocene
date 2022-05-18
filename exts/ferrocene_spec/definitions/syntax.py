# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from docutils import nodes
import sphinx


ROLE = "syntax"
NAME = "syntaxes"
PRETTY_NAME = "syntactic category"


class Syntax:
    def __init__(self, id, document):
        self.id = id
        self.document = document

    def anchor(self):
        return f"syntax_{self.id}"

    def search_name(self, env):
        return self.id


def collect_items_in_document(app, nodes):
    for node in nodes:
        yield Syntax(node.def_id, app.env.docname)


def replace_id_node(app, node, syntax):
    new = nodes.literal("", syntax.id)
    new["ids"].append(syntax.anchor())
    node.replace_self(new)


def create_ref_node(env, syntax, make_link):
    return make_link(
        syntax.document,
        syntax.anchor(),
        nodes.literal("", syntax.id),
    )
