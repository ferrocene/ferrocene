#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

# this file resides in `/ferrocene/ci/scripts/(here)`
basedir="$(realpath $(dirname $0)/../../..)"
qnxdir="$(realpath $(dirname $(which qcc))/../../../../..)"
emulatordir=/tmp/emulator
nto_target=aarch64-unknown-nto-qnx710

panic() {
    echo "error:" "${@}" >&2
    exit 1
}

on_vm() {
    echo >"${emulatordir}"/pipe.in
    echo "${@}" >"${emulatordir}"/pipe.in
    echo 'echo "===$?==="' >"${emulatordir}"/pipe.in

    while read -r line; do
        if [[ "$line" = "==="*"==="* ]]; then
            ret=$(echo "$line" | cut -d'=' -f4)

            if [ "$ret" -eq 0  ]; then
                break
            else
                panic failed to run "${@}" on VM. exit code: "$ret"
            fi
        fi

        echo "QEMU: $line"
    done <"${emulatordir}"/pipe.out
}

on_vm_nowait() {
    echo >"${emulatordir}"/pipe.in
    echo "${@}" >"${emulatordir}"/pipe.in
}

start_vm() {
    echo
    echo "===> setting up QEMU bridge network"
    sudo "${qnxdir}"/host/common/mkqnximage/qemu/net.sh /usr/lib/qemu/qemu-bridge-helper /etc/qemu/bridge.conf

    echo
    echo "===> creating virtual SD card"
    # the stock QNX rootfs is read-only
    # its `/tmp` is a symlink to `/dev/shmem` which does not support the creation of directories
    # we use a virtual SD card as an alternative to `/tmp` that does support directories
    # (remote-test-server will use `std::env::tmpdir` and place incoming test runner files there)
    sd_size=2G
    sd_img="$emulatordir"/sd.raw

    rm -f "$sd_img"
    fallocate -l $sd_size "$sd_img"

    echo
    echo "===> starting QEMU"
    if [ -f "${emulatordir}"/qemu.pid ]; then
        panic a previous instance of the emulator may already be running
    fi
    rm -f "${emulatordir}"/pipe.*
    mkfifo "${emulatordir}"/pipe.in "${emulatordir}"/pipe.out

    # NOTE(-net nic): the (real) ZCU102 has 4 NICs; only the 4th one can be used in QEMU
    # the unused NICs need to be listed in the command line invocation
    qemu-system-aarch64 \
        --accel tcg,thread=multi \
        -drive file="$sd_img",format=raw,if=sd,size=$sd_size,index=1 \
        -kernel "${emulatordir}/src/images/QNX-IFS" \
        -m 4G \
        -machine xlnx-zcu102 \
        -net nic,model=cadence_gem \
        -net nic,model=cadence_gem \
        -net nic,model=cadence_gem \
        -net nic,model=cadence_gem,netdev=net0 -netdev bridge,br=br0,id=net0 \
        -no-reboot \
        -nographic \
        -accel kvm \
        -pidfile "${emulatordir}"/qemu.pid \
        -serial pipe:"${emulatordir}"/pipe &

    while read -r line; do
        # last boot message
        if [[ "$line" = "Boot complete"* ]]; then
            break
        fi

        echo "QEMU: ${line}"
    done <"${emulatordir}"/pipe.out
}

cmd_prepare() {
    if ! [[ -d "${emulatordir}" ]]; then
        echo "error: directory ${emulatordir} does not exist"
        echo
        exit 1
    fi

    echo
    echo "===> building BSP"
    rm -rf "${emulatordir}"/src
    mkdir -p "${emulatordir}"/src
    pushd "${emulatordir}"/src
    cp "${qnxdir}"/bsp/BSP_xilinx-zynq-zcu102_br-710_be-710_SVN928034_JBN10.zip ./bsp.zip
    unzip -q ./bsp.zip -d .
    rm -f ./bsp.zip
    make >"${emulatordir}"/make-bsp.log 2>&1
    popd

    echo
    echo "===> building remote-test-server"
    stage="${REMOTE_TEST_SERVER_STAGE-1}"
    ./x build src/tools/remote-test-server --target "${nto_target}" --stage "${stage}"
    cp build/host/"stage${stage}-tools"/"${nto_target}"/release/remote-test-server "${emulatordir}"/src/install/aarch64le/sbin

    echo
    echo "===> building IFS"
    pushd "${emulatordir}"/src/images

    buildscript="${emulatordir}"/src/images/zcu102.build

    sha256sum "${buildscript}" | grep -q '^5cb44fba650dfd69d8ad00e4f2d684a8f7bb97bdbad5cd246e17097c05d99e3b ' || \
        panic "upstream file zcu102.build appears to have changed; this cmd_prepare function may need to be updated"

    # remove unused services from startup script
    line_ranges=(
        '455,460' # qconn
        '280,300' # I2C, SPI, OCM, PCI, CAN
        '272,273' # qconn
        '261,267' # USB
        '242,247' # QSPI
        '198,219' # GPIO, PCI
        '182,187' # USB
        '163,167' # SPI
        '112,130' # SPI, OCM, telnet
        '65,103' # USB, PCI, QSPI, I2C, CAN
        '9,10' # raw image compression (added back below)
    )
    for line_range in ${line_ranges[@]}; do
        sed -i "${line_range}d" "${buildscript}"
    done

    # add binaries to IFS
    sed -i '365i /bin/mkdir=mkdir\
/sbin/remote-test-server=remote-test-server\
/bin/head=head\
/bin/sh=sh\
/bin/echo=echo' "${buildscript}"

    # signal that boot is complete
    sed -i '326i echo "Boot complete"' "${buildscript}"

    # use virtual SD card as TMPDIR
    sed -i '268i export TMPDIR=/mnt/tmp' "${buildscript}"

    # hard-code a MAC address as the VM does not include one we can read at boot time
    sed -i '67s/.*/io-pkt-v6-hc -dxzynq-ultrascale mac=000A350373B1/' "${buildscript}"

    # mount virtual SD card
    sed -i '62i waitfor /dev/hd0\
mkqnx6fs -q /dev/hd0\
mount -t qnx6 /dev/hd0 /mnt\
mkdir -p /mnt/tmp' "${buildscript}"

    # change image type to ELF
    sed -i '11s/.*/[virtual=aarch64le,elf +compress] .bootstrap = {/' "${buildscript}"

    make >"${emulatordir}"/make-ifs.log 2>&1
    popd
}

cmd_run() {
    if ! [[ -f "${emulatordir}/src/install/aarch64le/sbin/remote-test-server" ]]; then
        echo "error: preparation is needed before running the emulator; run:"
        echo
        echo "    $0 prepare"
        echo
        exit 1
    fi

    start_vm

    echo
    echo "===> starting remote-test-server"
    on_vm_nowait RUST_TEST_THREADS=1 remote-test-server -v --bind 0.0.0.0:12345 --sequential

    while read -r line; do
        echo "QEMU: $line"
    done <"${emulatordir}"/pipe.out
}

if [[ $# -eq 1 ]] && [[ "$1" = "prepare" ]]; then
    cmd_prepare
elif [[ $# -eq 0 ]]; then
    cmd_run
else
    echo "usage: $0"
    echo "    or: $0 prepare"
    exit 1
fi
