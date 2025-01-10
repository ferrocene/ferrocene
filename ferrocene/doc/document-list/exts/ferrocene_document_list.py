# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import os
from docutils import nodes
import sphinx
from sphinx.roles import SphinxRole

class DocumentId(SphinxRole):

    def run(self):
        var = f"FERROCENE_DOCUMENT_ID_{self.text}"
        try:
            path = os.environ[var]
        except KeyError:
            # the build system only injects env variables for existing documents
            # ... hence a similar error message to when path is not found (below)
            return [], [self.error(self.text)]
        if not os.path.exists(path):
            return [], [self.error(path)]
        with open(path, "r", encoding="utf-8") as file:
            id = file.read()
        node = nodes.literal(text=id)
        return [node], []

    def error(self, id):
        logger = sphinx.util.logging.getLogger(__name__)
        message = f'Error! Document "{id}" not found'
        logger.warn(message, location=self.get_location())
        return nodes.inline(text=message)

def setup(app):
    app.add_role('document-id', DocumentId())

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }
