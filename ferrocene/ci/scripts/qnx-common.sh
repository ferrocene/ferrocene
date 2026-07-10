#!/usr/bin/env sh
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# this file resides in `/ferrocene/ci/scripts/(here)`
basedir="$(realpath $(dirname $0)/../../..)"
qnxdir="$(realpath $(dirname $(which qcc))/../../../../..)"
emulatordir=/tmp/emulator
# aarch64-qnx71 will get assigned an IP address at boot time, this one, if not already taken
# we'll statically assign this IP address to the other QNX VMs
vm_ipv4_addr=172.31.1.105

panic() {
    echo "error:" "${@}" >&2
    exit 1
}

qnx7_set_up_bridge_network() {
    echo
    echo "===> setting up QEMU bridge network"
    helper="$(locate_qemu_bridge_helper $1)"
    conf="$(locate_bridge_conf "$helper")"
    sudo "${qnxdir}"/host/common/mkqnximage/qemu/net.sh "$helper" "$conf"
}

qnx8_set_up_bridge_network() {
    echo
    echo "===> setting up QEMU bridge network"
    helper="$(locate_qemu_bridge_helper $1)"
    conf="$(locate_bridge_conf "$helper")"
    sudo "${qnxdir}"/host/common/mkqnximage/qemu/net.sh "$helper" "$conf" nat
}

locate_qemu_bridge_helper() {
    bindir="$(dirname "$(which qemu-system-$1)")"
    # default location when built from source
    helper1="$bindir/../libexec/qemu-bridge-helper"
    # location in Debian/Ubuntu package
    helper2="$bindir/../lib/qemu/qemu-bridge-helper"
    if [ -f "$helper1" ]; then
        echo "$(readlink -f "$helper1")"
    elif [ -f "$helper2" ]; then
        echo "$(readlink -f "$helper2")"
    else
        panic "qemu-bridge-helper not found; update this search logic (arch=$1)"
    fi
}

locate_bridge_conf() {
    qemu_bridge_helper="$1"

    # The location where `qemu-bridge-helper` expects `bride.conf` to be is
    # different on a custom QEMU build vs on the Debian packaged QEMU. Also, the
    # file is not contained in the Debian package not created by
    # `make install` on a custom build; instead the QNX SDP's `net.sh` script
    # will create it. As `bridge.conf` does not exist we cannot search for it
    # using `find`. The most reliable way to get its expected location seems to
    # be to extract the location directly from the helper binary itself
    #
    # Examples of bridge-qemu-helper and bridge.conf locations
    # - Ubuntu/Debian package:
    #   - `/usr/lib/qemu/qemu-bridge-helper`;
    #   - `/etc/qemu/bridge.conf`
    # - custom build:
    #   - `$PREFIX/libexec/qemu-bridge-helper`;
    #   - `$PREFIX/etc/qemu/bridge.conf`
    strings "$qemu_bridge_helper" | grep '^/.*/bridge.conf$'
}

check_no_other_emulator_is_running() {
    if [ -f "${emulatordir}"/qemu.pid ]; then
        panic a previous instance of the emulator may already be running
    fi
}

check_ip_address_is_free() {
     ping -w1 -c1 "${vm_ipv4_addr}" && panic "there's a node already active at the IP address (${vm_ipv4_addr}) we want to use" || true
}

check_emulatordir_exists() {
    if ! [[ -d "${emulatordir}" ]]; then
        panic "error: directory ${emulatordir} does not exist"
    fi
}
