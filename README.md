# Assets benchmark

A simple benchmark using tokio to test the differences between `UDS` and `TCP` based sockets.

## Results

Computer: `Macbook Pro 2017 14,3 i7-7700HQ, 16GB LPDDR3`

Output:

```bash
$ cargo run --release
   Compiling assets-benchmark v0.1.0 (/Users/lucio/code/assets-benchmark)
    Finished release [optimized] target(s) in 1.25s
     Running `target/release/assets-benchmark`
Starting TCP benchmark
TCP took: 300ms
Starting UDS benchmark
UDS took: 23883ms
```
