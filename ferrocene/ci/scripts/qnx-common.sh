#!/usr/bin/env sh

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
    sudo "${qnxdir}"/host/common/mkqnximage/qemu/net.sh /usr/lib/qemu/qemu-bridge-helper /etc/qemu/bridge.conf
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
