# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build

RUN yum -y install perl-core

ADD gcc.tar.xz /opt/local/gcc
RUN tar -xf /gcc.tar.xz -C /opt/local/gcc
ADD binutils.tar.xz /opt/local/
ADD openssl.tar.xz /usr/local

ENV PATH=/opt/local/gcc/bin:/opt/local/bin:$PATH
ENV PKG_CONFIG_PATH=/usr/local/lib64/pkgconfig/

COPY openssl.tar.gz /openssl.tar.gz
RUN mkdir -p /openssl-build
RUN tar -xf /openssl.tar.gz -C /openssl-build --strip-components=1
WORKDIR /openssl-build

RUN ./Configure --prefix=/usr/local --openssldir=/usr/local/ssl  '-Wl,-rpath,$(LIBRPATH)'
RUN make
RUN make install
# We know that /usr/local only contains our freshly built openssl
RUN tar -C /usr/local/ -cJf openssl-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .