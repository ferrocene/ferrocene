#!/bin/bash

set -ex

# Only run the stage 1 tests on merges, not on PR CI jobs.
if [[ -z "${PR_CI_JOB}" ]]; then
    ../x.py --stage 1 test --skip src/tools/tidy

    # Run the `mir-opt` tests again but this time for a 32-bit target.
    # This enforces that tests using `// EMIT_MIR_FOR_EACH_BIT_WIDTH` have
    # both 32-bit and 64-bit outputs updated by the PR author, before
    # the PR is approved and tested for merging.
    # It will also detect tests lacking `// EMIT_MIR_FOR_EACH_BIT_WIDTH`,
    # despite having different output on 32-bit vs 64-bit targets.
    ../x.py --stage 1 test tests/mir-opt --host='' --target=i686-unknown-linux-gnu

    # Run `ui-fulldeps` in `--stage=1`, which actually uses the stage0
    # compiler, and is sensitive to the addition of new flags.
    ../x.py --stage 1 test tests/ui-fulldeps

    # The tests are run a second time with the size optimizations enabled.
    ../x.py --stage 1 test library/std library/alloc library/core \
        --rustc-args "--cfg feature=\"optimize_for_size\""
fi

# NOTE: intentionally uses all of `x.py`, `x`, and `x.ps1` to make sure they all work on Linux.
../x.py --stage 2 test --skip src/tools/tidy

# Run the `mir-opt` tests again but this time for a 32-bit target.
# This enforces that tests using `// EMIT_MIR_FOR_EACH_BIT_WIDTH` have
# both 32-bit and 64-bit outputs updated by the PR author, before
# the PR is approved and tested for merging.
# It will also detect tests lacking `// EMIT_MIR_FOR_EACH_BIT_WIDTH`,
# despite having different output on 32-bit vs 64-bit targets.
../x --stage 2 test tests/mir-opt --host='' --target=i686-unknown-linux-gnu

# Run the UI test suite again, but in `--pass=check` mode
#
# This is intended to make sure that both `--pass=check` continues to
# work.
../x.ps1 --stage 2 test tests/ui --pass=check --host='' --target=i686-unknown-linux-gnu
