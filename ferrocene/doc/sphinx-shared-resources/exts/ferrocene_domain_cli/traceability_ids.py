# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from collections import defaultdict
import json
import os
import sphinx


def write_traceability_ids(app):
    env = app.env

    options_by_document = defaultdict(list)
    for option in env.get_domain("cli").get_options().values():
        if option.no_traceability_matrix:
            continue

        options_by_document[option.document].append(
            {
                "id": option.id(),
                "program": option.program,
                "option": option.option,
                "link": app.builder.get_target_uri(option.document) + "#" + option.id(),
            }
        )

    # Do not emit the traceability IDs if no option was defined.
    if not options_by_document:
        return

    documents = []
    for docname, title in env.titles.items():
        if docname not in options_by_document:
            continue

        documents.append(
            {
                "title": title.astext(),
                "link": app.builder.get_target_uri(docname),
                "options": options_by_document[docname],
                "informational": False,
            }
        )

    with open(os.path.join(app.outdir, "traceability-ids.json"), "w") as f:
        json.dump({"documents": documents}, f)


def build_finished(app, exception):
    # The build finished hook also runs when an exception is raised.
    if exception is not None:
        return

    if app.builder.name != "html":
        return

    with sphinx.util.display.progress_message("dumping traceability ids"):
        write_traceability_ids(app)


def setup(app):
    app.connect("build-finished", build_finished)
