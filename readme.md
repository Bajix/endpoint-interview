To build:
```
cargo build --release
```

To build & run dev:

```
cargo run
```

To run compiled release:
```
target/release/endpoint
```

To benchmark 1000 runs
```
time seq 1000 | xargs -I{} -P1 -n1 target/release/endpoint > /dev/null
```
