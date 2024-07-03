# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from sphinx import transforms


def section_id_and_anchor(section):
    if "names" in section:
        try:
            id = [name for name in section["names"] if name.startswith("fls_")][0]
        except IndexError:
            raise NoSectionIdError()
    else:
        raise NoSectionIdError()

    if section.parent is not None and isinstance(section.parent, nodes.document):
        anchor = ""
    else:
        anchor = "#" + section["ids"][0]

    return id, anchor


class NoSectionIdError(RuntimeError):
    pass


# Sphinx by default sorts all ids of the form `id[0-9]+` to the end.
# Our IDs are section name and fls_ id pairs, so in some cases this transform
# will instead sort the section name to the back, but not always!
# So we overwrite the transform instead so that our fls ids are sorted to the back.
# In addition to that we normalize them, as sphinx turns the `_` in `fls_{id}`
# into `fls-{id}` which can break the link check from working correctly.
class FlsSortIds(transforms.SphinxTransform):
    # Run this step after sphinx sorted.
    default_priority = transforms.SortIds.default_priority + 1

    def apply(self, **kwargs):
        from docutils import nodes

        for node in self.document.findall(nodes.section):
            for n, id in enumerate(node["ids"]):
                if id.startswith("fls-"):
                    node["ids"][n] = id[:3] + "_" + id[4:]
            # sort the fls id to the back
            if len(node["ids"]) > 1 and node["ids"][0].startswith("fls_"):
                node["ids"] = node["ids"][1:] + [node["ids"][0]]
