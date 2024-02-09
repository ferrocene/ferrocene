# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from . import rust_changelog, upcoming_release

def setup(app):
    rust_changelog.setup(app)
    upcoming_release.setup(app)

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }

