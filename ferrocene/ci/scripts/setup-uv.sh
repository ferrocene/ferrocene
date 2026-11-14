#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -xeuo pipefail

parent=$(cd $(dirname $0) && pwd)
source "$parent/../../../src/ci/shared.sh"

curl -LsSf https://astral.sh/uv/0.5.0/install.sh | sh

export PATH="$HOME/.local/bin:$PATH"

set +e
if [ ! -z ${BASH_ENV+x} ]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> $BASH_ENV
fi
set -e

retry uv python install 3.12
uv python pin 3.12
