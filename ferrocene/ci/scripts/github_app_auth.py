#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# dependencies = ["pyjwt", "requests"]
# ///

import argparse
import base64
import jwt
import os
import requests
import subprocess
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


def get_token(app_id, private_key):
    http = authenticate_session(app_id, private_key)
    installations = req(http, "GET", "https://api.github.com/app/installations")

    if not installations:
        print("error: the app is not installed in any organization")
        exit(1)
    elif len(installations) > 1:
        print("error: the app should only be installed in one organization")
        exit(1)

    token = req(http, "POST", installations[0]["access_tokens_url"])["token"]

    mask_secret(token)
    return token


def set_git_credentials(origin, token):
    if not origin.startswith("https://") or not origin.endswith("/"):
        print("error: origin must be an https URL with a trailing slash")
        exit(1)

    basic = base64.b64encode(f"x-access-token:{token}".encode("utf-8")).decode("utf-8")
    mask_secret(basic)

    # Configure git to use the token for HTTPS requests to GitHub in this repository.
    subprocess.run(
        [
            "git",
            "config",
            "--local",
            f"http.{origin}.extraheader",
            f"Authorization: basic {basic}",
        ],
        check=True,
    )


def mask_secret(value):
    if "GITHUB_ACTIONS" in os.environ and os.environ["GITHUB_ACTIONS"] == True:
        print(f"::add-mask::{value}")


if __name__ == "__main__":
    for var in ["APP_PRIVATE_KEY", "APP_ID"]:
        if var not in os.environ:
            print(f"error: missing environment variable {var}")
            exit(1)

    cli = argparse.ArgumentParser()
    cli.add_argument("--set-git-credentials", action="store_true")
    args = cli.parse_args()

    app_id = os.environ["APP_ID"]
    private_key = os.environ["APP_PRIVATE_KEY"]

    token = get_token(app_id, private_key)
    with open(os.environ["GITHUB_OUTPUT"], "a") as f:
        f.write(f"token={token}\n")

    if args.set_git_credentials:
        set_git_credentials("https://github.com/", token)
