# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from . import substitutions, document_id, domain, signature_page, target
import string


def setup(app):
    substitutions.setup(app)
    document_id.setup(app)
    domain.setup(app)
    signature_page.setup(app)
    target.setup(app)

    app.connect("config-inited", validate_config)
    app.add_config_value("ferrocene_id", None, "env", [str])
    app.add_config_value("ferrocene_substitutions_path", None, "env", [str])
    app.add_config_value("ferrocene_target_names_path", None, "env", [str])
    app.add_config_value("ferrocene_signature", None, "env", [str])
    app.add_config_value("ferrocene_private_signature_files_dir", None, "env", [str])
    app.add_config_value("ferrocene_version", None, "env", [str])
    app.add_config_value("rust_version", None, "env", [str])
    app.add_config_value("channel", None, "env", [str])

    return {
        "version": "0",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }


def validate_config(app, config):
    for required in [
        "ferrocene_id",
        "ferrocene_substitutions_path",
        "ferrocene_target_names_path",
    ]:
        if config[required] is None:
            raise ValueError(f"Missing required {required} configuration")

    if any(c not in string.ascii_uppercase for c in config["ferrocene_id"]):
        raise ValueError("ferrocene_id can only be uppercase letters")
