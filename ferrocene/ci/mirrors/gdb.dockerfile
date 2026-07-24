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
        tar \
        bzip2 \
        xz \
        patch \
        file \
        openssl11-devel \
        pkg-config \
        zlib-devel \
        python3 \
        python3-devel \
        gcc gcc-c++ binutils \
        gmp-devel

# Select gcc10 as the compiler
ENV CC="gcc10-cc"
ENV CXX="gcc10-c++"
ENV AR="gcc10-ar"
ENV LD="gcc10-ld"

COPY gdb-source.tar.xz /gdb-source.tar.xz
RUN mkdir /gdb
RUN tar -xf /gdb-source.tar.xz -C /gdb --strip-components=1
WORKDIR /gdb

RUN ./configure --with-python=/usr/bin/python3 --prefix=/gdb-install
RUN make

RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /gdb-install -cJf gdb-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .
