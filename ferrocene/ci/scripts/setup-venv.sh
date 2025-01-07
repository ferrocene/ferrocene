#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -xeuo pipefail

scripts_dir="$(dirname $(readlink -f "$0"))"
source "${scripts_dir}/setup-uv.sh"

uv venv
uv pip sync requirements.txt
