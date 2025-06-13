#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -xeuo pipefail
IFS=$'\n\t'

if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    echo "::group::Install dependencies (Windows)"
fi

# All of our Python scripts have `#!/usr/bin/env python3` as their shebang,
# but python3.exe does not exist on Windows by default. Thus we copy python.exe
# to python3.exe if the latter is not present.
setup_python3() {
    if ! command -v python3.exe >/dev/null; then
        if ! command -v python.exe >/dev/null; then
            echo "error: could not find python.exe on the system"
            exit 1
        fi

        python="$(command -v python.exe)"
        cp "${python}" "$(dirname "${python}")/python3.exe"
    fi
}

if [[ "${OSTYPE}" != "msys" ]]; then
    echo "error: the script can only run on Windows (under MSYS)"
    exit 1
fi

setup_python3

REALTIME_MONITORING_ENABLED=$(powershell -Command "(Get-MpPreference).DisableRealtimeMonitoring")
if [[ "${REALTIME_MONITORING_ENABLED}" == "False" ]]; then
    echo "Disabling Windows Defender Realtime Protection for performance reasons"
    powershell -Command "Set-MpPreference -DisableRealtimeMonitoring 1"
else
    echo "Windows Defender Realtime Protection already disabled"
fi

# Use `cmake.portable` to ensure it is added to path and because the virtual package
# was previously broken intermittently.
choco install --quiet --yes cmake.portable ninja gcc-arm-embedded llvm
choco install --quiet --yes zstandard --version=1.5.6 # 1.5.7 was reporting a mismatched SHA

if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    echo "::endgroup::"
fi