#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -xeuo pipefail

if ! command -v uv &> /dev/null; then
    curl -LsSf https://astral.sh/uv/0.4.3/install.sh | sh
fi

if [[ -d $HOME/.cargo ]]; then
    source $HOME/.cargo/env
    echo "source $HOME/.cargo/env" >> $BASH_ENV
fi

uv venv ~/.venv

parent=$(cd $(dirname $0) && pwd)
source "$parent/../../../src/ci/shared.sh"

if isWindows; then
    source ~/.venv/Scripts/activate
    echo "source $HOME/.venv/Scripts/activate" >> $BASH_ENV
else
    source ~/.venv/bin/activate
    echo "source $HOME/.venv/bin/activate" >> $BASH_ENV
fi

uv pip sync requirements.txt
