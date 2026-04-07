#!/usr/bin/env bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -xeuo pipefail

parent=$(cd $(dirname $0) && pwd)
source "$parent/../../../src/ci/shared.sh"
retry curl --location --silent --show-error --fail --output /tmp/mergiraf.tar.gz \
    https://codeberg.org/mergiraf/mergiraf/releases/download/v0.16.3/mergiraf_x86_64-unknown-linux-gnu.tar.gz
echo "91daeb9925d616d19a1784fcb973514ee8ea70ef830baa49aed168ab5ad09b77 /tmp/mergiraf.tar.gz" | sha256sum -c
# extract the binary to `~/.local/bin` (will been added to `PATH` below)
tar xf /tmp/mergiraf.tar.gz -C "$HOME/.local/bin"
rm /tmp/mergiraf.tar.gz

export PATH="$HOME/.local/bin:$PATH"

set +e
if [ ! -z ${BASH_ENV+x} ]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> $BASH_ENV
fi
set -e
