#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# Checkout all the submodules in parallel.

set -euo pipefail
IFS=$'\n\t'

git submodule init
submodules="$(git config --file .gitmodules --get-regexp path | awk '{ print $2 }')"

# Spawn all `git submodule update`s in parallel and then wait for all of them
# to complete. This intentionally doesn't use the `-j` flag, as that flag
# apparently still clones the individual submodules in serial.
for submodule in ${submodules}; do
    git submodule update --depth 1 "${submodule}" &
done
wait
