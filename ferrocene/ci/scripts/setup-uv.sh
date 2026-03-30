#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -xeuo pipefail

parent=$(cd $(dirname $0) && pwd)
source "$parent/../../../src/ci/shared.sh"
retry curl --location --silent --show-error --fail --remote-name https://astral.sh/uv/0.11.2/install.sh
retry sh install.sh
rm install.sh

export PATH="$HOME/.local/bin:$PATH"

set +e
if [ ! -z ${BASH_ENV+x} ]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> $BASH_ENV
fi
set -e

uv python install 3.12
uv python pin 3.12
