# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-builder AS base

ARG TARGETPLATFORM
ARG PYTHON_VERSION

COPY python.tar.xz /python.tar.xz
RUN mkdir -p /opt/python/${PYTHON_VERSION}
RUN tar -xf /python.tar.xz -C /opt/python/${PYTHON_VERSION}
ENV PATH=/opt/python/${PYTHON_VERSION}/bin:$PATH