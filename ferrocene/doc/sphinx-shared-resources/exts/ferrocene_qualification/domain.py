# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from sphinx.directives import ObjectDescription
from sphinx.domains import Domain, ObjType
from sphinx.roles import XRefRole
import sphinx


class Id:
    def __init__(self, document, id):
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

    def add_target_and_index(self, name_cls, sig, signode):
        id = Id(self.env.docname, sig)
        signode["ids"].append(id.id)

        domain = self.env.get_domain("qualification")
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

    def add_id(self, id):
        self.data["ids"][id.id] = id

    def clear_doc(self, docname):
        self.data["ids"] = {
            key: value
            for key, value in self.data["ids"].items()
            if value.document != docname
        }

    def merge_domaindata(self, docnames, otherdata):
        for key, value in otherdata["ids"].items():
            if value.document in docnames:
                self.data["ids"][value.id] = value

    def resolve_xref(self, env, fromdocname, builder, type, target, node, contnode):
        if type != "id":
            raise RuntimeError(f"unsupported xref type {type}")

        if target not in self.data["ids"]:
            return
        id = self.data["ids"][target]

        return sphinx.util.nodes.make_refnode(
            builder, fromdocname, id.document, id.id, contnode
        )

    def get_objects(self):
        for id in self.data["ids"].values():
            # (name, display_name, type, document, anchor, priority)
            yield id.id, id.id, "id", id.document, id.id, 1


def setup(app):
    app.add_domain(QualificationDomain)
