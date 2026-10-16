## Running

```
git checkout certified-json-docs
./configure --set change-id=146663 --set profile=compiler --set build.library-docs-private-items --set build.profiler=true --set ferrocene.aws-profile=ferrocene-ci
# if necessary:
# aws sso login --profile ferrocene-ci
./x test --coverage=library library/core --tests
```

This will generate an HTML report and print its path, along with an ascii report in the terminal.

By default, the report only covers `core`. To report on a different library crate, pass
`--coverage-crate` (repeat for a list of crates):

```
./x test --coverage=library library/alloc --tests --coverage-crate alloc
```

If you see "Parsing Failed", this is a known upstream bug in [llvm-profparser](https://github.com/xd009642/llvm-profparser/).
The workaround is to test fewer things (e.g. just `library/core --tests`).
Jynn is working on fixing it.
