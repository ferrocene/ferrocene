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
echo "source $HOME/.cargo/env" >> ~/.bashrc
echo "PATH=$HOME/.cargo/bin:$HOME/.venv/bin:$PATH" >> $GITHUB_ENV

uv venv ~/.venv
export VIRTUAL_ENV=$HOME/.venv
echo "VIRTUAL_ENV=$HOME/.venv" >> ~/.bashrc
echo "VIRTUAL_ENV=$HOME/.venv" >> $GITHUB_ENV

uv pip sync requirements.txt
