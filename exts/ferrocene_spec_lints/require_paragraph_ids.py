# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

from docutils import nodes
from ferrocene_spec.definitions import DefIdNode


def check(app, raise_error):
    for document in app.env.found_docs:
        doctree = app.env.get_doctree(document)
        if document in app.config.lint_no_paragraph_ids:
            check_does_not_have_ids(doctree, raise_error)
        else:
            check_has_ids(doctree, raise_error)


def check_has_ids(node, raise_error):
    is_paragraph = type(node) == nodes.paragraph

    if is_paragraph and type(node.parent) == nodes.section:
        should_have_id(node, "paragraph", raise_error)
    elif is_paragraph and type(node.parent) == nodes.list_item:
        should_have_id(node, "list item", raise_error)
    elif is_paragraph and type(node.parent) == nodes.entry:
        if node.parent.parent.index(node.parent) == 0:
            should_have_id(node, "first cell of a table row", raise_error)
        else:
            should_not_have_id(
                node,
                "second or later cell of a table row",
                raise_error,
            )
    elif type(node) == nodes.section:
        if not any(name.startswith("fls_") for name in node["names"]):
            raise_error("section should have an id", location=node)
    else:
        should_not_have_id(node, type(node).__name__, raise_error)

    for child in node.children:
        check_has_ids(child, raise_error)


def check_does_not_have_ids(node, raise_error):
    if type(node) == nodes.section:
        if any(name.startswith("fls_") for name in node["names"]):
            raise_error("section should not have an id", location=node)
    else:
        should_not_have_id(node, type(node).__name__, raise_error)

    for child in node.children:
        check_does_not_have_ids(child, raise_error)


def should_have_id(node, what, raise_error):
    if any(is_definition(child) for child in node.children[1:]):
        raise_error(f"id in {what} is not the first element", location=node)
    elif not len(node.children) or not is_definition(node.children[0]):
        raise_error(f"{what} should have an id", location=node)


def should_not_have_id(node, what, raise_error):
    if any(is_definition(child) for child in node.children):
        raise_error(f"{what} should not have an id", location=node)


def is_definition(node):
    return (
        type(node) == DefIdNode
        and node["def_kind"] == "paragraph"
        and node["def_id"].startswith("fls_")
    )
