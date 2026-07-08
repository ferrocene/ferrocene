# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# From the root of this repository, build with
# `docker build --tag facade-test-server --file ferrocene/ci/facade-test-server/Dockerfile .`

ARG BASE_IMAGE

FROM ${BASE_IMAGE} as base

USER root
# The following needs to be in sync with the x86-64-linux-dist-targets step

COPY --chmod=0755 --chown=root:ci ferrocene/ci/docker-images/common/run-testserver.sh /bin/run-testserver.sh

USER ci

CMD [ "sleep", "5000" ]