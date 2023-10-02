#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import os
import subprocess
import sys

root = os.path.abspath(os.path.dirname(__file__))
subprocess.run(
    ["git", "submodule", "update", "--init"],
    check=True,
    cwd=root,
)

sys.path.insert(0, "shared")
import make_common  # noqa: E402

make_common.main(root)
