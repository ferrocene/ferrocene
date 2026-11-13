#!/bin/false
# shellcheck shell=bash

# This file is intended to be sourced with `. shared.sh` or
# `source shared.sh`, hence the invalid shebang and not being
# marked as an executable file in git.

function hide_output {
  { set +x; } 2>/dev/null
  on_err="
echo ERROR: An error was encountered with the build.
cat /tmp/build.log
exit 1
"
  trap "$on_err" ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  PING_LOOP_PID=$!
  "$@" &> /tmp/build.log
  trap - ERR
  kill $PING_LOOP_PID
  set -x
}

# Try a command up to 7 times, with randomized exponential backoff between
# attempts.
#
# The backoff sequence is {1-5, 1-10, 1-20, ..., 1-160} seconds, allowing an
# average of 160 seconds (just over 2.5 minutes) and a worst case of 315 seconds
# (just over 5 minutes) before giving up.
#
# Due to shell arithmetic limitations, we always wait for a whole number of
# seconds between retries, but each integer in the above ranges (inclusive)
# is equally likely.
#
# See https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/
# for analysis; this follows the "Full Jitter" method mentioned there, with
# the small difference that we always wait at least 1 second between tries.
#
# Duplicated in src/ci/shared.sh
function retry {
  echo "Attempting with retry:" "$@"
  local n=1
  local base=5
  local max=7
  while true; do
    "$@" && break || {
      if [[ $n -lt $max ]]; then
        # $RANDOM returns a random integer in [0, 32767],
        # so remap to a random integer in [1, base * 2^(n - 1)]
        local delay=$(( 1 + $RANDOM * $base * 2**($n - 1) / 32768 ))
        echo "Attempt $n/$max failed. Waiting $delay second(s) before retrying."
        sleep $delay
        ((n++))
        echo "Attempt $n/$max:"
      else
        echo "The command has failed after $n attempts."
        return 1
      fi
    }
  done
}

download_tar_and_extract_into_dir() {
  local url="$1"
  local sum="$2"
  local dir="$3"
  local file=$(mktemp -u)

  while :; do
    if [[ -f "$file" ]]; then
      if ! h="$(sha256sum "$file" | awk '{ print $1 }')"; then
        printf 'ERROR: reading hash\n' >&2
        exit 1
      fi

      if [[ "$h" == "$sum" ]]; then
        break
      fi

      printf 'WARNING: hash mismatch: %s != expected %s\n' "$h" "$sum" >&2
      rm -f "$file"
    fi

    printf 'Downloading: %s\n' "$url"
    if ! curl -f -L -o "$file" "$url"; then
       rm -f "$file"
      sleep 1
    fi
  done

  mkdir -p "$dir"
  cd "$dir"
  tar -xf "$file"
  rm -f "$file"
}
