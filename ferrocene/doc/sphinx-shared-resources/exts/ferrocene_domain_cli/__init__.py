# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This extension defines the "cli" domain, which contains the "program" and
# "option" directives. Those directives are used to document the CLI interface
# of the binaries we ship.
#
# Sphinx *has* builtin "program" and "option" directives, but unfortunately
# they assume that the name of the command line option is just the one that
# starts with "-" (for example "--crate-name"), but for our use case we want to
# consider "-C opt-level=<level>" as the "-C opt-level" argument.

from . import domain, traceability_ids


def setup(app):
    domain.setup(app)
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
