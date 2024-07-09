# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers


# 3rd-party imports
from docutils import nodes
from sphinx import addnodes
from sphinx.application import Sphinx
from sphinx.builders import Builder
from sphinx.directives import ObjectDescription
from sphinx.domains import Domain, ObjType
from sphinx.roles import XRefRole
from sphinx.util import nodes as sphinx_nodes


class Id:
    def __init__(self, document: str, id: str):
        self.document = document
        self.id = id


class IdDirective(ObjectDescription):
    has_content = False
    required_arguments = 1
    option_spec = {}

    def handle_signature(self, sig, signode):
        prefix = nodes.inline()
        prefix["classes"].append("hide-inside-tables")
        prefix += nodes.strong("", "Identifier:")
        prefix += nodes.Text(" ")

        signode += prefix
        signode += nodes.literal("", sig)

    def add_target_and_index(
        self, _name_cls, sig: str, signode: addnodes.desc_signature
    ):
        id = Id(self.env.docname, sig)
        signode["ids"].append(id.id)

        domain: QualificationDomain = self.env.get_domain("qualification")
        domain.add_id(id)


class QualificationDomain(Domain):
    name = "qualification"
    labels = "Qualification Documents"
    directives = {
        "id": IdDirective,
    }
    roles = {
        "id": XRefRole(),
    }
    object_types = {
        "id": ObjType("Identifier", "id"),
    }

    initial_data = {"ids": {}}
    # Bump whenever the format of the data changes!
    data_version = 1

    def add_id(self, id: Id):
        self.get_ids()[id.id] = id

    def get_ids(self) -> dict[str, Id]:
        return self.data["ids"]

    def clear_doc(self, docname: str):
        self.data["ids"] = {
            key: value
            for key, value in self.get_ids().items()
            if value.document != docname
        }

    def merge_domaindata(self, docnames, otherdata):
        other_ids: dict[str, Id] = otherdata["ids"]
        for other_id in other_ids.values():
            if other_id.document in docnames:
                self.get_ids()[other_id.id] = other_id

    def resolve_xref(
        self,
        _env,
        fromdocname: str,
        builder: Builder,
        type: str,
        target: str,
        _node,
        contnode: nodes.Node,
    ):
        if type != "id":
            raise RuntimeError(f"unsupported xref type {type}")

        if target not in self.get_ids():
            return
        id = self.get_ids()[target]

        return sphinx_nodes.make_refnode(
            builder, fromdocname, id.document, id.id, contnode
        )

    def get_objects(self):
        for id in self.get_ids().values():
            # (name, display_name, type, document, anchor, priority)
            yield id.id, id.id, "id", id.document, id.id, 1


def setup(app: Sphinx):
    app.add_domain(QualificationDomain)
