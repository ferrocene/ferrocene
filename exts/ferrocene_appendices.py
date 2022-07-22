# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from docutils import nodes
from sphinx import addnodes as sphinxnodes
from sphinx.directives.other import TocTree
from sphinx.environment.collectors.toctree import TocTreeCollector
import gc


class AppendicesDirective(TocTree):
    def run(self):
        result = super().run()

        def compat_check(condition):
            if not condition:
                raise RuntimeError(
                    "bug: the toctre Sphinx directive used by appendix emitted "
                    "unexpected nodes, please update the extension to handle that"
                )

        # We're modifying the result of Sphinx's builtin TocTree directive, so
        # ensure it contains what we expect.
        compat_check(isinstance(result, list))
        compat_check(len(result) == 1)
        compat_check(isinstance(result[0], nodes.compound))
        compat_check(len(result[0].children) == 1)
        compat_check(isinstance(result[0].children[0], sphinxnodes.toctree))

        # Mark this toctree as containing appendices, so that the environment
        # collector can distinguish it from normal toctrees.
        result[0].children[0]["are_appendices"] = True

        return result


# To ensure the minimum disruption possible, to update section numbers we're
# replacing the EnvironmentCollector responsible for assigning section numbers.
#
# We let the builtin Sphinx logic assign section numbers, and as soon as it
# finishes we go over the sections and replace the first number with a letter.
# This ensures the rest of the build always sees the correct ID for sections.
class TocTreeCollectorWithAppendices(TocTreeCollector):
    def assign_section_numbers(self, env):
        result = super().assign_section_numbers(env)

        for docname in env.numbered_toctrees:
            doctree = env.get_doctree(docname)
            for toctree in doctree.findall(sphinxnodes.toctree):
                self.__replace_toctree(env, toctree)

        return result

    def __replace_toctree(self, env, toctree):
        within_appendices = "are_appendices" in toctree
        for _, ref in toctree["entries"]:
            if within_appendices:
                env.titles[ref]["secnumber"] = make_letter(env.titles[ref]["secnumber"])
            if ref in env.tocs:
                self.__replace_toc(env, ref, env.tocs[ref], within_appendices)

    def __replace_toc(self, env, ref, node, within_appendices):
        if within_appendices and isinstance(node, nodes.reference):
            fixed_number = make_letter(node["secnumber"])
            node["secnumber"] = fixed_number
            env.toc_secnumbers[ref][node["anchorname"]] = fixed_number

        elif isinstance(node, sphinxnodes.toctree):
            self.__replace_toctree(env, node)

        else:
            for child in node.children:
                self.__replace_toc(env, ref, child, within_appendices)


def make_letter(section_number):
    if not section_number:
        return section_number
    if section_number[0] > 26:
        raise RuntimeError("more than 26 appendices are not supported")

    return (chr(ord("A") - 1 + section_number[0]), *section_number[1:])


# This extension needs to replace the builtin TocTreeCollector, so it's safer
# to disable it. That'll avoid two TocTreeCollectors running in the build.
def disable_builtin_toctree_collector(app):
    for obj in gc.get_objects():
        if isinstance(obj, TocTreeCollector):
            obj.disable(app)


def setup(app):
    app.add_directive("appendices", AppendicesDirective)

    disable_builtin_toctree_collector(app)
    app.add_env_collector(TocTreeCollectorWithAppendices)

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }
