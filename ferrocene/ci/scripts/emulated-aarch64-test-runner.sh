#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

emulatordir=/tmp/emulator

AARCH64_TARGET=aarch64-unknown-ferrocenecoretest

cmd_prepare() {
    if ! [[ -d "${emulatordir}" ]]; then
        echo "error: directory ${emulatordir} does not exist"
        echo
        exit 1
    fi

    if ! command -v qemu-aarch64-static >/dev/null 2>&1; then
        echo "error: missing qemu-aarch64-static binary"
        echo "help: on Ubuntu, install the qemu-user-static package"
        exit 1
    fi

    if ! command -v update-binfmts >/dev/null 2>&1; then
        echo "error: missing update-binfmts binary"
        echo "help: on Ubuntu, install the binfmt-support package"
        exit 1
    fi

    echo "===> building remote-test-server"
    stage="${REMOTE_TEST_SERVER_STAGE-0}"
    ./x build src/tools/remote-test-server --target $AARCH64_TARGET --stage "${stage}"
    cp "build/host/stage${stage}-tools/${AARCH64_TARGET}/release/remote-test-server" $emulatordir
}

cmd_run() {
    if ! [[ -f $emulatordir/remote-test-server ]]; then
        echo "error: preparation is needed before running the emulator; run:"
        echo
        echo "    $0 prepare"
        echo
        exit 1
    fi

    # While on normal Ubuntu installations this command is executed at boot, on
    # Docker containers there is no systemd executing it. To make sure the
    # emulator starts up properly, run the enable command every time.
    #
    # Since Ubuntu 20.04, imported binfmts need to be manually enabled after
    # the import if /proc/sys/fs/binfmt_misc is not mounted (like in CI).
    echo "===> loading binfmt configuration into the kernel"
    sudo update-binfmts --import
    sudo update-binfmts --enable qemu-aarch64

    export QEMU_CPU=cortex-a53
    qemu-aarch64-static $emulatordir/remote-test-server -v --bind 127.0.0.1:12345
}

if [[ $# -eq 1 ]] && [[ "$1" = "prepare" ]]; then
    cmd_prepare
elif [[ $# -eq 0 ]]; then
    cmd_run
else
    echo "usage: $0"
    echo"    or: $0 prepare"
    exit 1
fi
