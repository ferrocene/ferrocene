# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-python-builder AS base

RUN yum -y install zlib-devel
# git now needs rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
# somehow meson will not pick this up on its own
ENV CFLAGS="-ldl"
COPY /git-source.tar.xz /git-source.tar.xz
RUN mkdir -p /git-src
WORKDIR /git-src
RUN tar -xf /git-source.tar.xz --strip-components=1
RUN meson setup _build
RUN meson compile -C _build
RUN meson install -C _build --destdir /stage
RUN tar -C /stage/usr/local/  -cJf /git-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .