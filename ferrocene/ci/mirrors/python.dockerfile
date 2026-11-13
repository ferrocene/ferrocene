# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-builder AS base
ARG PYTHON_VERSION

RUN yum -y install \
    zlib-devel xz-devel bzip2-devel

# We need a newer Python than what's included in Ubuntu 20.04, so we have to
# build it from source. Python 3.12.x has been chosen as it's the same minor
# release shipped with Ubuntu 24.04 LTS.

RUN mkdir /python
WORKDIR /python

COPY python-source.tar.xz /python-source.tar.xz

RUN tar  -C /python -xf /python-source.tar.xz --strip-components=1

RUN ./configure --enable-optimizations --prefix=/opt/python/${PYTHON_VERSION}/ --with-openssl=/usr/local
RUN make -j$(nproc)

RUN make install
ENV PATH=/opt/python/${PYTHON_VERSION}/bin:$PATH
# upgrade pip
RUN pip3 install --upgrade pip
# python 3.12 no longer comes with bundled setuptools
RUN pip3 install setuptools
RUN pip3 install truststore
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /opt/python/${PYTHON_VERSION}/ -cJf python-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .