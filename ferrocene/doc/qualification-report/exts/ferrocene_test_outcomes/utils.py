# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes

class RenderTable:
    def __init__(self, columns, *, title=None):
        # Apparently the order in which elements are created matters :/
        # https://github.com/sphinx-doc/sphinx/issues/6691#issuecomment-570537138
        self.root = nodes.table()

        if title is not None:
            title_node = nodes.title()
            title_node += nodes.Text(title)
            self.root += title_node

        self.tgroup = nodes.tgroup(cols=columns)
        self.root += self.tgroup

        for _ in range(columns):
            colspec = nodes.colspec()
            colspec["colwidth"] = "3"
            self.tgroup += colspec

        self.body = nodes.tbody()
        self.head = nodes.thead()

    def add_row(self, *cells, head=False):
        row = nodes.row()
        for cell in cells:
            entry = nodes.entry()
            entry += cell
            row += entry
        if head:
            self.head += row
        else:
            self.body += row

    def finalize(self):
        if len(self.head.children):
            self.tgroup += self.head
        if len(self.body.children):
            self.tgroup += self.body
        return self.root


def paragraph(*content):
    node = nodes.paragraph()
    for piece in content:
        if type(piece) == str:
            node += nodes.Text(piece)
        else:
            node += piece
    return node


def literal(text):
    node = nodes.literal()
    node += nodes.Text(text)
    return node


def error(*message):
    node = nodes.error()
    node += paragraph(*message)
    return [node]
