## Running

```
(
    cd ~/src/ferrocene
    git checkout certified-json-docs
    ./configure --set change-id=146663 --set profile=compiler --set build.library-docs-private-items --set build.profiler=true --set ferrocene.aws-profile=ferrocene-ci
    # if necessary:
    # aws sso login --profile ferrocene-ci
    ./x test --coverage=library library/core library/std --no-doc
)

./run.nu --ferrocene-src ~/src/ferrocene
```

See https://www.nushell.sh/book/installation.html for instructions on how to install nushell.

If you see "Parsing Failed", this is a known upstream bug in [llvm-profparser](https://github.com/xd009642/llvm-profparser/).
The workaround is to test fewer things (e.g. just `library/core --no-doc`).
Jynn is working on fixing it.
