# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

from docutils import nodes
from sphinx.roles import SphinxRole
from urllib.parse import quote
from .definitions import parse_target_from_text


class StdRefRole(SphinxRole):
    def run(self):
        text, target = parse_target_from_text(self.text)
        url = f"{self.env.config.spec_std_docs_url}/?search={quote(target)}"

        node = nodes.reference(internal=False, refuri=url)
        node += nodes.literal("", text)

        return [node], []
