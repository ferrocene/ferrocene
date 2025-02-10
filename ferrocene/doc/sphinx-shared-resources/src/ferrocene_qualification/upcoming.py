# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from sphinx.util.docutils import SphinxRole
from sphinx.transforms import SphinxTransform
from sphinx.util.logging import getLogger


BADGE_CONTENT = "coming soon"


class UpcomingRole(SphinxRole):
    def run(self):
        content = self.text

        try:
            major, minor = content.split(".")
            role_version = (int(major), int(minor))
        except ValueError:
            return self.error(f"{self.text} is not a version in the YY.MM format")

        if self.env.config.ferrocene_version != "rolling":
            try:
                current_major, current_minor, *_ = (
                    self.env.config.ferrocene_version.split(".")
                )
                current_version = (int(current_major), int(current_minor))
            except ValueError:
                raise RuntimeError("invalid Ferrocene version injected")

            # Don't render the role if we are outside of rolling and the release referenced by this
            # role is the current or a past one.
            if current_version >= role_version:
                return [], []

        return [build_badge_node()], []

    def error(self, message):
        # Show a warning in the build log and fail the build.
        logger = getLogger(__name__)
        logger.warning(
            f"{self.text} is not a version in the YY.MM format",
            location=self.get_location(),
        )
        # Show the error message in the generated HTML.
        problematic = nodes.problematic()
        problematic += nodes.Text(message)
        return [problematic], []


# Annoyingly, Sphinx's :doc: role strips any rich formatting from the page title when generating the
# link label, which turns the badge into plaintext. This transform re-adds the badge to the links it
# was removed from.
class FixLinksWithBadge(SphinxTransform):
    default_priority = 500

    def apply(self):
        suffix_text = build_badge_node().astext()

        for link in self.document.findall(nodes.reference):
            # Match exactly the AST of the links we want to replace.
            if "internal" not in link:
                continue
            if len(link.children) != 1:
                continue
            if not isinstance(link[0], nodes.inline):
                continue
            if "std-ref" not in link[0]["classes"]:
                continue
            if len(link[0].children) != 1:
                continue
            if not isinstance(link[0][0], nodes.Text):
                continue

            # Replace the plaintext badge with the
            current_text = link[0][0].astext()
            if current_text.endswith(suffix_text):
                del link[0][0]
                link[0] += nodes.Text(current_text.removesuffix(suffix_text))
                link[0] += build_badge_node()


def build_badge_node():
    # Unfortunately we cannot just return a <span class="badge-yellow">. While doing so would
    # work great whenever the badge is included in a paragraph/table/list, it would break in
    # some contexts when used as a page/section title.
    #
    # When rich formatting is used inside of a title, in most cases it will be rendered as HTML
    # (table of contents, navbar, <h1> tag). When used in the page's <title>, Sphinx will not
    # render any rich formatting, and instead strip all HTML tags. This would leave a "coming
    # soon" plaintext suffix, which is quite ugly.
    #
    # To work around the problem, we add plaintext formatting around the badge with the .hidden
    # CSS class. This way it won't be rendered in places with rich formatting enabled, but it
    # will be shown in places where the title is converted to plaintext.
    #
    # Note that technically this problem doesn't only appear in the page <title>, but also when
    # linking to a document using the :doc: role. The `FixLinksWithBadge` fixes that.
    badge = nodes.inline()
    badge += span_with_class("(", "hidden")
    badge += span_with_class(BADGE_CONTENT, "badge-yellow")
    badge += span_with_class(")", "hidden")
    return badge


def span_with_class(text, css_class):
    node = nodes.inline()
    node["classes"].append(css_class)
    node += nodes.Text(text)
    return node


def setup(app):
    app.add_role("upcoming", UpcomingRole())
    app.add_post_transform(FixLinksWithBadge)
