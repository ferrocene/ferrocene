# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from pathlib import Path


def setup(app):
    app.add_html_theme("ferrocene", Path(__file__).resolve().parent)

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }
