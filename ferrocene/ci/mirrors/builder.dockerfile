# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build

COPY gcc.tar.xz /gcc.tar.xz
RUN mkdir -p /opt/local/gcc
RUN tar -xf /gcc.tar.xz -C /opt/local/gcc
ENV PATH=/opt/local/gcc/bin:$PATH

COPY openssl.tar.xz /openssl.tar.xz
RUN tar -xf /openssl.tar.xz -C /usr/local

# add /usr/local/lib to ld's path
RUN echo '/usr/local/lib/' > /etc/ld.so.conf.d/openssl.conf
RUN echo '/usr/local/lib64' >> /etc/ld.so.conf.d/openssl.conf
RUN ldconfig