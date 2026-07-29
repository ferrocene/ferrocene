# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build

RUN yum -y install perl-core

ADD gcc.tar.xz /opt/local/gcc
ADD binutils.tar.xz /opt/local/

ENV PATH=/opt/local/gcc/bin:/opt/local/bin:$PATH

COPY openssl.tar.gz /openssl.tar.gz
RUN mkdir -p /openssl-build
RUN tar -xf /openssl.tar.gz -C /openssl-build --strip-components=1
WORKDIR /openssl-build

RUN ./Configure --prefix=/usr/local --openssldir=/usr/local/ssl  '-Wl,-rpath,$(LIBRPATH)'
RUN make -j$(nproc)
RUN make install
# We know that /usr/local only contains our freshly built openssl
RUN tar -C /usr/local/ -cJf openssl-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .