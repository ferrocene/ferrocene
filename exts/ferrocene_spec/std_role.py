# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from docutils import nodes
from sphinx.roles import SphinxRole
from urllib.parse import quote


class StdRefRole(SphinxRole):
    def run(self):
        url = f"https://doc.rust-lang.org/stable/std/?search={quote(self.text)}"

        node = nodes.reference(internal=False, refuri=url)
        node += nodes.literal("", self.text)

        return [node], []
