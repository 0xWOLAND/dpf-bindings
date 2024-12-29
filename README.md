# An Implementation of Incremental Distributed Point Functions in C++ with Rust Bindings [![Build status](https://badge.buildkite.com/64bb7c0fcc8c11d630517356b2c3932d7e14850801a5f22c48.svg?branch=master)](https://buildkite.com/bazel/google-distributed-point-functions)

This library contains an implementation of incremental distributed point
functions, based on the following paper:
> Boneh, D., Boyle, E., Corrigan-Gibbs, H., Gilboa, N., & Ishai, Y. (2020).
Lightweight Techniques for Private Heavy Hitters. arXiv preprint
> arXiv:2012.14884. https://arxiv.org/abs/2012.14884

## Project Structure

```
.
├── MODULE.bazel         # Bazel module configuration (Bzlmod)
├── WORKSPACE.bzlmod     # Bazel workspace configuration
├── Cargo.toml          # Rust crate manifest
├── dpf/                # Core C++ implementation
│   ├── BUILD           # Bazel build file for C++ library
│   ├── distributed_point_function.h
│   └── distributed_point_function.cc
├── rust/               # Rust bindings
│   ├── BUILD          # Bazel build file for Rust library
│   └── lib.rs         # Rust bindings implementation
└── examples/          # Usage examples
```

## About Incremental Distributed Point Functions

A distributed point function (DPF) is parameterized by an index `alpha` and a
value `beta`. It consists of two algorithms: key generation and evaluation.
The key generation procedure produces two keys `k_a` and `k_b`, given `alpha`
and `beta`. Evaluating each key on any point `x` in the DPF domain results in an
additive secret share of `beta`, if `x == alpha`, and a share of 0 otherwise.

Incremental DPFs additionally can be evaluated on prefixes of the index domain.
More precisely, an incremental DPF is parameterized by a hierarchy of index
domains, each a power of two larger than the previous. Key generation now takes
a vector `beta`, one value `beta[i]` for each hierarchy level.
When evaluated on a `b`-bit prefix of `alpha`, where b is the log domain size of
the `i`-th hierarchy level, the incremental DPF returns a secret share of
`beta[i]`, otherwise a share of 0.

For more details, see the above paper, as well as the
[`DistributedPointFunction` class documentation](dpf/distributed_point_function.h).

## Building the Project

This project uses both Bazel and Cargo for building. The core C++ implementation is wrapped with a C++ wrapper layer, which is then exposed to Rust through bindings.

### Prerequisites

- Bazel (latest version with Bzlmod support)
- Rust toolchain
- C++ compiler

### Building with Bazel

The build process involves three main components:
1. The core C++ DPF implementation
2. The C++ wrapper layer
3. The Rust bindings

To build everything:
```bash
bazel build //...
```

To build components individually:
```bash
# First build the core C++ implementation
bazel build //dpf:distributed_point_function

# Then build the C++ wrapper
bazel build //dpf:wrapper

# Finally build the Rust bindings
bazel build //rust:dpf
```

### Building with Cargo

When building with Cargo, you'll still need Bazel installed as the build process needs to compile the C++ components. The Cargo build will automatically handle building the C++ wrapper and linking it with the Rust bindings:

```bash
cargo build
```

## Running Tests

To run all tests:
```bash
bazel test //...
```

To test individual components:
```bash
# Test the C++ implementation
bazel test //dpf:distributed_point_function_test

# Test the wrapper
bazel test //dpf:wrapper_test

# Test the Rust bindings
bazel test //rust:dpf_test
# or with Cargo
cargo test
```

## Security
To report a security issue, please read [SECURITY.md](SECURITY.md).

## Disclaimer

This is not an officially supported Google product. The code is provided as-is,
with no guarantees of correctness or security.
