#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

. "$(dirname $0)"/qnx-common.sh

nto_target=aarch64-unknown-nto-qnx710_iosock
remote_ip=172.23.4.18
ci_dir=/sdcard/scratch/ferrocene-ci

export SSHPASS="qnxuser"
sshpass -e ssh qnxuser@${remote_ip} rm -rf $ci_dir
sshpass -e ssh qnxuser@${remote_ip} mkdir $ci_dir
sshpass -e scp "$basedir"/build/host/"stage2-tools"/"${nto_target}"/release/remote-test-server qnxuser@${remote_ip}:$ci_dir/
sshpass -e ssh qnxuser@${remote_ip} slay remote-test-server || true
# FIXME remove the `PATH` override when IFS is updated to include `head` command
# `cd` is used because the default current directory is in the IFS which is immutable and causes a `mkdir` test to fail
sshpass -e ssh qnxuser@${remote_ip} 'export PATH=/sdcard/bin:$PATH;'"cd ${ci_dir}; RUST_TEST_THREADS=1 TMPDIR=$ci_dir/tmp $ci_dir/remote-test-server -v --bind 0.0.0.0:12345 --sequential"

