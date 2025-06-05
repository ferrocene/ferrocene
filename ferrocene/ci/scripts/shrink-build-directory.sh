#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# This script reduces the size of the build directory, and it's meant to be
# executed before persisting the build directory into the cache.

set -euo pipefail
IFS=$'\n\t'

if [[ "${OSTYPE}" != "msys" ]]; then
    echo "build directory size:"
    du -sh build/*/* | sort -hr
fi

# Temporary files. *Should* be empty at the end of the build but let's delete
# it to be extra sure it's not persisted.
rm -rf build/tmp

# Caches of the bootstrap compiler tarballs. The bootstrap compiler is already
# extracted in `build/{target}/stage0`, so there is no advantage keeping the
# downloaded tarballs around.
rm -rf build/cache
