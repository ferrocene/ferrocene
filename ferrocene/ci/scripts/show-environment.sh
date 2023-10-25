#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

# The two commands are not supported on Windows
if [[ "${OSTYPE}" != "msys" ]]; then
    echo "CPU"
    echo "==="
    lscpu

    echo
    echo "System memory"
    echo "============="
    free -h
fi

echo "Disk"
echo "===="
df -h
