#!/bin/sh

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -eu

BLESS=
if [ "${1:-}" = bless ]; then
    BLESS=--bless
fi

cd "$(git rev-parse --show-toplevel)"

set -x

ferrocene/ci/scripts/detect-conflict-markers.py
./x run generate-help
./x test tests/ui/ferrocene $BLESS
./x test tidy $BLESS
./x build core --set 'rust.std-features=["ferrocene_subset"]'
if [ -n "$BLESS" ]; then
    if ! ./x test bootstrap; then
        cargo insta review --manifest-path src/bootstrap/Cargo.toml
    fi
else
    ./x test bootstrap
fi
