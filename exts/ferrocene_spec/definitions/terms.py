# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from dataclasses import dataclass
from docutils import nodes
import sphinx


ROLE = "term"
NAME = "term"
PRETTY_NAME = "term"


@dataclass
class Term:
    id: str
    document: str

    def anchor(self):
        return f"term_{self.id.lower().replace(' ', '_')}"

    def search_name(self, env):
        return self.id


def collect_items_in_document(app, nodes):
    for node in nodes:
        yield Term(node.def_id, app.env.docname)


def replace_id_node(app, node, term):
    new = nodes.emphasis("", term.id)
    new["ids"].append(term.anchor())
    node.replace_self(new)


def create_ref_node(env, term, make_link):
    return make_link(
        term.document,
        term.anchor(),
        nodes.Text(term.id),
    )
