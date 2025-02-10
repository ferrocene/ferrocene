# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This part of the extension processes the `:upcoming-release:` file-wide
# metadata, adding an "upcoming" badge next to the page title, and a disclaimer
# at the top of the page mentioning that the contents of the page are subject
# to change.

from sphinx.transforms import SphinxTransform
from docutils import nodes
from .utils import parse_docinfo, span_with_class


CAUTION_MESSAGE = (
    "This version of Ferrocene has not been released yet. The contents of this "
    "page might change before the release, and they should not be relied upon."
)


class MarkPageAsUpcoming(SphinxTransform):
    default_priority = 500

    def apply(self):
        docinfo = parse_docinfo(self.document)
        if "upcoming-release" not in docinfo:
            return

        for child_section in self.document.children:
            if type(child_section) != nodes.section:
                continue
            for child_title in child_section.children:
                if type(child_title) != nodes.title:
                    continue
                self.add_badge(child_title)
                self.add_caution_after(child_title)

    def add_badge(self, title_node):
        # In Sphinx, the <title> element is not only used to display the
        # relevant heading in the HTML output. It is also used as the HTML
        # title, in the table of contents, and in the navigation sidebar.
        #
        # All of those places *except* for the HTML title support spans with
        # CSS classes, and thus the "upcoming" badge is rendered correctly.
        #
        # HTML titles don't support nested HTML markup though, so Sphinx
        # converts the node structure into plain text. To ensure the title is
        # formatted correctly, we add extra markup hidden with CSS that will be
        # displayed only in the HTML title.
        title_node += nodes.Text(" ")
        title_node += span_with_class("(", "hidden")
        title_node += span_with_class("upcoming", "badge-yellow")
        title_node += span_with_class(")", "hidden")

    def add_caution_after(self, after_node):
        caution = nodes.caution()
        caution += nodes.paragraph("", CAUTION_MESSAGE)

        after_node.parent.insert(after_node.parent.index(after_node) + 1, caution)


def setup(app):
    app.add_transform(MarkPageAsUpcoming)
