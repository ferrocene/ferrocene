# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM ghcr.io/rust-lang/centos:7 AS base

# CentOS 7 EOL is June 30, 2024, but the repos remain in the vault.
RUN sed -i /etc/yum.repos.d/*.repo -e 's!^mirrorlist!#mirrorlist!' \
  -e 's!^#baseurl=http://mirror.centos.org/!baseurl=https://vault.centos.org/!'
RUN sed -i 's/enabled=1/enabled=0/' /etc/yum/pluginconf.d/fastestmirror.conf

RUN yum -y install \
    git \
    make \
    wget \
    xz \
    gcc gcc-c++ binutils \
    tar \
    patch \
    bzip2 \
    file \
    openssl11-devel \
    zlib-devel 

COPY cmake-source.tar.gz /cmake-source.tar.gz

RUN mkdir /cmake
RUN mkdir /cmake-install

WORKDIR /cmake

RUN tar --strip-components=1 -C /cmake -xzf /cmake-source.tar.gz

RUN ./configure --prefix=/cmake-install
RUN make -j$(nproc)

RUN make install
RUN tar -C /cmake-install -cJf cmake-binaries.tar.xz .