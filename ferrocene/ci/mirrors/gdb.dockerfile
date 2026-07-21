# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM amazonlinux:2

RUN yum -y install git make wget xz gcc gcc-c++ gcc10 gcc10-c++ tar patch bzip2 file openssl11-devel pkg-config zlib-devel python3

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
