#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import jwt
import os
import requests
import time


def authenticate_session(app_id, private_key):
    # Move the issuing time to the past to allow for a bit of clock drift
    # between the current machine and GitHub's servers.
    now = int(time.time()) - 30
    payload = {
        "iat": now,
        "exp": now + 60 * 10,
        "iss": app_id,
    }
    token = jwt.encode(payload, private_key, algorithm="RS256")
    # PyJWT < 2.0 returns bytes instead of strings.
    if type(token) == bytes:
        token = token.decode("utf-8")

    session = requests.Session()
    session.headers["Authorization"] = f"Bearer {token}"
    return session


def req(session, method, url):
    resp = session.request(method, url)
    resp.raise_for_status()
    return resp.json()


if __name__ == "__main__":
    for var in ["APP_PRIVATE_KEY", "APP_ID"]:
        if var not in os.environ:
            print(f"error: missing environment variable {var}")
            exit(1)

    app_id = os.environ["APP_ID"]
    private_key = os.environ["APP_PRIVATE_KEY"]

    http = authenticate_session(app_id, private_key)
    installations = req(http, "GET", "https://api.github.com/app/installations")

    if not installations:
        print("error: the app is not installed in any organization")
        exit(1)
    elif len(installations) > 1:
        print("error: the app should only be installed in one organization")
        exit(1)

    token = req(http, "POST", installations[0]["access_tokens_url"])["token"]

    print(f"::add-mask::{token}")
    with open(os.environ["GITHUB_OUTPUT"], "a") as f:
        f.write(f"token={token}\n")
