# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM ubuntu:20.04 as base

RUN apt update && apt install -y git
RUN git clone https://github.com/richfelker/musl-cross-make
WORKDIR musl-cross-make
RUN git checkout fd6be58297ee21fcba89216ccd0d4aca1e3f1c5c # v0.9.11

RUN apt update && apt install -y make wget xz-utils
ENV LINUX_HEADERS_SITE="https://mirrors.2f30.org/sabotage/tarballs/"
ENV GNU_SITE="https://gcc.gnu.org/"
ENV MUSL_VER=1.2.5
RUN make extract_all

# we need to build the code as relocatable / position independent (-fPIC) or else we get
# linker errors with the x86_64 MUSL target. the `-g1` flag lets backtraces cross into C libraries
ENV CFLAGS="-fPIC -g1"
RUN apt update && apt install -y \
  gcc-aarch64-linux-gnu g++-aarch64-linux-gnu gcc-x86-64-linux-gnu g++-x86-64-linux-gnu

FROM base as aarch64

ENV TARGET=aarch64-linux-musl
RUN make

RUN make install
RUN echo "Creating $(du -hs output) archive"
RUN tar -C output -cJf musl-cross-make-aarch64.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

FROM base as x86_64

ENV TARGET=x86_64-linux-musl
RUN make

RUN make install
RUN echo "Creating $(du -hs output) archive"
RUN tar -C output -cJf musl-cross-make-x86_64.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .
COPY --from=aarch64 /musl-cross-make/musl-cross-make-aarch64.tar.xz .
