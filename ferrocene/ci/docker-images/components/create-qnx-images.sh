#!/bin/bash

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

## This builds a plain docker image and a containerdisk image of the linux components
## of the given qnx toolchain. Windows components are stripped out.

## This script is intended to be executed by an operator once after uploading a new QNX tarball.
## It requires elevated permissions. The script assumes that you're authenticated againt s3 with
## a suitable role. This script assumes you're authenticated against the harbor registry with
## push permissions to the ci-images and containerdisks project.

## This script is largely os-independent and relies on aws-cli and qemu-img being present (apt install -y qemu-utils)

set -xe -o pipefail

QNX_VERSION=$1

echo "Building for qnx version $QNX_VERSION"
echo "Downloading QNX tarball from s3"
aws s3 cp s3://ferrocene-ci-mirrors/manual/qnx/${QNX_VERSION}-deployment.tar.zst ${QNX_VERSION}-deployment.tar.zst

echo "Building docker image"
docker build --tag harbor.infra.ferrous-systems.net/ferrocene-images/qnx:${QNX_VERSION}-linux --file qnx.dockerfile --build-arg QNX_VERSION=${QNX_VERSION} .

echo "Building containerdisk"
echo "Creating empty raw disk"
dd if=/dev/null of=disk.raw bs=1M seek=10240
mkfs.ext2 -F disk.raw
sudo mkdir -p /tmp/qnx-imagemount
echo "Mounting raw disk"
sudo mount disk.raw /tmp/qnx-imagemount/
echo "Unpacking tarball"
sudo tar -C /tmp/qnx-imagemount/ --strip=1 --exclude "*host/win64*" --exclude "*bat"  -xf ${QNX_VERSION}-deployment.tar.zst
sudo chown -R 1000:1000 /tmp/qnx-imagemount/
echo "Unmounting raw disk"
sudo umount /tmp/qnx-imagemount
echo "Converting image to qcow2"
qemu-img convert -O qcow2 -c disk.raw disk.img
echo "Building containerdisk image"
cat <<EOF | docker build --file - --tag harbor.infra.ferrous-systems.net/containerdisks/qnx:${QNX_VERSION}-linux .
    FROM scratch
    COPY disk.img /disk/
EOF

echo "Pushing docker images"
docker push harbor.infra.ferrous-systems.net/containerdisks/qnx:${QNX_VERSION}-linux
docker push harbor.infra.ferrous-systems.net/ferrocene-images/qnx:${QNX_VERSION}-linux

echo "Cleaning up"
rm disk.raw disk.img ${QNX_VERSION}-deployment.tar.zst