`src/main.rs` should output

```
$  cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rstar-boilerplate`
thread 'main' panicked at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/ops/arith.rs:346:1:
attempt to multiply with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

after cloning this repository and running `cargo run`.
