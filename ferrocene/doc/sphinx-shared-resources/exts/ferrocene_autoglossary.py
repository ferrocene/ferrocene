# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This extension automatically inserts links across the whole document pointing
# to the relevant terms in the glossary, without having to manually annotate
# words with :term:`Glossary Entry`.
#
# The extension requires no configuration, and just adding a glossary directive
# is enough for the extension to work.

from dataclasses import dataclass
from docutils import nodes
from sphinx import addnodes
from sphinx.environment.collectors import EnvironmentCollector
from sphinx.transforms import SphinxTransform
import sphinx
import string


class GlossaryCollector(EnvironmentCollector):
    def clear_doc(self, app, env, docname):
        state = State.get(env)
        state.terms = [item for item in state.terms if item.document != docname]

    def merge_other(self, app, env, docnames, other):
        state = State.get(env)
        other_state = State.get(other)
        for item in other_state.terms:
            if item.document in docnames:
                state.terms.append(item)

    def process_doc(self, app, document):
        state = State.get(app.env)
        for glossary in document.findall(addnodes.glossary):
            for term in glossary.findall(nodes.term):
                name = term.astext()
                state.terms.append(
                    Term(
                        name=name,
                        document=app.env.docname,
                        anchor=term["ids"][0],
                        # If the term is only made of uppercase letters we
                        # treat it as an abbreviation rather than a term.
                        abbreviation=all(c in string.ascii_uppercase for c in name),
                    )
                )


class LinkToTermsTransform(SphinxTransform):
    default_priority = 100

    def apply(self):
        state = State.get(self.env)
        for node in find_lexable_nodes(self.document):
            self.apply_to_node(state, node)

    def apply_to_node(self, state, node):
        lexed = list(lexer(node.astext(), state.terms))
        if len(lexed) == 1 and type(lexed[0]) == str:
            # Do nothing if the lexed version returned the same string.
            pass
        else:
            container = nodes.inline()
            for part in lexed:
                if type(part) == str:
                    container.append(nodes.Text(part))
                elif type(part) == MatchedTerm:
                    container.append(self.make_link(part))
                else:
                    raise RuntimeError("unexpected result of lexer")
            node.parent.replace(node, container)

    def make_link(self, matched):
        node = sphinx.util.nodes.make_refnode(
            self.app.builder,
            self.env.docname,
            matched.term.document,
            matched.term.anchor,
            nodes.Text(matched.text),
        )
        node["classes"].append("ferrocene-autoglossary")
        return node


class PruneGlossaryTransform(SphinxTransform):
    default_priority = 500

    def apply(self):
        state = State.get(self.env)
        glossaries = list(self.document.findall(addnodes.glossary))
        if glossaries:
            used_terms = self.discover_used_terms()
            for glossary in glossaries:
                self.prune(glossary, used_terms)

    # We have to re-scan all the documents to see which terms are used,
    # duplicating the scanning effort. Ideally we would first run the
    # LinkToTermsTransform transform and collect from there the terms that were
    # linked, and then pass that information to this transform.
    #
    # Unfortunately that's not possible, because each "post transform" is
    # executed on the process handling that file, and there is no way for those
    # processes to synchronize and communicate.
    def discover_used_terms(self):
        state = State.get(self.env)
        used_terms = set()
        for docname in self.env.all_docs.keys():
            doctree = self.env.get_doctree(docname)
            for node in find_lexable_nodes(doctree):
                for part in lexer(node.astext(), state.terms):
                    if type(part) == MatchedTerm:
                        used_terms.add(part.term.name)
        return used_terms

    def prune(self, glossary, used_terms):
        for item in list(glossary.findall(nodes.definition_list_item)):
            for term in item.findall(nodes.term):
                if term.astext() in used_terms:
                    break
            else:
                item.parent.remove(item)


@dataclass
class Term:
    name: str
    document: str
    anchor: str
    abbreviation: bool


@dataclass
class State:
    terms: list[Term]

    def get(env):
        key = "ferrocene_autoglossary"
        if not hasattr(env, key):
            setattr(env, key, State(terms=list()))
        return getattr(env, key)


def find_lexable_nodes(node, *, inside_glossary=False):
    if type(node) == nodes.Text:
        yield node
    elif type(node) == addnodes.glossary:
        inside_glossary = True

    for child in node.children:
        if inside_glossary and type(child) == nodes.term:
            continue
        for result in find_lexable_nodes(child, inside_glossary=inside_glossary):
            yield result


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
class MatchedTerm:
    term: Term
    text: str


def normalize(name):
    # This can be expanded to apply more normalizations before checking for
    # matches. The only constraint is that this function CANNOT add or remove
    # chars to the input, it MUST be the same length.
    return name.lower()


def setup(app):
    app.add_env_collector(GlossaryCollector)
    app.add_post_transform(LinkToTermsTransform)
    app.add_post_transform(PruneGlossaryTransform)

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
        # The version needs to be updated whenever there is a breaking change
        # in the data stored in the environment. Bumping the version number
        # will ensure Sphinx will do a clean build.
        #
        # Version history:
        # - 0: initial implementation
        "env_version": "0",
    }
