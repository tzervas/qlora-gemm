# qlora-gemm

A fork of the [gemm](https://github.com/sarah-ek/gemm/) crate that uses [qlora-paste](https://crates.io/crates/qlora-paste) instead of the unmaintained [paste](https://crates.io/crates/paste) crate.

This fork addresses the RUSTSEC-2024-0436 security advisory by replacing the unmaintained paste dependency with a maintained fork.

Playground for testing high performance matrix multiplication.
