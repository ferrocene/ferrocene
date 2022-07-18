# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from . import definitions, utils
from collections import defaultdict
from docutils import nodes
from sphinx.environment.collectors import EnvironmentCollector
import json
import os
import sphinx


def write_paragraph_ids(app):
    env = app.env

    paragraphs_by_section = defaultdict(list)
    for paragraph in definitions.get_storage(env, definitions.paragraphs).values():
        paragraphs_by_section[paragraph.section_id].append(
            {
                "id": paragraph.id,
                "number": paragraph.number(app.env),
                "link": app.builder.get_target_uri(paragraph.document)
                + "#"
                + paragraph.id,
                "checksum": paragraph.content_checksum(),
            }
        )

    sections_by_document = defaultdict(list)
    for section in env.spec_sections:
        sections_by_document[section.document].append(
            {
                "id": section.id,
                "number": ".".join(
                    str(n) for n in env.toc_secnumbers[section.document][section.anchor]
                ),
                "title": section.title,
                "link": app.builder.get_target_uri(section.document) + section.anchor,
                "paragraphs": paragraphs_by_section[section.id],
            }
        )

    documents = []
    for docname, title in env.titles.items():
        documents.append(
            {
                "title": title.astext(),
                "link": app.builder.get_target_uri(docname),
                "sections": sections_by_document[docname],
            }
        )

    with open(os.path.join(app.outdir, "paragraph-ids.json"), "w") as f:
        json.dump({"documents": documents}, f)
        f.write("\n")


def build_finished(app, exception):
    # The build finished hook also runs when an exception is raised.
    if exception is not None:
        return

    with sphinx.util.progress_message("dumping paragraph ids"):
        write_paragraph_ids(app)


def setup(app):
    app.connect("build-finished", build_finished)
    app.add_env_collector(SectionsCollector)


class SectionsCollector(EnvironmentCollector):
    def clear_doc(self, app, env, docname):
        """
        This is called by Sphinx during incremental builds, either when a
        document was removed or when the document has been changed. In the
        latter case, process_doc is called after this method.
        """
        if not hasattr(env, "spec_sections"):
            env.spec_sections = []
        env.spec_sections = [s for s in env.spec_sections if s.document != docname]

    def merge_other(self, app, env, docnames, other):
        """
        Sphinx supports parallel builds, with each process having its own
        environment instance, but once each document is processed those
        parallel environments need to be merged together. This method does it.
        """
        if not hasattr(env, "spec_sections"):
            env.spec_sections = []
        if not hasattr(other, "spec_sections"):
            return

        for section in other.spec_sections:
            if section.document not in docnames:
                continue
            env.spec_sections.append(section)

    def process_doc(self, app, doctree):
        """
        This method can expect no existing information about the same document
        being stored in the environment, as during incremental rebuilds the
        clear_doc method is called ahead of this one.
        """
        env = app.env
        if not hasattr(env, "spec_sections"):
            env.spec_sections = []

        for section in doctree.findall(nodes.section):
            try:
                id, anchor = utils.section_id_and_anchor(section)
            except utils.NoSectionIdError:
                continue

            title = None
            for child in section.children:
                if isinstance(child, nodes.title):
                    title = child.astext()
            if title is None:
                raise RuntimeError(f"section without title: {section}")

            env.spec_sections.append(
                Section(id=id, title=title, anchor=anchor, document=env.docname)
            )


class Section:
    def __init__(self, id, title, anchor, document):
        self.id = id
        self.title = title
        self.anchor = anchor
        self.document = document
