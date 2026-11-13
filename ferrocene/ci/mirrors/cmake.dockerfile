# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-builder AS base

COPY cmake-source.tar.gz /cmake-source.tar.gz

RUN mkdir /cmake
RUN mkdir /cmake-install

WORKDIR /cmake

RUN tar --strip-components=1 -C /cmake -xzf /cmake-source.tar.gz

RUN ./configure --prefix=/cmake-install
RUN make -j$(nproc)

RUN make install
RUN tar -C /cmake-install -cJf cmake-binaries.tar.xz .