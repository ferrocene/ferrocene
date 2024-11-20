# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers


from dataclasses import dataclass
import re
import string

from docutils import nodes
from docutils.parsers.rst import directives

from sphinx import addnodes
from sphinx.directives import SphinxDirective, ObjectDescription
from sphinx.domains import Domain, ObjType
from sphinx.roles import XRefRole
import sphinx


PROGRAM_STORAGE = "ferrocene_domain_cli:program"


class ProgramDirective(SphinxDirective):
    has_content = True
    required_arguments = 1
    final_argument_whitespace = True
    option_spec = {"no_traceability_matrix": directives.flag}

    def run(self):
        # if there already is program data in storage, a ProgramDirective is
        # within a ProgramDirective, which isn't supported
        if PROGRAM_STORAGE in self.env.temp_data:
            warn("cli:program inside cli:program isn't supported", self.get_location())
            return []

        # store arguments, so they can be accessed by child `OptionDirective`s
        self.env.temp_data[PROGRAM_STORAGE] = ProgramStorage(
            self.arguments[0],
            "no_traceability_matrix" in self.options,
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
    option_spec = {}

    def handle_signature(self, sig, signode):
        signode += addnodes.desc_name("", sig)

    def add_target_and_index(self, name_cls, sig, signode):
        if PROGRAM_STORAGE not in self.env.temp_data:
            warn("cli:option outside cli:program isn't supported", self.get_location())
            program_storage = ProgramStorage("PLACEHOLDER", False)
        else:
            program_storage: ProgramStorage = self.env.temp_data[PROGRAM_STORAGE]

        option = Option(
            self.env.docname,
            program_storage.program_name,
            sig,
            program_storage.no_traceability_matrix,
        )

        signode["ids"].append(option.id())

        domain = self.env.get_domain("cli")
        domain.add_option(option)


ARGUMENT_PLACEHOLDER_RE = re.compile(r"(<[^>]+>|\[[^\]]+\])")
MULTIPLE_UNDERSCORES_RE = re.compile(r"__+")
ALLOWED_CHARS_IN_OPTION_ID = string.ascii_letters + string.digits + "_"


class Option:
    def __init__(self, document, program, option, no_traceability_matrix):
        self.document = document
        self.program = program
        self.option = option
        self.no_traceability_matrix = no_traceability_matrix

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
            warn(f"cannot properly normalize option {self.option}")

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


@dataclass
class ProgramStorage:
    program_name: str
    no_traceability_matrix: bool
