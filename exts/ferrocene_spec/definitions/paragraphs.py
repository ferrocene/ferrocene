# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from collections import defaultdict
from docutils import nodes


ROLE = "p"
NAME = "paragraph"
PRETTY_NAME = "paragraph"


class Paragraph:
    def __init__(self, id, document, section, sequential):
        self.id = id
        self.document = document
        self.section = section
        self.sequential = sequential

    def number(self, env):
        section = ".".join(
            str(n) for n in env.toc_secnumbers[self.document][self.section]
        )
        return f"{section}:{self.sequential}"

    def anchor(self):
        return self.id

    def include_in_search(self):
        return False

    def display_name(self, env):
        return self.number(env)


def collect_items_in_document(app, nodes):
    ids = defaultdict(lambda: 1)
    for node in nodes:
        section = find_section(node)
        yield Paragraph(
            id=node["def_id"],
            document=app.env.docname,
            section=section,
            sequential=ids[section],
        )
        ids[section] += 1


def replace_id_node(app, node, paragraph):
    new = nodes.inline()
    new["ids"].append(paragraph.id)
    new["classes"].append("spec-paragraph-id")
    new += nodes.Text(paragraph.number(app.env))
    node.replace_self(new)


def create_ref_node(env, text, item):
    if item is not None:
        return nodes.emphasis("", item.number(env))
    else:
        return nodes.emphasis("", "Paragraph " + text)


def find_section(node):
    cursor = node.parent
    while cursor is not None:
        if not isinstance(cursor, nodes.section):
            cursor = cursor.parent
            continue

        if cursor.parent is not None and isinstance(cursor.parent, nodes.document):
            return ""
        else:
            return "#" + cursor["ids"][0]

    raise RuntimeError(f"could not find section for {node!r}")
