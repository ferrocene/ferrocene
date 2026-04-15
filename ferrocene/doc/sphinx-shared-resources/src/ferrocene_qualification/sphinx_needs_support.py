# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers


def setup(app):
    app.add_config_value("ferrocene_external_needs", None, "env", str)
