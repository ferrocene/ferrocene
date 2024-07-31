# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This part of the extension adds the `.. rust-changelog::` directive to inject
# Rust changes from usptream's RELEASES.md.

from docutils import nodes
from myst_parser.docutils_ import Parser
from semver import Version
from sphinx.directives import SphinxDirective
import docutils
import sphinx


class RustChangelogDirective(SphinxDirective):
    has_content = False
    option_spec = {
        "from": docutils.parsers.rst.directives.unchanged_required,
        "to": docutils.parsers.rst.directives.unchanged_required,
    }

    def run(self):
        # Filter all versions in semver range
        exprs = [f">={self.options['from']}", f"<={self.options['to']}"]
        versions = []
        for version in self.env.rust_release_notes.keys():
            if all(version.match(expr) for expr in exprs):
                versions.append(version)

        result = []
        for version in sorted(versions):
            result.append(self.env.rust_release_notes[version])
        return result


def load_release_notes(app):
    # Loading the release notes is fairly slow (takes more than a few seconds
    # on Pietro's machine), and we don't need them when collecting intersphinx
    # references. This saves time when building documents.
    if app.builder.name == "ferrocene-intersphinx":
        app.env.rust_release_notes = {}
    else:
        with sphinx.util.display.progress_message("loading Rust release notes"):
            app.env.rust_release_notes = split_release_notes(
                read_release_notes(app.config.rust_release_notes)
            )


def read_release_notes(path):
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()

    parsed = docutils.core.publish_doctree(
        source=content,
        parser=Parser(),
        # Suppress MyST warnings:
        settings_overrides={"report_level": 999},
    )

    # Parts of the rendered release notes include raw HTML, remove it. This is
    # mostly used by upstream for <a id="1.NN-section">.
    for node in list(parsed.findall(nodes.raw)):
        node.parent.remove(node)
    # MyST complains a lot about duplicate references, remove the errors.
    for node in list(parsed.findall(nodes.system_message)):
        node.parent.remove(node)

    return parsed


def split_release_notes(doctree):
    result = {}

    for section in doctree.children:
        if type(section) != nodes.section:
            continue
        for node in section.children:
            if type(node) == nodes.title:
                title_node = node
                title = node.astext()
                break
        else:
            raise RuntimeError("top-level section without title")

        raw_version = title.split(" ")[1]
        # Needed if version is not SemVer compatible
        if len(raw_version.split(".")) == 2:
            raw_version += ".0"
        try:
            version = Version.parse(raw_version)
        except ValueError as e:
            # Old releases have invalid version numbers according to semver,
            # but we don't care about them.
            if raw_version.startswith("0."):
                continue
            raise e

        # Replace the content of upstream's title with ours.
        for child in list(title_node.children):
            title_node.remove(child)
        title_node += nodes.Text(f"Rust {version}")

        # The section identifiers, used for anchors, are dynamically generated
        # by MyST, but unfortunately are not consistent (they are in the form
        # of `language-{number}`, with the number increased every time the
        # section is repeated), which means all anchors would become invalid
        # when a new release is published by upstream.
        #
        # We thus have to replace the identifiers with our own.
        replace_section_identifiers(section)

        result[version] = section

    return result


def replace_section_identifiers(node, prefix=""):
    if type(node) == nodes.section:
        for child in node.children:
            if type(child) == nodes.title:
                title = child
                break
        else:
            raise RuntimeError("section without title")

        slug = slugify(prefix + title.astext())
        prefix = f"{slug}-"

        node["ids"] = [slug]

    for child in node.children:
        replace_section_identifiers(child, prefix)


def slugify(text):
    result = ""
    last_was_ascii = False
    for c in text:
        if c.isalnum():
            result += c.lower()
            last_was_ascii = True
        elif last_was_ascii:
            result += "-"
            last_was_ascii = False
    return result


def setup(app):
    app.add_directive("rust-changelog", RustChangelogDirective)

    app.connect("builder-inited", load_release_notes)
    app.add_config_value("rust_release_notes", None, "env", [str])
