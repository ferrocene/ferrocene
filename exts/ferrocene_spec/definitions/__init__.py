# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from . import paragraphs, syntax, terms, code_terms
from docutils import nodes
from sphinx.domains import ObjType
from sphinx.environment.collectors import EnvironmentCollector
from sphinx.roles import SphinxRole
from sphinx.transforms import SphinxTransform
import sphinx


KINDS = [
    paragraphs,
    syntax,
    terms,
    code_terms,
]


class DefIdNode(nodes.Element):
    def __init__(self, kind, text):
        super().__init__(def_kind=kind, def_text=text, def_id=id_from_text(text))

    def astext(self):
        return self["def_text"]


class DefRefNode(nodes.Element):
    def __init__(self, kind, source_doc, text):
        if "<" in text and text.endswith(">"):
            target_start = text.rfind("<")
            target = text[target_start + 1 : len(text) - 1]
            text = text[:target_start].rstrip()
        elif "[" in text and "]" in text:
            target = text[text.find("[") + 1 : text.rfind("]")]
            text = text.replace("[", "").replace("]", "")
        else:
            target = text

        super().__init__(
            ref_kind=kind,
            ref_source_doc=source_doc,
            ref_text=text,
            ref_target=id_from_text(target),
        )

    def astext(self):
        return self["ref_text"]


class DefIdRole(SphinxRole):
    def __init__(self, kind):
        self.kind = kind

    def run(self):
        return [DefIdNode(self.kind, self.text)], []


class DefRefRole(SphinxRole):
    def __init__(self, kind):
        self.kind = kind

    def run(self):
        return [DefRefNode(self.kind, self.env.docname, self.text)], []


class Reference:
    def __init__(self, kind, id, document):
        self.kind = kind
        self.id = id
        self.document = document


class DefinitionsCollector(EnvironmentCollector):
    def clear_doc(self, app, env, docname):
        """
        Remove all definitions and references contained into a document.

        This is called by Sphinx during incremental builds, either when a
        document was removed or when the document has been changed. In the
        latter case, process_doc is called after this method.
        """
        for kind in KINDS:
            storage = get_storage(env, kind)
            for item in list(storage.values()):
                if item.document == docname:
                    del storage[item.id]

            refs = get_refs_storage(env, kind)
            removed = 0
            for i, ref in enumerate(list(refs)):
                if ref.document == docname:
                    del refs[i - removed]
                    removed += 1

    def merge_others(self, app, env, docnames, other):
        """
        Merge the collected informations from two environments into one.

        Sphinx supports parallel builds, with each process having its own
        environment instance, but once each document is processed those
        parallel environments need to be merged together. This method does it.
        """
        for kind in KINDS:
            storage = get_storage(env, kind)
            other_storage = get_storage(other, kind)

            for item in other_storage.values():
                if item.document in docnames:
                    storage[item.id] = item

            refs = get_refs_storage(env, kind)
            other_refs = get_refs_storage(other, kind)
            for ref in other_refs:
                refs.append(ref)

    def process_doc(self, app, document):
        """
        Collect all the definitions and references present in the document.

        This method can expect no existing information about the same document
        being stored in the environment, as during incremental rebuilds the
        clear_doc method is called ahead of this one.
        """
        for kind in KINDS:
            storage = get_storage(app.env, kind)
            nodes = filter(
                lambda node: node["def_kind"] == kind.NAME,
                document.findall(DefIdNode),
            )
            for item in kind.collect_items_in_document(app, nodes):
                storage[item.id] = item

            refs = get_refs_storage(app.env, kind)
            for node in document.findall(DefRefNode):
                if node["ref_kind"] != kind.NAME:
                    continue
                refs.append(
                    Reference(
                        kind=node["ref_kind"],
                        document=node["ref_source_doc"],
                        id=node["ref_target"],
                    )
                )


class DefinitionsTransform(SphinxTransform):
    default_priority = 500

    def apply(self):
        for kind in KINDS:
            storage = get_storage(self.env, kind)

            for node in self.document.findall(DefIdNode):
                if node["def_kind"] != kind.NAME:
                    continue
                item = storage[node["def_id"]]
                kind.replace_id_node(self.app, node, item)

            for node in self.document.findall(DefRefNode):
                if node["ref_kind"] != kind.NAME:
                    continue

                if node["ref_target"] in storage:
                    item = storage[node["ref_target"]]
                    node.replace_self(
                        sphinx.util.nodes.make_refnode(
                            self.app.builder,
                            node["ref_source_doc"],
                            item.document,
                            item.anchor(),
                            kind.create_ref_node(self.env, node["ref_text"], item),
                        )
                    )
                else:
                    new = nodes.inline(
                        "",
                        "",
                        kind.create_ref_node(self.env, node["ref_text"], None),
                    )
                    new["classes"].append("spec-missing-ref")
                    node.replace_self(new)


def get_objects(env):
    """
    Get objects that will be included in the objects.inv file and in search.

    The return type is a list of tuples of:
    * Fully qualified name
    * Display name in search
    * Object type
    * Document name
    * Anchor
    * Search priority (1: default, 0: important, 2: not important, -1: ignore)
    """
    result = []
    for kind in KINDS:
        storage = get_storage(env, kind)
        for item in storage.values():
            result.append(
                (
                    item.id,
                    item.display_name(env),
                    kind.NAME,
                    item.document,
                    item.anchor(),
                    1 if item.include_in_search() is not None else -1,
                )
            )

    return result


def get_storage(env, kind):
    key = f"spec_items_{kind.NAME}"
    if not hasattr(env, key):
        setattr(env, key, {})
    return getattr(env, key)


def get_refs_storage(env, kind):
    key = "spec_refs_{kind.NAME}"
    if not hasattr(env, key):
        setattr(env, key, [])
    return getattr(env, key)


def get_roles():
    result = {}
    for kind in KINDS:
        result["d" + kind.ROLE[0]] = DefIdRole(kind.NAME)
        result[kind.ROLE[0]] = DefRefRole(kind.NAME)
    return result


def get_object_types():
    result = {}
    for kind in KINDS:
        result[kind.NAME] = ObjType(kind.PRETTY_NAME, kind.ROLE)
    return result


def id_from_text(text):
    return "".join(c if c.isalnum() else "_" for c in text.lower())


def setup(app):
    app.add_node(DefIdNode)
    app.add_env_collector(DefinitionsCollector)
    app.add_post_transform(DefinitionsTransform)
