## Running

```
(
    cd ~/src/ferrocene
    ./configure --set build.library-docs-private-items --set build.profiler=true
    ./x run certified-core-symbols
    ./x test --coverage=library library/core library/std --no-doc
)

./run.nu
```
