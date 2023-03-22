# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

import datetime
import json
import os
import shutil
import sphinx
import tomli


class SignatureStatus:
    def __init__(self, app):
        self.app = app
        self.loaded_files = []
        self.copiable_files = []
        self.context = {}

        if not app.config.ferrocene_signed:
            self.state = "unsigned"
            return

        try:
            self.context["config"] = tomli.loads(self.load_file("config.toml"))
            self.load_file("pinned.toml", copy=True)

            self.context["signatures"] = {}
            for role in self.context["config"]["roles"]:
                try:
                    bundle = json.loads(
                        self.load_file(f"{role}.cosign-bundle", copy=True)
                    )
                    time = datetime.datetime.utcfromtimestamp(
                        bundle["rekorBundle"]["Payload"]["integratedTime"]
                    ).strftime("%Y-%m-%d %H:%M:%S UTC")
                except FileNotFoundError:
                    time = "-"
                self.context["signatures"][role] = {
                    "time": time,
                }

            self.state = "signed"
        except FileNotFoundError:
            self.state = "inconsistent"

    def load_file(self, name, *, copy=False):
        path = f"{self.app.srcdir}/../signature/{name}"
        self.loaded_files.append(path)
        if copy:
            self.copiable_files.append(path)
        with open(path) as f:
            return f.read()


def doctree_read(app, doctree):
    # We want to regenerate the signature/index.html page whenever the
    # signature files change.
    #
    # Normally, pages returned by html_collect_pages are generated every build,
    # but if there are no changes to the RST files Sphinx will skip generation
    # altogether, preventing changes to only signature files from appearing.
    #
    # To fix this, we tell Sphinx that the index RST page depends on the
    # signature files, to ensure a rebuild is triggered when they change.
    if app.env.docname != "index":
        return

    signature = SignatureStatus(app)
    for file in signature.loaded_files:
        app.env.note_dependency(file)


def html_collect_pages(app):
    # Generate the signature/index.html file.
    signature = SignatureStatus(app)
    yield (
        "signature/index",
        {"state": signature.state, "signature": signature.context},
        os.path.join(os.path.dirname(__file__), "signature_page.html"),
    )


def html_page_context(app, pagename, templatename, context, doctree):
    # Add a variable to all Sphinx templates on whether the document is signed.
    # This is used by the breadcrumbs template to decide whether to include the
    # link to the signature page or not.
    context["ferrocene_signed"] = app.config.ferrocene_signed


def build_finished(app, exception):
    if exception is not None:
        return

    with sphinx.util.progress_message("copying signature files"):
        signature = SignatureStatus(app)
        for file in signature.copiable_files:
            name = os.path.basename(file)
            try:
                shutil.copyfile(file, f"{app.outdir}/signature/{name}")
            except FileNotFoundError:
                pass


def setup(app):
    app.connect("doctree-read", doctree_read)
    app.connect("html-collect-pages", html_collect_pages)
    app.connect("html-page-context", html_page_context)
    app.connect("build-finished", build_finished)
