#!/bin/sh

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -eu

BLESS=
EXTENDED=
if [ "${1:-}" = bless ]; then
    BLESS=--bless
    shift
fi

if [ "${1:-}" = extended ]; then
    EXTENDED=1
    shift
fi

cd "$(git rev-parse --show-toplevel)"

set -x

ferrocene/ci/scripts/detect-conflict-markers.py
./x run generate-help
./x run generate-completions
./x test tests/ui/ferrocene $BLESS
./x test tidy $BLESS
./x check core

if [ -n "$BLESS" ]; then
    if ! ./x test bootstrap; then
        cargo insta review --manifest-path src/bootstrap/Cargo.toml
    fi
    ./x run update-certified-core-symbols
else
    ./x test bootstrap
    ./x test certified-core-symbols
fi

./x test --coverage=library library/core library/alloc --tests

if [ -z "$EXTENDED" ]; then
    exit 0
fi

./x test $(ferrocene/ci/split-tasks.py test:compiletest) --target thumbv7em-ferrocene.facade-eabi
./x test $(ferrocene/ci/split-tasks.py test:docs)
