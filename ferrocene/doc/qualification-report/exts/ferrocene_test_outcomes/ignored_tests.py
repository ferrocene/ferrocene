# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from .utils import RenderTable, paragraph, literal, error
from .outcomes import TEST_IGNORED_NO_REASON
from docutils import nodes
from sphinx.util.docutils import SphinxDirective
import re


_DOCTEST_RE = re.compile(r"^[a-zA-Z0-9\-_\.\/]+ - [a-zA-Z0-9_:'<>,]+ \(line [0-9]+\)$")


class IgnoredTestsDirective(SphinxDirective):
    has_content = False

    def run(self):
        outcomes = self.env.ferrocene_test_outcomes
        if outcomes is None:
            return error(
                "The ",
                literal("ferrocene.test_outcomes_dir"),
                " setting in ",
                literal("config.toml"),
                " was not set. The list of ignored tests won't be populated",
                " until you add the setting and store the relevant build metrics",
                " files in the directory.",
            )

        table = RenderTable(2)
        table.add_row(
            paragraph("Test name"),
            paragraph("Ignore reason"),
            head=True,
        )
        for suite in outcomes.suites.values():
            for test, reason in suite.ignored_tests.items():
                # Do not include ignored doctests in the ignored tests table.
                # Rustdoc treats *all* code snippets as doctests, even the ones
                # that are not meant to be valid Rust code, or Rust code at
                # all. Those snippets have to be marked as ignored, but are not
                # really tests. To avoid confusion we remove them.
                if _DOCTEST_RE.search(test) is not None:
                    continue

                if reason is TEST_IGNORED_NO_REASON:
                    reason = nodes.emphasis()
                    reason += nodes.Text("no reason provided")
                table.add_row(paragraph(literal(test)), paragraph(reason))

        if len(table.body.children):
            return [
                paragraph(
                    f"{len(table.body.children)} tests were ignored by our testing "
                    "infrastructure:"
                ),
                table.finalize(),
            ]
        else:
            note = nodes.note()
            note += paragraph("No tests were ignored as part of this qualification.")
            return [note]



def setup(app):
    app.add_directive("ignored-tests", IgnoredTestsDirective)
