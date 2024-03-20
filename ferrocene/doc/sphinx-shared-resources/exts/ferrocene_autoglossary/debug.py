# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This module appends to a file all lexed nodes that have at least one match
# within them, if the AUTOGLOSSARY_DEBUG_FILE environment variable is set when
# building the docs (and contains the path to that file). It is helpful when
# making changes to the lexer.

from . import lexer
from .lexer import MatchedTerm
import os


def write_lexer_outcome(app, env):
    debug_file = os.environ.get("AUTOGLOSSARY_DEBUG_FILE")
    if debug_file is None or app.builder.name == "ferrocene-intersphinx":
        return

    # The problem this tries to prevent is that the build system changes the
    # current directory before invoking Sphinx, which will result in the file
    # not being placed where the user expects with a relative path.
    #
    # This would be fixable by having the build system resolve the path before
    # providing it to the extension, but it doesn't make sense to add that
    # complexity for a rarely-used debug feature.
    if not os.path.isabs(debug_file):
        raise RuntimeError("AUTOGLOSSARY_DEBUG_FILE must be an absolute path")
    output = open(debug_file, "a")

    from . import State  # Imported here to avoid circular dependencies

    terms = State.get(env).terms

    for docname in sorted(env.found_docs):
        doctree = env.get_doctree(docname)
        for node in lexer.find_lexable_nodes(doctree):
            rendered = render_lexed_node(terms, node.node)
            if rendered is None:
                continue
            rendered = rendered.replace("\n", " ")
            output.write(f"{node_location(node.node)}: {rendered}\n")


def render_lexed_node(terms, node):
    result = ""
    has_matches = False

    for token in lexer.lexer(node.astext(), terms):
        if type(token) is MatchedTerm:
            result += f"[{token.text}]"
            has_matches = True
        elif type(token) is str:
            result += token
        else:
            raise RuntimeError("invalid token type")

    if has_matches:
        return result


def node_location(node):
    if (
        getattr(node, "source", None) is not None
        and getattr(node, "line", None) is not None
    ):
        return f"{node.source}:{node.line}"
    elif node.parent is not None:
        return node_location(node.parent)
    else:
        return "<unknown>"


def setup(app):
    app.connect("env-check-consistency", write_lexer_outcome)
