# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes


def span_with_class(text, css_class):
    node = nodes.inline("", text)
    node["classes"].append(css_class)
    return node


def parse_docinfo(document):
    result = {}
    for node in document.findall(nodes.docinfo):
        for field in node.children:
            if type(field) != nodes.field:
                continue
            if type(field.children[0]) != nodes.field_name:
                raise RuntimeError("first child should be a field name")
            if type(field.children[1]) != nodes.field_body:
                raise RuntimeError("second child should be a field body")

            # Fields without a body are represented with an empty <field_body>
            # element, but in the array we return it as None.
            body = field.children[1].astext()
            result[field.children[0].astext()] = body if body else None

    return result
