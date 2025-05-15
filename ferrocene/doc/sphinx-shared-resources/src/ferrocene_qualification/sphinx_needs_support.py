# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import json

def setup(app):
    app.add_config_value("ferrocene_external_needs", None, "env", str)
