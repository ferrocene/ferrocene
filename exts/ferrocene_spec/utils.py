# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

from docutils import nodes


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
