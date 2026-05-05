#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

. "$(dirname $0)"/qnx-common.sh

nto_target=x86_64-pc-nto-qnx710
vm_hostname=x86_64-qnx-vm

start_vm() {
    qnx7_set_up_bridge_network

    echo
    echo "===> starting QEMU"
    check_no_other_emulator_is_running
    check_ip_address_is_free

    qemu-system-x86_64 \
        -smp 2 \
        -m 1G \
        -drive file="${emulatordir}"/disk-qemu.vmdk,if=ide,id=drv0 \
        -netdev bridge,br=br0,id=net0 -device e1000,netdev=net0 \
        -pidfile "${emulatordir}"/qemu.pid \
        -nographic \
        -kernel "${emulatordir}"/ifs.bin \
        -object rng-random,filename=/dev/urandom,id=rng0 \
        -device virtio-rng-pci,rng=rng0 \
        -serial mon:stdio
}

cmd_prepare() {
    check_emulatordir_exists

    echo
    echo "===> building remote-test-server"
    # a stage $N tool is built using a stage $(N-1) compiler
    # when cross compiling a stage0 compiler cannot be used as it does not
    # include cross-compiled stage0 libraries
    # in that case, bootstrap bumps the stage by one so that the minimum is stage1
    # what that means for this command is that both `--stage 1` and `--stage 2` produce the same
    # outcome; remote-test-server will be built using a stage1 compiler
    # see https://github.com/ferrocene/ferrocene/blob/2118a6927233a45998e02c1fc341beb21a15c1b6/src/bootstrap/src/core/build_steps/tool.rs#L332
    #
    # also this is a testing tool, not the code under test so the stage used is not critical so long
    # the tool adheres to the remote-test protocol
    ./x build src/tools/remote-test-server --stage 2 --target "${nto_target}"

    echo
    echo "===> creating initial IFS"
    tmpdir="$(mktemp -d)"
    pushd "$tmpdir"
    # NOTE `--data-size=5000 --data-inodes=40000` are required to make the test `std::fs::tests::read_large_dir` pass
    QNX_TARGET="${qnxdir}/target/qnx7" mkqnximage \
        --data-size=5000 --data-inodes=40000 --noprompt \
        --hostname="${vm_hostname}" --type=qemu --arch=x86_64 \
        --ip="${vm_ipv4_addr}" --ssh-ident=none

    echo
    echo "===> re-building a custom IFS that includes remote-test-server"
    # as per https://www.qnx.com/support/knowledgebase.html?id=5015Y000001gM7T
    # the ifs.build script needs to include the libpci.so.2.3 file in the IFS
    # but the stock version does not so patch it and then re-generate ifs.bin
    local ifsbuild=output/build/ifs.build
    grep 'lib/libpci.so.2.3' "${ifsbuild}" || sed -i 's|lib/libpci.so|lib/libpci.so\
lib/libpci.so.2.3|' "${ifsbuild}"

    # add remote-test-servere binary to IFS
    cp "$basedir"/build/host/"stage2-tools"/"${nto_target}"/release/remote-test-server "${tmpdir}"/output/build/
    echo 'remote-test-server=output/build/remote-test-server' >> "${ifsbuild}"

    # run remote-test-server once startup is complete
    local startup=output/build/startup.sh
    # UI tests and libstd tests will try to resolve 'localhost'
    echo 'grep -q localhost /etc/hosts || echo "127.0.0.1 localhost" >> /etc/hosts' >> "${startup}"
    echo 'RUST_TEST_THREADS=1 remote-test-server -v --bind 0.0.0.0:12345 --sequential' >> "${startup}"

    rm output/ifs.bin
    mkifs "${ifsbuild}" output/ifs.bin

    cp output/{disk-qemu{,.vmdk},ifs.bin} "${emulatordir}/"
    popd
    rm -rf "${tmpdir}"
}

cmd_run() {
    if ! [[ -f "${emulatordir}/ifs.bin" ]]; then
        echo "error: preparation is needed before running the emulator; run:"
        echo
        echo "    $0 prepare"
        echo
        exit 1
    fi

    start_vm
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
