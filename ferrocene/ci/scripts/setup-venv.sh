#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -xeuo pipefail

if command -v uv &> /dev/null; then
    echo "uv was already installed, skipping"
    exit 0
fi

curl -LsSf https://astral.sh/uv/install.sh | sh

source $HOME/.cargo/env
echo "source $HOME/.cargo/env" >> $HOME/.bashrc

uv venv ~/.venv
if [[ "${OSTYPE}" = "msys" ]]; then
    source ~/.venv/Scripts/activate
    echo "source $HOME/.venv/Scripts/activate" >> $HOME/.bashrc
else
    source ~/.venv/bin/activate
    echo "source $HOME/.venv/bin/activate" >> $HOME/.bashrc
fi

uv pip sync requirements.txt
