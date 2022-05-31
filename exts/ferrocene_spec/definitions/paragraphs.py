# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from collections import defaultdict
from docutils import nodes
import hashlib


ROLE = "p"
NAME = "paragraph"
PRETTY_NAME = "paragraph"


class Paragraph:
    def __init__(self, id, document, section, plaintext, sequential):
        self.id = id
        self.document = document
        self.section = section
        self.plaintext = plaintext
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
        # Include the hash of the paragraph contents in `objects.inv`. This
        # will be used by the tooling in ferrocene/ferrocene to generate
        # lockfiles, requiring tests to be updated whenever the corresponding
        # paragraph's contents change.
        sha256 = hashlib.sha256()
        sha256.update(self.plaintext.encode("utf-8"))

        return f"{self.number(env)} {sha256.hexdigest()}"


def collect_items_in_document(app, nodes):
    ids = defaultdict(lambda: 1)
    for node in nodes:
        section = find_section(node)
        yield Paragraph(
            id=node["def_id"],
            document=app.env.docname,
            section=section,
            plaintext=plaintext_paragraph(node),
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


def plaintext_paragraph(node):
    paragraph = find_parent_of_type(node, nodes.paragraph)
    if paragraph is None:
        paragraph = node
    return paragraph.astext().replace("\n", " ")


def find_section(node):
    section = find_parent_of_type(node, nodes.section)
    if section is None:
        raise RuntimeError(f"could not find section for {node!r}")
    if section.parent is not None and isinstance(section.parent, nodes.document):
        return ""
    else:
        return "#" + section["ids"][0]


def find_parent_of_type(node, ty):
    cursor = node
    while cursor is not None:
        if isinstance(cursor, ty):
            return cursor
        cursor = cursor.parent
    return None
