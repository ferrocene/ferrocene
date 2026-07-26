# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-builder AS base

ARG TARGETPLATFORM
ARG PYTHON_VERSION

RUN yum -y install \
        gmp-devel mpfr-devel

COPY python.tar.xz /python.tar.xz
RUN mkdir -p /opt/python/${PYTHON_VERSION}
RUN tar -xf /python.tar.xz -C /opt/python/${PYTHON_VERSION}
ENV PATH=/opt/python/${PYTHON_VERSION}/bin:$PATH

COPY gdb-source.tar.xz /gdb-source.tar.xz
RUN mkdir -p /gdb
RUN tar -xf /gdb-source.tar.xz -C /gdb --strip-components=1
WORKDIR /gdb

# readline in gdb12 is incompatible with the C21 standard
ENV CFLAGS="-g -O2 -std=gnu17"
# gdb 12 is incompatible with newer C++ standards
ENV CXXFLAGS="-g -O2 -std=gnu++17"

RUN ./configure --with-python=/opt/python/${PYTHON_VERSION}/bin/python3 --prefix=/gdb-install
RUN make

RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /gdb-install -cJf gdb-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .
