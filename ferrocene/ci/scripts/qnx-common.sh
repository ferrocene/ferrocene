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
    helperdir="$(dirname "$1")"
    # default search location when built from source
    conf1="$helperdir/../etc/qemu/bridge.conf"
    # location in Debian/Ubuntu package
    conf2="$helperdir/../../../etc/qemu/bridge.conf"
    if [ -f "$conf1" ]; then
        echo "$(readlink -f "$conf1")"
    elif [ -f "$conf2" ]; then
        echo "$(readlink -f "$conf2")"
    else
        panic "bridge.conf not found; update this search logic (helper=$1)"
    fi
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
