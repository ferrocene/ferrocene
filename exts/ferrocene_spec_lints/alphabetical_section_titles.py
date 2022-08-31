# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

import re
from docutils import nodes


SPLIT_NUMBERS = re.compile(r"([0-9]+)")


def check(app, raise_error):
    for docname in app.config.lint_alphabetical_section_titles:
        recursive_check(app.env.get_doctree(docname), raise_error)


def recursive_check(node, raise_error):
    prev_title = None

    for child in node.children:
        recursive_check(child, raise_error)

        if type(child) == nodes.section:
            title_node = next(child.findall(nodes.title))
            title = title_node.astext()
            if prev_title is not None and not compare_titles(prev_title, title):
                raise_error(
                    f"unordered sections: '{title}' should be before '{prev_title}'",
                    location=title_node,
                )
            prev_title = title


def compare_titles(prev, next):
    def key(k):
        return [(int(c) if c.isdigit() else c.lower()) for c in SPLIT_NUMBERS.split(k)]

    return key(prev) < key(next)
