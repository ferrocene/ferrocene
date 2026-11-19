#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

parent=$(cd $(dirname $0) && pwd)
source "$parent/../../../src/ci/shared.sh"

echo "CPU"
echo "==="
if isMacOS; then
    sysctl -n machdep.cpu.brand_string
elif isWindows; then
    systeminfo
elif isLinux; then
    lscpu
else
    echo "Unknown OS"
fi

echo
echo "System memory"
echo "============="
if isMacOS; then
    vm_stat
elif isLinux; then
    free -h

    echo "Disk"
    echo "===="
    df -h
else
    echo "Unknown OS"
fi
