# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

import sphinx
import typing
from . import alphabetical_section_titles


def run_lints(app, env):
    alphabetical_section_titles.check(app, raise_error)


def raise_error(message, *, location=None):
    logger = sphinx.util.logging.getLogger(__name__)
    logger.warning(message, location=location)


def setup(app):
    app.connect("env-check-consistency", run_lints)

    app.add_config_value("lint_alphabetical_section_titles", [], "", typing.List[str])

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }
