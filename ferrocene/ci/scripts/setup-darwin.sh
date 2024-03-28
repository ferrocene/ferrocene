#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Unlike Windows/Mac executors, Macs do not come with awscli by default
brew install awscli cmake ninja zstd

# There are a number of reports of very slow uploads in Mac VMs due to TSO, disable it.
# https://github.com/aws/aws-sdk/issues/469
# https://github.com/cypress-io/cypress/issues/28033#issuecomment-1879102700
# https://support.circleci.com/hc/en-us/articles/19334402064027-Troubleshooting-slow-uploads-to-S3-for-jobs-using-an-m1-macOS-resource-class
sudo sysctl net.inet.tcp.tso=0
