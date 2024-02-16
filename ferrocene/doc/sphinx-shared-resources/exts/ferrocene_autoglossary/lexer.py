# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Note: check out debug.py within the extension for useful tools to check the
# lexer behavior when making changes to this file.

from dataclasses import dataclass
from docutils import nodes
from sphinx import addnodes
from typing import Optional


def find_lexable_nodes(node, *, inside_glossary=False, inside_definition_of=None):
    if type(node) == nodes.Text:
        yield LexableNode(node=node, inside_definition_of=inside_definition_of)
    elif type(node) == addnodes.glossary:
        inside_glossary = True
    elif inside_glossary and type(node) == nodes.definition_list_item:
        inside_definition_of = {term.astext() for term in node.findall(nodes.term)}
    elif type(node) in (
        nodes.reference,
        nodes.literal,
        nodes.literal_block,
        nodes.comment,
    ):
        return

    for child in node.children:
        if inside_glossary and type(child) == nodes.term:
            continue
        for result in find_lexable_nodes(
            child,
            inside_glossary=inside_glossary,
            inside_definition_of=inside_definition_of,
        ):
            yield result


@dataclass
class LexableNode:
    node: object
    inside_definition_of: Optional[set[str]]


def lexer(text, terms):
    normalized_text = normalize(text)
    while text:
        # We need to look for all possible matches of all possible terms,
        # otherwise we might replace a term at the end of the sentence
        # ignoring the terms in the middle if that term happens to be
        # earlier in the list of terms.
        nearest = None
        for term in terms:
            if term.abbreviation:
                pos = text.find(term.name)
            else:
                pos = normalized_text.find(normalize(term.name))
            if pos == -1:
                continue

            if (
                nearest is None
                # Always prefer closer matches, as farther matches will be
                # handled in the next iterations.
                or nearest[0] > pos
                # Prefer longer matches if the position is the same, for
                # example to prefer "LLVM IR" to "LLVM".
                or (nearest[0] == pos and len(nearest[1].name) < len(term.name))
            ):
                nearest = (pos, term)

        if nearest is not None:
            start, term = nearest
            end = start + len(term.name)

            yield text[:start]
            yield MatchedTerm(term=term, text=text[start:end])

            text = text[end:]
            normalized_text = normalized_text[end:]
        else:
            # No terms left in the text sentence.
            yield text
            break


@dataclass
class Term:
    name: str
    document: str
    anchor: str
    abbreviation: bool


@dataclass
class MatchedTerm:
    term: Term
    text: str


def normalize(name):
    # This can be expanded to apply more normalizations before checking for
    # matches. The only constraint is that this function CANNOT add or remove
    # chars to the input, it MUST be the same length.
    return name.lower()
