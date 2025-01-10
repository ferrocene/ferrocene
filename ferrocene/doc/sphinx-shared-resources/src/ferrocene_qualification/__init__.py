# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from . import (
    document_id,
    domain,
    intersphinx_support,
    signature_page,
    sphinx_needs_support,
    substitutions,
    target,
)
import string


def setup(app):
    substitutions.setup(app)
    document_id.setup(app)
    domain.setup(app)
    signature_page.setup(app)
    target.setup(app)
    intersphinx_support.setup(app)
    sphinx_needs_support.setup(app)

    app.connect("config-inited", validate_config)
    app.connect("config-inited", inject_version)
    app.add_config_value("ferrocene_id", None, "env", [str])
    app.add_config_value("ferrocene_substitutions_path", None, "env", [str])
    app.add_config_value("ferrocene_target_names_path", None, "env", [str])
    app.add_config_value("ferrocene_signature", None, "env", [str])
    app.add_config_value("ferrocene_private_signature_files_dir", None, "env", [str])
    app.add_config_value("rustfmt_version", None, "env", [str])
    app.add_config_value("ferrocene_version", None, "env", [str])
    app.add_config_value("rust_version", None, "env", [str])

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


def inject_version(app, config):
    # sphinx-needs requires the document version to be configured in order for
    # external needs to be loaded. Dynamically set it to the Ferrocene version
    # if there is no existing version.
    if not config.version and config.ferrocene_version:
        config.version = config.ferrocene_version
