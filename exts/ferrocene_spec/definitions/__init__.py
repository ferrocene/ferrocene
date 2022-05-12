from . import paragraphs, syntax, terms, code_terms
from dataclasses import dataclass
from docutils import nodes
from sphinx import addnodes
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


class DefIdNode(nodes.General, nodes.Element):
    def __init__(self, kind, id):
        super().__init__()
        self.def_kind = kind
        self.def_id = id

    def __repr__(self):
        return f'<DefIdNode {self.def_kind} "{self.def_id}" />'

    def __str__(self):
        return repr(self)


class DefIdRole(SphinxRole):
    def __init__(self, kind):
        self.kind = kind

    def run(self):
        return [DefIdNode(self.kind, self.text)], []


@dataclass
class Reference:
    id: str
    document: str


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

        refs = get_refs_storage(env)
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

        refs = get_refs_storage(env)
        other_refs = get_refs_storage(other)
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
                lambda node: node.def_kind == kind.NAME,
                document.findall(DefIdNode),
            )
            for item in kind.collect_items_in_document(app, nodes):
                storage[item.id] = item

        refs = get_refs_storage(app.env)
        for node in document.findall(addnodes.pending_xref):
            if node["refdomain"] != "spec":
                continue
            refs.append(Reference(id=node["reftarget"], document=node["refdoc"]))


class DefinitionsTransform(SphinxTransform):
    default_priority = 500

    def apply(self):
        for kind in KINDS:
            storage = get_storage(self.env, kind)

            for node in self.document.findall(DefIdNode):
                if node.def_kind != kind.NAME:
                    continue
                item = storage[node.def_id]
                kind.replace_id_node(self.app, node, item)


def handle_ref(builder, env, fromdocname, target):
    def make_link(document, target, contents):
        return sphinx.util.nodes.make_refnode(
            builder,
            fromdocname,
            document,
            target,
            contents,
        )

    for kind in KINDS:
        storage = get_storage(env, kind)
        if target in storage:
            return kind.create_ref_node(env, storage[target], make_link)

    # TODO: handle missing references, and collect all of them in a page


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
            search_name = item.search_name(env)
            result.append(
                (
                    item.id,
                    search_name if search_name is not None else item.id,
                    kind.NAME,
                    item.document,
                    item.anchor(),
                    1 if search_name is not None else -1,
                )
            )

    return result


def get_storage(env, kind):
    key = f"spec_items_{kind.NAME}"
    if not hasattr(env, key):
        setattr(env, key, {})
    return getattr(env, key)


def get_refs_storage(env):
    key = "spec_refs"
    if not hasattr(env, key):
        setattr(env, key, [])
    return getattr(env, key)


def get_roles():
    result = {}
    for kind in KINDS:
        result[kind.ROLE] = DefIdRole(kind.NAME)
    return result


def get_object_types():
    result = {}
    for kind in KINDS:
        result[kind.NAME] = ObjType(kind.PRETTY_NAME, kind.ROLE)
    return result


def setup(app):
    app.add_node(DefIdNode)
    app.add_env_collector(DefinitionsCollector)
    app.add_post_transform(DefinitionsTransform)
