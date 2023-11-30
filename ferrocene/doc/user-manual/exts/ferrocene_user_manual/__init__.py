# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from . import traceability_ids


def setup(app):
    traceability_ids.setup(app)

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
        # The version needs to be updated whenever there is a breaking change
        # in the data stored in the environment. Bumping the version number
        # will ensure Sphinx will do a clean build.
        #
        # Version history:
        # - 0: initial implementation
        "env_version": "0",
    }
