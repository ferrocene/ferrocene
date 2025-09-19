## Running

```
(
    cd ~/src/ferrocene
    ./configure --set build.library-docs-private-items --set build.profiler=true
    ./x doc --json library/core
    ./x test --coverage=library library/core --no-doc
)

cargo run --release -- show --instr-profile build/tmp/ferrocene-library-aarch64-apple-darwin.profdata \
  --object build/host/stage1-std/aarch64-apple-darwin/release/deps/libstd-5b1775e8332543d5.dylib \
  --object build/host/stage1-std/aarch64-apple-darwin/release/deps/corebenches-f57cc667ccd8ee9b \
  --object build/host/stage1-std/aarch64-apple-darwin/release/deps/coretests-994a1f24809b5d33 \
  --rustdoc-json build/host/json-doc/core.json \
  --ferrocene-src ~/src/ferrocene > coverage.txt
```
