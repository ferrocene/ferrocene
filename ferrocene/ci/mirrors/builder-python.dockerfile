# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-builder AS base

ARG TARGETPLATFORM
ARG PYTHON_VERSION

RUN mkdir -p /opt/python/${PYTHON_VERSION}
ADD python.tar.xz /opt/python/${PYTHON_VERSION}/
ENV PATH=/opt/python/${PYTHON_VERSION}/bin:$PATH

RUN <<EOT
    curl -Lo /meson.tar.gz https://github.com/mesonbuild/meson/releases/download/1.11.2/meson-1.11.2.tar.gz
    mkdir -p /meson-src
    cd /meson-src
    tar -xf /meson.tar.gz --strip-components=1
    python3 setup.py install
    cd /
    rm -rf /meson-src
EOT