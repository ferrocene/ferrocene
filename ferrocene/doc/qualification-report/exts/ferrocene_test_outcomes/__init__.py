# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from . import ignored_tests, suite_summary, outcomes

def setup(app):
    ignored_tests.setup(app)
    suite_summary.setup(app)
    outcomes.setup(app)

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }
