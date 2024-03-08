#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

UBUNTU_RELEASE="22.04.1"
UBUNTU_ARCH="arm64"
UBUNTU_SHA256="b2259205f2e94971e9e146ad1e30925d05d05e6575685cc0125b83746105ec45"

rootfs="/tmp/emulator/rootfs"

cmd_prepare() {
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

    rm -rf "${rootfs}"
    mkdir -p "${rootfs}"

    echo "===> downloading and extracting Ubuntu ${UBUNTU_RELEASE} base image"
    curl -L https://cdimage.ubuntu.com/ubuntu-base/releases/${UBUNTU_RELEASE}/release/ubuntu-base-${UBUNTU_RELEASE}-base-arm64.tar.gz -o /tmp/emulator-ubuntu-base.tar.gz
    echo "${UBUNTU_SHA256}  /tmp/emulator-ubuntu-base.tar.gz" | sha256sum -c
    tar xzf /tmp/emulator-ubuntu-base.tar.gz -C "${rootfs}"

    echo "===> configuring networking in the rootfs"
    echo "127.0.0.1 localhost" > "${rootfs}/etc/hosts"

    echo "===> copying qemu-aarch64-static in the rootfs"
    cp "$(command -v qemu-aarch64-static)" "${rootfs}/usr/bin"

    echo "===> building and copying remote-test-server into the rootfs"
    stage="${REMOTE_TEST_SERVER_STAGE-0}"
    ./x build src/tools/remote-test-server --target aarch64-unknown-linux-gnu --stage "${stage}"
    cp "build/x86_64-unknown-linux-gnu/stage${stage}-tools/aarch64-unknown-linux-gnu/release/remote-test-server" "${rootfs}/usr/bin"
}

cmd_run() {
    if ! [[ -d "${rootfs}" ]]; then
        echo "error: preparation is needed before running the emulator; run:"
        echo
        echo "    $0 prepare"
        echo
        exit 1
    fi

    echo "Starting the emulator, this will call sudo under the hood."
    echo "To configure a separate terminal to use the emulator to run tests, run on it:"
    echo
    echo "    export TEST_DEVICE_ADDR=127.0.0.1:12345"
    echo

    if ! [[ -e "${rootfs}/dev/stdout" ]]; then
        echo "===> mounting /dev into the rootfs"
        sudo mount -o bind /dev "${rootfs}/dev"
        trap cleanup_mount EXIT # Ensure the mount will be removed at exit
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

    # We pass --sequential because we've seen deadlocks when running UI tests
    # without that flag. Test execution will be slower, but at least it won't
    # lock CI up.
    echo "===> starting remote-test-server"
    sudo chroot "${rootfs}" /usr/bin/qemu-aarch64-static /usr/bin/remote-test-server -v --bind 127.0.0.1:12345 --sequential
}

cleanup_mount() {
    echo
    echo "===> unmounting /dev from the rootfs"
    sudo umount "${rootfs}/dev"
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
