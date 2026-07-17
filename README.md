# qlora-gemm

<!-- FLEET-BADGES:BEGIN -->
[![CI](https://github.com/tzervas/qlora-gemm/actions/workflows/fleet-ci.yml/badge.svg?branch=main)](https://github.com/tzervas/qlora-gemm/actions/workflows/fleet-ci.yml?query=branch%3Amain)
[![Security](https://github.com/tzervas/qlora-gemm/actions/workflows/fleet-security.yml/badge.svg?branch=main)](https://github.com/tzervas/qlora-gemm/actions/workflows/fleet-security.yml?query=branch%3Amain)
<!-- FLEET-BADGES:END -->

A fork of the [gemm](https://github.com/sarah-ek/gemm/) crate that uses [qlora-paste](https://crates.io/crates/qlora-paste) instead of the unmaintained [paste](https://crates.io/crates/paste) crate.

This fork addresses the RUSTSEC-2024-0436 security advisory by replacing the unmaintained paste dependency with a maintained fork.

Playground for testing high performance matrix multiplication.
