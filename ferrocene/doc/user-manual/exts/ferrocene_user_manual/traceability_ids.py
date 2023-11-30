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
        options_by_document[option.document].append(
            {
                "id": option.id(),
                "program": option.program,
                "option": option.option,
                "link": app.builder.get_target_uri(option.document) + "#" + option.id(),
            }
        )

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

    with sphinx.util.progress_message("dumping traceability ids"):
        write_traceability_ids(app)


def setup(app):
    app.connect("build-finished", build_finished)
