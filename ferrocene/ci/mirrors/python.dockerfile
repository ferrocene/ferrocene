# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM ghcr.io/rust-lang/centos:7 AS base

# CentOS 7 EOL is June 30, 2024, but the repos remain in the vault.
RUN sed -i /etc/yum.repos.d/*.repo -e 's!^mirrorlist!#mirrorlist!' \
  -e 's!^#baseurl=http://mirror.centos.org/!baseurl=https://vault.centos.org/!'
RUN sed -i 's/enabled=1/enabled=0/' /etc/yum/pluginconf.d/fastestmirror.conf

# OpenSSL 11 is only available via epel
RUN yum -y install epel-release

RUN yum -y install \
    git \
    make \
    gcc gcc-c++ binutils \
    wget \
    xz \
    tar \
    patch \
    bzip2 \
    file \
    openssl11 \
    openssl11-devel \
    zlib-devel


# We need a newer Python than what's included in Ubuntu 20.04, so we have to
# build it from source. Python 3.12.x has been chosen as it's the same minor
# release shipped with Ubuntu 24.04 LTS.

RUN mkdir /python
WORKDIR /python

COPY python-source.tar.xz /python-source.tar.xz

RUN tar  -C /python -xf /python-source.tar.xz --strip-components=1

RUN ./configure --enable-optimizations --prefix=/python-install
RUN make -j$(nproc)

RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /python-install -cJf python-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .