# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-builder AS base

ARG TARGETPLATFORM

RUN yum -y install \
        gmp-devel

COPY python.tar.xz /python.tar.xz
RUN mkdir -p /opt/python
RUN tar -xf /python.tar.xz -C /opt/python --strip-components=1
ENV PATH=/opt/python/bin:$PATH

COPY gdb-source.tar.xz /gdb-source.tar.xz
RUN mkdir /gdb
RUN tar -xf /gdb-source.tar.xz -C /gdb --strip-components=1
WORKDIR /gdb

RUN ./configure --with-python=/usr/bin/python3 --prefix=/gdb-install
RUN make

RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /gdb-install -cJf gdb-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .
