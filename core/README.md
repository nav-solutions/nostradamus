Nostradamus (core)
==================

[![Rust](https://github.com/nav-solutions/nostradamus/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/nostradamus/actions/workflows/rust.yml)
[![Rust](https://github.com/nav-solutions/nostradamus/actions/workflows/daily.yml/badge.svg)](https://github.com/nav-solutions/nostradamus/actions/workflows/daily.yml)
[![crates.io](https://docs.rs/nostradamus-core/badge.svg)](https://docs.rs/nostradamus-core/)
[![crates.io](https://docs.rs/nostradamus-cli/badge.svg)](https://docs.rs/nostradamus-cli/)
[![crates.io](https://img.shields.io/crates/d/nostradamus-core.svg)](https://crates.io/crates/nostradamus-core)

[![MRSV](https://img.shields.io/badge/MSRV-1.83.0-orange?style=for-the-badge)](https://github.com/rust-lang/rust/releases/tag/1.83.0)
[![License](https://img.shields.io/badge/license-AGPLv3.0-orange?style=for-the-badge)](https://github.com/nav-solutions/gnss-rtk/blob/main/LICENSE)

`nostradamus-core` is the core library for simulation and orbital prediction.
It provides three main objects:

- `Satellite` which is a solver that supports both forward and backwards predictions
- `Simulation` is the process that will eventually get deployed
- `User` is a ground scenario

<div align="center">
    <img src="https://raw.githubusercontent.com/nav-solutions/.github/master/logos/logo3.png" width=750px alt="Logo">
</div>


API
===

TODO
