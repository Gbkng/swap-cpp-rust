#!/usr/bin/env bash
cargo build --profile profiling && 
  valgrind --tool=cachegrind --cachegrind-out-file=cachegrind.out ./target/profiling/swap &&
  cg_annotate cachegrind.out
