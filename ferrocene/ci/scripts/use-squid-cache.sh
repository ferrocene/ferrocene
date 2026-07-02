#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -v

export http_proxy="http://squid.ci-squid-cache.svc.cluster.local:3148"
export https_proxy="https://squid.ci-squid-cache.svc.cluster.local:4148"
