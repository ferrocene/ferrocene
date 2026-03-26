#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -xeuo pipefail

# stolen from src/ci/shared.sh
function retry {
  echo "Attempting with retry:" "$@"
  local n=1
  local max=5
  while true; do
    "$@" && break || {
      if [[ $n -lt $max ]]; then
        sleep $n  # don't retry immediately
        ((n++))
        echo "Command failed. Attempt $n/$max:"
      else
        echo "The command has failed after $n attempts."
        return 1
      fi
    }
  done
}

retry curl --location --silent --show-error --fail --remote-name https://astral.sh/uv/0.9.26/install.sh
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
