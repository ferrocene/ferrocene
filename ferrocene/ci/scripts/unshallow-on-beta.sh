#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# On the beta channel, rustbuild needs the whole history for the main and the
# beta branch to be available, so that it can calculate the version number for
# the beta release. On CI we're performing shallow clones to save on CI time,
# while this script unshallows the clone on the beta channel.

set -euo pipefail
IFS=$'\n\t'

if [[ "$(cat src/ci/channel)" != "beta" ]]; then
    echo "not on the beta channel, skipping unshallow"
    exit
fi

release_branch="release/$(cat src/version | awk '{split($0,n,"."); print(n[1] "." n[2])}')"
git fetch --recurse-submodules=no --unshallow origin main "${release_branch}"
