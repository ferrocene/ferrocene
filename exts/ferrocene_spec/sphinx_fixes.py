# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from docutils import nodes
from sphinx.transforms import SphinxTransform, SortIds


# Sphinx by default sorts all ids of the form `id[0-9]+` to the end.
# Our IDs are section name and fls_ id pairs, so in some cases this transform
# will instead sort the section name to the back, but not always!
# So we overwrite the transform instead so that our fls ids are sorted to the back.
# In addition to that we normalize them, as sphinx turns the `_` in `fls_{id}`
# into `fls-{id}` which can break the link check from working correctly.
class FerroceneSortIds(SphinxTransform):
    # Run this step after sphinx sorted.
    default_priority = SortIds.default_priority + 1

    def apply(self):
        for node in self.document.findall(nodes.section):
            for n, id in enumerate(node["ids"]):
                node["ids"][n] = normalize_fls_id(id)
            # sort the fls id to the back
            if len(node["ids"]) > 1 and node["ids"][0].startswith("fls_"):
                node["ids"] = node["ids"][1:] + [node["ids"][0]]


class NormalizeFlsReferences(SphinxTransform):
    default_priority = 500

    def apply(self):
        for reference in self.document.findall(nodes.reference):
            if "internal" not in reference or not reference["internal"]:
                continue
            if "refuri" not in reference or "#" not in reference["refuri"]:
                continue
            path, hash = reference["refuri"].rsplit("#", 1)
            reference["refuri"] = f"{path}#{normalize_fls_id(hash)}"


def normalize_fls_id(id):
    if id.startswith("fls-"):
        return id.replace("fls-", "fls_", 1)
    else:
        return id


def setup(app):
    app.add_transform(FerroceneSortIds)
    app.add_post_transform(NormalizeFlsReferences)
