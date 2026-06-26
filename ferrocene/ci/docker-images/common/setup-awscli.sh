#!/bin/sh

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

rm -rf /tmp/awscli /tmp/awscli.zip
set -xe
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
    ARCH="x86_64"
    SHA="d4bbcb5532c8bf43f9149f1b1e0d09f77f388df088656e349570637c06b76d2d"
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
    ARCH="aarch64"
    SHA="df18f41e85b1305747a51b2da3886dd17ecde7412614068ed19905d6b245c69b"
else
    echo "Unsupported platform"
    exit 1
fi
# The version defined here needs to be kept in sync with the version defined in
# ferrocene/ci/scripts/calculate-parameters.py
PACKAGE_URL="https://awscli.amazonaws.com/awscli-exe-linux-${ARCH}-2.35.11.zip"
curl -Lo /tmp/awscli.zip $PACKAGE_URL
echo "${SHA} /tmp/awscli.zip" | sha256sum -c
unzip -q -d /tmp/awscli /tmp/awscli.zip
# we want to update any install previously found
/tmp/awscli/aws/install --update

# some of our environments record all downloaded packages here
if [ test -x "/ferrocene/packages" ]; then
    echo "$SHA $PACKAGE_URL" >> /ferrocene/packages/downloads
fi