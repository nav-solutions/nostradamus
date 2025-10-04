Nostradamus
===========

[![Rust](https://github.com/nav-solutions/nostradamus/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/nostradamus/actions/workflows/rust.yml)
[![Rust](https://github.com/nav-solutions/nostradamus/actions/workflows/daily.yml/badge.svg)](https://github.com/nav-solutions/nostradamus/actions/workflows/daily.yml)
[![crates.io](https://docs.rs/nostradamus-core/badge.svg)](https://docs.rs/nostradamus-core/)
[![crates.io](https://docs.rs/nostradamus-cli/badge.svg)](https://docs.rs/nostradamus-cli/)
[![crates.io](https://img.shields.io/crates/d/nostradamus-core.svg)](https://crates.io/crates/nostradamus-core)

[![MRSV](https://img.shields.io/badge/MSRV-1.83.0-orange?style=for-the-badge)](https://github.com/rust-lang/rust/releases/tag/1.83.0)
[![License](https://img.shields.io/badge/license-AGPLv3.0-orange?style=for-the-badge)](https://github.com/nav-solutions/gnss-rtk/blob/main/LICENSE)

`nostradamus` is an open-source framework to simulate and predict the state of a constellation
of satellites. To achieve this task, it combines the [Nyx-Space spatial solver](https://nyx-space.com)
and its own temporal solver.

This framework is divided into a two parts:

- the [core library](core/) which is open-source and can be used to perform POD elsewhere
- the [command-line tool](cli/) allows to initiate a simulation. It is available worldwide
through crates.io

<div align="center">
    <img src="https://raw.githubusercontent.com/nav-solutions/.github/master/logos/logo3.png" width=250px alt="Logo">
</div>

Contributions
=============

To contribute to either of our project or join our community, you way
- open an [Issue on Github.com](https://github.com/nav-solutions/nostradamus/issues) 
- follow our [Discussions on Github.com](https://github.com/nav-solutions/discussions)
- join our [Discord channel](https://discord.gg/EqhEBXBmJh)

Citation and referencing
========================

If you need to reference this library, please use the following model:

`nav-solutions (2025), nostradamus: orbit determination and simulations (AGPL-3), https://github.com/nav-solutions`

Gloassary
=========

- OD: Orbital Determination  
- ODP: Orbital Determination Process, is a tasklet we can deploy on individual satellites
- Prediction: as opposed to an actually measured state, is the result of our solver
- SV: Satellite Vehicle, also referred to as spacecraft or simply "satellite"
- Timescale: hum ?

API
===

The API is [documented in the library folder](core/)
