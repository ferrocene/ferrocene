# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM ghcr.io/rust-lang/centos:7 AS base

# CentOS 7 EOL is June 30, 2024, but the repos remain in the vault.
RUN sed -i /etc/yum.repos.d/*.repo -e 's!^mirrorlist!#mirrorlist!' \
  -e 's!^#baseurl=http://mirror.centos.org/!baseurl=https://vault.centos.org/!'
RUN sed -i 's/enabled=1/enabled=0/' /etc/yum/pluginconf.d/fastestmirror.conf


RUN yum -y install \
    wget curl \
    xz bzip2 gzip unzip \
    binutils \
    tar \
    make patch \
    file which \
    glibc-devel


RUN <<-EOF
    set -xe

    groupadd --gid 1000 ci
    groupadd --gid 1001 ci-usergroup

    useradd --uid 1000 --gid 1000 -d /home/ci -m ci
    useradd --uid 1001 --gid 1001 ci-user
EOF