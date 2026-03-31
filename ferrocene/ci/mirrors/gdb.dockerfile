# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM ubuntu:20.04

ENV OUT=gdb_12.1.orig.tar.xz
ENV PACKAGE_URL=https://launchpad.net/ubuntu/+archive/primary/+sourcefiles/gdb/12.1-0ubuntu1~22.04.2/$OUT
ENV SHA=0e1793bf8f2b54d53f46dea84ccfd446f48f81b297b28c4f7fc017b818d69fed

RUN apt update && apt install -y curl
RUN curl -LO $PACKAGE_URL
RUN echo "$SHA $OUT" | sha256sum -c

RUN apt update && apt install -y xz-utils
RUN tar -xf $OUT
WORKDIR gdb-12.1

RUN apt update && apt install -y \
        gcc g++ \
        libgmp-dev libmpfr-dev \
        libpython3-dev python3-distutils
RUN ./configure --with-python=/usr/bin/python3 --prefix=/gdb-install

RUN apt update && apt install -y make
RUN make
RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /gdb-install -cJf gdb-12.1.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .
