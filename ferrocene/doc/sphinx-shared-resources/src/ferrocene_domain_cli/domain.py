# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers


from dataclasses import dataclass
import re
import string
from typing import Optional

from docutils import nodes
from docutils.parsers.rst import directives
import docutils

from sphinx import addnodes
from sphinx.directives import SphinxDirective, ObjectDescription
from sphinx.domains import Domain, ObjType
from sphinx.roles import XRefRole
import sphinx


PROGRAM_STORAGE = "ferrocene_domain_cli:program"

ALLOWED_CATEGORIES = {
    "informational": "informational",
    "narrow": "narrow impact",
    "wide": "wide impact",
    "unqualified": "not qualified",
}


CATEGORIES_TARGET_REF = "evaluation-report:rustc-cli-testing-categories"


@dataclass
class ProgramStorage:
    program_name: str
    qualified: bool


class ProgramDirective(SphinxDirective):
    has_content = True
    required_arguments = 1
    final_argument_whitespace = True
    option_spec = {"not_qualified": directives.flag}

    def run(self):
        # if there already is program data in storage, a ProgramDirective is
        # within a ProgramDirective, which isn't supported
        if PROGRAM_STORAGE in self.env.temp_data:
            warn("cli:program inside cli:program isn't supported", self.get_location())
            return []

        # store arguments, so they can be accessed by child `OptionDirective`s
        self.env.temp_data[PROGRAM_STORAGE] = ProgramStorage(
            program_name=self.arguments[0],
            qualified="not_qualified" not in self.options,
        )

        # parse and process content of `ProgramDirective``
        # (one or more `OptionDirective`s)
        node = nodes.container()
        self.state.nested_parse(self.content, self.content_offset, node)

        # clear program storage
        del self.env.temp_data[PROGRAM_STORAGE]

        return [node]


class OptionDirective(ObjectDescription):
    has_content = True
    required_arguments = 1
    option_spec = {"category": directives.unchanged}

    def handle_signature(self, sig, signode):
        signode += addnodes.desc_name("", sig)

    def transform_content(self, content_node):
        category = None
        if self._program_storage().qualified and self.options["category"] in ALLOWED_CATEGORIES:
            category = self.options["category"]

        if category is not None:
            xref = addnodes.pending_xref()
            xref["reftype"] = "ref"
            xref["refdomain"] = "std"
            xref["refexplicit"] = True
            xref["refdoc"] = self.env.docname
            xref["reftarget"] = CATEGORIES_TARGET_REF
            xref += nodes.strong("", ALLOWED_CATEGORIES[self.options["category"]])

            paragraph = nodes.paragraph()
            paragraph += nodes.Text("Testing category: ")
            paragraph += xref
            content_node.insert(0, paragraph)

    def add_target_and_index(self, name_cls, sig, signode):
        program_storage = self._program_storage()

        if "category" not in self.options:
            if program_storage.qualified:
                warn("cli:option without a category", self.get_location())
            category = None
        elif self.options["category"] not in ALLOWED_CATEGORIES:
            warn(
                f"unsupported category: {self.options['category']}", self.get_location()
            )
            category = None
        else:
            if not program_storage.qualified:
                warn("category set in an unqualified program", self.get_location())
            category = self.options["category"]

        option = Option(
            document=self.env.docname,
            program=program_storage.program_name,
            option=sig,
            category=category,
            location=self.get_location(),
            no_traceability_matrix=not program_storage.qualified,
        )

        signode["ids"].append(option.id())

        domain = self.env.get_domain("cli")
        domain.add_option(option)

    def _program_storage(self) -> ProgramStorage:
        if PROGRAM_STORAGE not in self.env.temp_data:
            warn("cli:option outside cli:program isn't supported", self.get_location())
            return ProgramStorage("PLACEHOLDER", False)
        else:
            return self.env.temp_data[PROGRAM_STORAGE]


ARGUMENT_PLACEHOLDER_RE = re.compile(r"(<[^>]+>|\[[^\]]+\])")
MULTIPLE_UNDERSCORES_RE = re.compile(r"__+")
ALLOWED_CHARS_IN_OPTION_ID = string.ascii_letters + string.digits + "_"


@dataclass
class Option:
    document: str
    program: str
    option: str
    category: Optional[str]
    location: str
    no_traceability_matrix: bool

    def id(self):
        option = (
            ARGUMENT_PLACEHOLDER_RE.sub("", self.option)
            .replace("=", "")
            .replace("-", "_")
            .replace(" ", "_")
            .strip("_")
        )
        option = MULTIPLE_UNDERSCORES_RE.sub("_", option)

        # Sanity check to notice when the normalization doesn't work
        if any(c for c in option if c not in ALLOWED_CHARS_IN_OPTION_ID):
            warn(f"cannot properly normalize option {self.option}", self.location)

        return f"um_{self.program}_{option}"


class CliDomain(Domain):
    name = "cli"
    labels = "Command-line interface"
    roles = {
        "option": XRefRole(),
    }
    directives = {
        "program": ProgramDirective,
        "option": OptionDirective,
    }
    object_types = {
        "option": ObjType("CLI option", "option"),
    }
    initial_data = {"options": {}}
    # Bump whenever the format of the data changes!
    data_version = 1

    def add_option(self, option):
        self.data["options"][f"{option.program} {option.option}"] = option

    def get_options(self):
        return self.data["options"]

    def clear_doc(self, docname):
        self.data["options"] = {
            key: item
            for key, item in self.data["options"].items()
            if item.document != docname
        }

    def merge_domaindata(self, docnames, otherdata):
        for key, option in otherdata["options"].items():
            if option.document in docnames:
                self.data["options"][key] = option

    def resolve_xref(self, env, fromdocname, builder, type, target, node, contnode):
        if type != "option":
            raise RuntimeError(f"unsupported xref type {type}")

        if target not in self.data["options"]:
            return
        option = self.data["options"][target]

        return sphinx.util.nodes.make_refnode(
            builder, fromdocname, option.document, option.id(), contnode
        )

    def get_objects(self):
        for key, option in self.data["options"].items():
            yield (
                key,  # Name
                f"{option.program} {option.option}",  # Display name
                "option",  # Type
                option.document,  # Document name
                option.id(),  # Anchor
                0,  # Priority
            )


def warn(message, location):
    logger = sphinx.util.logging.getLogger(__name__)
    logger.warn(message, location=location)


def setup(app):
    app.add_domain(CliDomain)
