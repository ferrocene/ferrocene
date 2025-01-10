# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This module adds some helpers needed to integrate Ferrocene's build system
# with InterSphinx. More specifically, the extension:
#
# - Defines the "ferrocene-intersphinx" Sphinx builder, which only produces the
#   objects.inv file required by InterSphinx. This is used to gather all the
#   inventories for all of our documentation before actually building anything,
#   as we have circular references between documents.
#
# - Defines the "ferrocene_intersphinx_mappings" configuration, which this
#   extension deserializes from JSON and then adds to the intersphinx_mapping
#   configuration. This is needed because the format of intersphinx_mapping is
#   too complex to be provided with the -D flag.

from sphinx.builders import Builder
from sphinx.builders.html import StandaloneHTMLBuilder
import json
import sphinx
import sphinx.ext.intersphinx


class IntersphinxBuilder(Builder):
    name = "ferrocene-intersphinx"
    format = ""
    epilog = "InterSphinx inventory file generated."
    allow_parallel = True

    def init(self):
        self.standalone_html_builder = StandaloneHTMLBuilder(self.app, self.env)

        # Do not emit any warning in the ferrocene-intersphinx builder: there
        # will be warnings when using the builder, as the rest of the documents
        # won't be built yet, but we don't care about them.
        #
        # Keeping the warnings will confuse people who read the build logs,
        # thinking they should fix them while they're expected to happen.
        #
        # Unfortunately the only reliable way to suppress the warnings is
        # monkey-patching Sphinx's code, as you cannot set a global filter in
        # Python's logging module.
        sphinx.util.logging.WarningStreamHandler.emit = lambda _self, _record: None

    def build(self, *args, **kwargs):
        # Normally you're not supposed to override the build() method, as
        # Sphinx calls all the relevant overrideable methods from it.
        #
        # Unfortunately though, Sphinx doesn't execute the finish() method if
        # there are no outdated docs (as we're simulating in this builder).
        #
        # Returning all documents from get_outdated_docs() would fix that
        # problem, but would also execute all the post_transforms for all
        # documents, which on large documents can take a while.
        #
        # Instead, we're returning an empty list of outdated documents, and
        # manually dumping the inventory here after the parent build() returns.
        super().build(*args, **kwargs)
        self.standalone_html_builder.dump_inventory()

    def get_outdated_docs(self):
        return []

    def prepare_writing(self, docnames):
        pass

    def write_doc(self, docname, doctree):
        pass

    def get_target_uri(self, docname, typ=None):
        # Defer to the standalone HTML builder to generate builders.
        return self.standalone_html_builder.get_target_uri(docname, typ)


def inject_intersphinx_mappings(app, config):
    if config.ferrocene_intersphinx_mappings is not None:
        for inventory in json.loads(config.ferrocene_intersphinx_mappings):
            config.intersphinx_mapping[inventory["name"]] = (
                inventory["html_root"],
                inventory["inventory"],
            )


def setup(app):
    # Automatically enable the sphinx.ext.intersphinx extension without
    # requiring users to configure it in their conf.py.
    sphinx.ext.intersphinx.setup(app)

    app.add_builder(IntersphinxBuilder)

    app.add_config_value("ferrocene_intersphinx_mappings", None, "env", [str])
    app.connect("config-inited", inject_intersphinx_mappings, priority=1)
