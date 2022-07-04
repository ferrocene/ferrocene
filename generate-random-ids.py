#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

# Convenience script to generate a list of random paragraph IDs, ready to be
# copy-pasted as you write new paragraphs.

import string
import random

CHARS = string.ascii_letters + string.digits
LENGTH = 12
GENERATE = 10

for _ in range(GENERATE):
    id = "".join(random.choice(CHARS) for _ in range(LENGTH))
    print(f":dp:`fls_{id}`")
