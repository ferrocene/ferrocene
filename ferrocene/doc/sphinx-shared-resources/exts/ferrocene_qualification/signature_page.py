# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

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
        self.copiable_files = {}
        self.context = {}
        self.private_files = {}

        if app.config.ferrocene_signature is None:
            self.state = "not_needed"
            return
        elif app.config.ferrocene_signature == "missing":
            self.state = "unsigned"
            return
        elif app.config.ferrocene_signature != "present":
            raise RuntimeError("invalid value to ferrocene_signature")

        try:
            self.context["config"] = tomli.loads(self.load_file("config.toml"))
            self.private_files = tomli.loads(self.load_file("signature.toml"))["files"]
            self.load_private_file("pinned.toml", copy=True)

            self.context["signatures"] = {}
            for role in self.context["config"]["roles"]:
                try:
                    bundle = json.loads(
                        self.load_private_file(f"{role}.cosign-bundle", copy=True)
                    )
                    time = datetime.datetime.utcfromtimestamp(
                        bundle["rekorBundle"]["Payload"]["integratedTime"]
                    ).strftime("%Y-%m-%d %H:%M:%S UTC")
                    present = True
                except FileNotFoundError:
                    time = "-"
                    present = False
                self.context["signatures"][role] = {
                    "time": time,
                    "present": present,
                }

            self.state = "signed"
        except FileNotFoundError:
            self.state = "inconsistent"

    def load_file(self, name, *, copy=False):
        path = f"{self.app.srcdir}/../signature/{name}"
        self.loaded_files.append(path)
        if copy:
            self.copiable_files[name] = path
        with open(path, "r", encoding="utf-8") as f:
            return f.read()

    def load_private_file(self, name, *, copy=False):
        try:
            uuid = self.private_files[name]
        except KeyError:
            raise FileNotFoundError(f"private signature file {name}")
        path = f"{self.app.config.ferrocene_private_signature_files_dir}/{uuid}"
        if copy:
            self.copiable_files[name] = path
        with open(path, "r", encoding="utf-8") as f:
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
    if signature.state != "not_needed":
        yield (
            "signature/index",
            {"state": signature.state, "signature": signature.context},
            f"{os.path.dirname(__file__)}/signature_page/template.html",
        )


def html_page_context(app, pagename, templatename, context, doctree):
    # Add a variable to all Sphinx templates on whether the document is signed.
    # This is used by the breadcrumbs template to decide whether to include the
    # link to the signature page or not.
    context["ferrocene_signature"] = app.config.ferrocene_signature


def build_finished(app, exception):
    if exception is not None:
        return

    if app.builder.name != "html":
        return

    with sphinx.util.display.progress_message("copying signature files"):
        os.makedirs(f"{app.outdir}/signature", exist_ok=True)
        signature = SignatureStatus(app)
        for name, path in signature.copiable_files.items():
            try:
                shutil.copyfile(path, f"{app.outdir}/signature/{name}")
            except FileNotFoundError:
                pass

        # The JS file is written here instead of calling app.add_js_file
        # because that method places the file in the _static/ output directory,
        # while we want it in the signature/ output directory.
        with open(f"{os.path.dirname(__file__)}/signature_page/breadcrumbs.js") as f:
            breadcrumbs_js = f.read().replace(
                "%%%SIGNED%%%", "true" if signature.state == "signed" else "false"
            )
        with open(f"{app.outdir}/signature/breadcrumbs.js", "w") as f:
            f.write(breadcrumbs_js)

        # Only include signatures in the qualification subset of tarballs.
        with open(f"{app.outdir}/signature/ferrocene-subset", "w") as f:
            f.write("signatures\n")


def setup(app):
    app.connect("doctree-read", doctree_read)
    app.connect("html-collect-pages", html_collect_pages)
    app.connect("html-page-context", html_page_context)
    app.connect("build-finished", build_finished)
