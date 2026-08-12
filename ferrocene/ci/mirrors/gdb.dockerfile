# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM ubuntu:20.04



RUN apt update && apt install -y xz-utils \
        gcc g++ make \
        libgmp-dev libmpfr-dev \
        libpython3-dev python3-distutils

COPY gdb-source.tar.xz /gdb-source.tar.xz
RUN mkdir /gdb
RUN tar -xf /gdb-source.tar.xz -C /gdb --strip-components=1
WORKDIR /gdb


RUN ./configure --with-python=/usr/bin/python3 --prefix=/gdb-install
RUN make
RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /gdb-install -cJf gdb-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .
