# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This extension automatically inserts links across the whole document pointing
# to the relevant terms in the glossary, without having to manually annotate
# words with :term:`Glossary Entry`.
#
# The extension requires no configuration, and just adding a glossary directive
# is enough for the extension to work.

from . import debug, lexer
from .lexer import Term, MatchedTerm
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
        for node in lexer.find_lexable_nodes(self.document):
            self.apply_to_node(state, node.node)

    def apply_to_node(self, state, node):
        lexed = list(lexer.lexer(node.astext(), state.terms))
        if len(lexed) == 1 and type(lexed[0]) is str:
            # Do nothing if the lexed version returned the same string.
            pass
        else:
            container = nodes.inline()
            for part in lexed:
                if type(part) is str:
                    container.append(nodes.Text(part))
                elif type(part) is MatchedTerm:
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
        # The key of this dict is the name of the term, while the value is the
        # "dependencies" of the term. When the term is referred to by any page
        # other than the glossary the dependencies will be None, while if it's
        # only referenced by other glossary entries the value will be the names
        # of those entries.
        used_terms = dict()

        state = State.get(self.env)
        for docname in self.env.all_docs.keys():
            doctree = self.env.get_doctree(docname)
            for node in lexer.find_lexable_nodes(doctree):
                for part in lexer.lexer(node.node.astext(), state.terms):
                    if type(part) is not MatchedTerm:
                        continue
                    name = part.term.name
                    # Join the list of dependencies, setting None when either
                    # side is None.
                    if name not in used_terms:
                        used_terms[name] = node.inside_definition_of
                    elif used_terms[name] is not None:
                        if node.inside_definition_of is None:
                            used_terms[name] = None
                        else:
                            used_terms[name].update(node.inside_definition_of)

        # Keep resolving dependencies until no change is made in the previous
        # iteration. This makes sure that term "A" referred by term "B"
        # referred by term "C" referred by the rest of the text is kept.
        changed = True
        while changed:
            changed = False
            for term, depends_on in dict(used_terms.items()).items():
                if depends_on is None:
                    continue
                for dependency in depends_on:
                    if dependency in used_terms and used_terms[dependency] is None:
                        used_terms[term] = None
                        changed = True
                        break

        return {term for term, deps in used_terms.items() if deps is None}

    def prune(self, glossary, used_terms):
        for item in list(glossary.findall(nodes.definition_list_item)):
            for term in item.findall(nodes.term):
                if term.astext() in used_terms:
                    break
            else:
                item.parent.remove(item)


@dataclass
class State:
    terms: list[Term]

    def get(env):
        key = "ferrocene_autoglossary"
        if not hasattr(env, key):
            setattr(env, key, State(terms=list()))
        return getattr(env, key)


def setup(app):
    app.add_env_collector(GlossaryCollector)
    app.add_post_transform(LinkToTermsTransform)
    app.add_post_transform(PruneGlossaryTransform)

    debug.setup(app)

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
