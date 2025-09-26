## Running

```
(
    cd ~/src/ferrocene
    ./configure --set build.library-docs-private-items --set build.profiler=true
    ./x doc --json library/core
    ./x test --coverage=library library/core library/std --no-doc
)

./run.nu
```
