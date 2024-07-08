# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from . import definitions, informational, syntax_directive, std_role, paragraph_ids
from . import items_with_rubric, sphinx_fixes
from sphinx.domains import Domain


class SpecDomain(Domain):
    name = "spec"
    label = "Specification"
    roles = {
        **definitions.get_roles(),
        "std": std_role.StdRefRole(),
    }
    directives = {
        "syntax": syntax_directive.SyntaxDirective,
        "informational-page": informational.build_directive("page"),
        "informational-section": informational.build_directive("section"),
        "items-with-rubric": items_with_rubric.ItemsWithRubricDirective,
    }
    object_types = definitions.get_object_types()
    indices = {}

    def get_objects(self):
        return definitions.get_objects(self.env)

    def merge_domaindata(self, docnames, other):
        def is_empty(data):
            return not data or list(data.keys()) == ["version"]

        if not is_empty(self.data) or not is_empty(other):
            raise NotImplementedError(
                "there is data in the domain, merge_domaindata should be implemented"
            )


def setup(app):
    app.add_domain(SpecDomain)
    definitions.setup(app)
    paragraph_ids.setup(app)
    informational.setup(app)
    items_with_rubric.setup(app)
    sphinx_fixes.setup(app)

    app.add_config_value(
        name="spec_std_docs_url",
        default="https://doc.rust-lang.org/stable/std",
        rebuild="env",  # Rebuild the environment when this changes
        types=[str],
    )

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
        # - 1: changed how informational sections and pages are stored
        "env_version": "1",
    }
