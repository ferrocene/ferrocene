#!/usr/bin/env python3

import sys
import subprocess

# This script invokes `x.py test --test-args=--edition=2021 tests/ui --bless`
# Then appends `//@ edition: 2015` too all failures to effectively ignore them
# Then it reruns `x.py test --test-args=--edition=2021 tests/ui --bless` again to fix up the
# expected outputs given the sources have changed.
# Any remaining failures need manual fixing.

if __name__ == "__main__":
    print("blessing with 2021 edition")
    result = subprocess.run(
        ["./x.py", "test", "--test-variant=2021", "tests/ui", "--bless"],
        stdout=subprocess.PIPE,
        text=True
    )
    print("ran x.py --edition=2021")
    stdout_output = result.stdout
    # Discard everything in stdout_output up to and including "failures:"
    failures_start = stdout_output.find("failures:\n    [ui]", 2)
    assert failures_start != -1, "Expected 'failures:' in stdout output"
    stdout_output = stdout_output[failures_start + len("failures:"):]
    failures_end = stdout_output.find("test result:")
    assert failures_end != -1, "Expected 'test result:' in stdout output"
    stdout_output = stdout_output[:failures_end].strip()

    failures = stdout_output.splitlines()
    print(f"found {len(failures)} failures")
    for line in failures:
        assert line.strip().startswith("[ui] "), f"Expected line to start with '[ui] ', got: {line}"
        file_path = line.strip()[len("[ui] "):]
        if "#" in file_path:
            print(f"skipping due to being a revision test: {file_path}")
            continue
        with open(file_path, 'r') as f:
            content = f.read()
        if ' edition: 20' in content or '@edition: 20' in content:
            print(f"skipping due to already having edition header: {file_path}")
            continue
        with open(file_path, 'w') as f:
            print(f"writing 2015 edition header to: {file_path}")
            f.write('//@ edition: 2015\n' + content)

    print("reblessing with 2021 edition")
    result = subprocess.run(
        ["./x.py", "test", "--test-variant=2021", "tests/ui", "--bless"],
        stdout=subprocess.PIPE,
        text=True
    )
    print("ran x.py --edition=2021")
    stdout_output = result.stdout
    # Discard everything in stdout_output up to and including "failures:"
    failures_start = stdout_output.find("failures:\n    [ui]", 2)
    assert failures_start != -1, "Expected 'failures:' in stdout output"
    stdout_output = stdout_output[failures_start + len("failures:"):]
    failures_end = stdout_output.find("test result:")
    assert failures_end != -1, "Expected 'test result:' in stdout output"
    stdout_output = stdout_output[:failures_end].strip()
    failures = stdout_output.splitlines()
    print(f"found {len(failures)} remaining failures")
    for line in failures:
        print(line.strip()[len("[ui] "):])
    sys.exit(1)
