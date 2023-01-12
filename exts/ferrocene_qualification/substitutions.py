# SPDX-License-Identifier: LicenseRef-Ferrocene
# SPDX-FileCopyrightText: Critical Section GmbH

# We use substitutions across all our documents, sharing a lot of them (like
# the name of the product). This extension sets some default substitutions, and
# supports loading additional ones from a TOML file.

from docutils import nodes
from sphinx.transforms import SphinxTransform
import docutils
import sphinx
import tomli


class AddCustomSubstitutions(SphinxTransform):
    # Run this step before substitutions are applied.
    default_priority = (
        min(
            sphinx.transforms.DefaultSubstitutions.default_priority,
            docutils.transforms.references.Substitutions.default_priority,
        )
        - 1
    )

    def apply(self):
        with open(self.app.config["ferrocene_substitutions_path"], "rb") as f:
            substitutions = tomli.load(f)
        for key, value in substitutions.items():
            self.add_substitution(key, value)

        self.add_substitution("doc_title", self.app.config["html_short_title"])
        self.add_substitution("doc_short_title", self.app.config["ferrocene_id"])

    def add_substitution(self, name, value):
        substitution = nodes.substitution_definition()
        substitution["names"].append(name)
        substitution += nodes.Text(value)

        self.document.substitution_defs[name] = substitution
        self.document.substitution_names[nodes.fully_normalize_name(name)] = name


def setup(app):
    app.add_transform(AddCustomSubstitutions)
