# swap-cpp-rust

A small comparative bench of std::swap to assert its actual behavior

## Rust bench

To launch the Rust Criterion benchmark:

```sh
cargo bench
```

It is also possible to compare implementation through binaries:

```sh
cargo build -r && hyperfine --shell=none \
                      "./target/release/swap 10" \
                      "./target/release/swap 1000000" \
                      "./target/release/naive_swap 10" \
                      "./target/release/naive_swap 1000000"
```

## C++ bench

To observe scaling of C++ `std::swap` on `std::vector`:

```sh
cmake -S . -B build && cmake --build build && hyperfine "./build/main 10" "./build/main 1000000"
```