# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from .utils import RenderTable, paragraph, literal, error
from docutils import nodes
from docutils.parsers.rst import directives
from sphinx.util.docutils import SphinxDirective
import sphinx


class SuiteSummaryDirective(SphinxDirective):
    has_content = True
    option_spec = {
        "only_match_root_node": directives.flag,
    }

    def run(self):
        if len(self.content) != 1:
            logger = sphinx.util.logging.getLogger(__name__)
            logger.error(
                "the suite-summary directive requires one argument",
                location=self.get_location(),
            )
            return []
        matcher = self.content[0]

        outcomes = self.env.ferrocene_test_outcomes
        if outcomes is None:
            return error(
                "The ",
                literal("ferrocene.test_outcomes_dir"),
                " setting in ",
                literal("config.toml"),
                " was not set. The test suite summary won't be populated",
                " until you add the setting and store the relevant build metrics",
                " files in the directory.",
            )

        tables = []
        for suite in outcomes.suites.values():
            for invocation in suite.invocations:
                if "only_match_root_node" in self.options:
                    if invocation.bootstrap_types[0] != matcher:
                        continue
                else:
                    if matcher not in invocation.bootstrap_types:
                        continue

                metadata = invocation.metadata
                table = RenderTable(2)
                if metadata["kind"] == "compiletest":
                    table.add_row(paragraph("Suite:"), paragraph(metadata["suite"]))
                elif metadata["kind"] == "cargo_package":
                    contents = []
                    for crate in metadata["crates"]:
                        if contents:
                            contents.append(", ")
                        contents.append(literal(crate))
                    table.add_row(paragraph("Tested crates:"), paragraph(*contents))
                table.add_row(
                    paragraph("Host compiler:"),
                    paragraph(
                        literal(metadata["host"]), f" (stage {metadata['stage']})"
                    ),
                )
                table.add_row(
                    paragraph("Target:"), paragraph(literal(metadata["target"]))
                )
                table.add_row(
                    paragraph("Total tests:"), paragraph(str(invocation.total_tests()))
                )
                table.add_row(
                    paragraph("Passed tests:"), paragraph(str(invocation.passed_tests))
                )
                table.add_row(
                    paragraph("Failed tests:"), paragraph(str(invocation.failed_tests))
                )
                table.add_row(
                    paragraph("Ignored tests:"),
                    paragraph(str(invocation.ignored_tests)),
                )

                tables.append(table.finalize())
                # FIXME: hacky way to add spacing since we can't change the theme
                # anymore for 1.68.
                tables.append(paragraph())

        if len(tables):
            return tables
        else:
            warning = nodes.warning()
            warning += paragraph(
                "No tests matching ", literal(matcher), " were executed."
            )
            return [warning]


def setup(app):
    app.add_directive("suite-summary", SuiteSummaryDirective)
