# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

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
                    "bug: the toctree Sphinx directive used by appendix emitted "
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
# finishes we go over the sections and replace the first number of appendices
# with a letter, and we offset everything based on the previous toctrees (to
# support multiple toctrees in a site).
#
# Doing this as part of TocTreeCollector ensures the rest of the build always
# sees the correct ID for sections, as no Sphinx code runs between the upstream
# number assigning and us modifying them.
class TocTreeCollectorWithAppendices(TocTreeCollector):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.__within_appendices = False
        self.__chapter_offset = 0
        self.__appendix_offset = 0
        self.__chapter_max = 0
        self.__appendix_max = 0

    def assign_section_numbers(self, env):
        result = super().assign_section_numbers(env)

        for docname in env.numbered_toctrees:
            doctree = env.get_doctree(docname)
            for toctree in doctree.findall(sphinxnodes.toctree):
                self.__replace_toctree(env, toctree)

                self.__chapter_offset = self.__chapter_max
                self.__appendix_offset = self.__appendix_max

        return result

    def __replace_toctree(self, env, toctree):
        self.__within_appendices = "are_appendices" in toctree
        for _, ref in toctree["entries"]:
            env.titles[ref]["secnumber"] = self.__renumber(env.titles[ref]["secnumber"])
            if ref in env.tocs:
                self.__replace_toc(env, ref, env.tocs[ref])

    def __replace_toc(self, env, ref, node):
        if isinstance(node, nodes.reference):
            fixed_number = self.__renumber(node["secnumber"])
            node["secnumber"] = fixed_number
            env.toc_secnumbers[ref][node["anchorname"]] = fixed_number

        elif isinstance(node, sphinxnodes.toctree):
            raise RuntimeError("nested toctrees are not supported")

        else:
            for child in node.children:
                self.__replace_toc(env, ref, child)

    def __renumber(self, number):
        if not number:
            return number

        if self.__within_appendices:
            with_offset = self.__appendix_offset + number[0]
            if with_offset > 26:
                raise RuntimeError("more than 26 appendices are not supported")

            fixed = chr(ord("A") - 1 + with_offset)
            self.__appendix_max = max(self.__appendix_max, with_offset)
        else:
            fixed = self.__chapter_offset + number[0]
            self.__chapter_max = max(self.__chapter_max, fixed)

        return (fixed, *number[1:])


# This extension needs to replace the builtin TocTreeCollector, so it's safer
# to disable it. That'll avoid two TocTreeCollectors running in the build.
def disable_builtin_toctree_collector(app):
    for obj in gc.get_objects():
        if not isinstance(obj, TocTreeCollector):
            continue
        # When running sphinx-autobuild, this function might be called multiple
        # times. When the collector is already disabled `listener_ids` will be
        # `None`, and thus we don't need to disable it again.
        #
        # Note that disabling an already disabled collector will fail.
        if obj.listener_ids is None:
            continue
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
