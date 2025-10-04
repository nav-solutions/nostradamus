Nostradamus
===========

[![Rust](https://github.com/nav-solutions/nostradamus/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/nostradamus/actions/workflows/rust.yml)
[![Rust](https://github.com/nav-solutions/nostradamus/actions/workflows/daily.yml/badge.svg)](https://github.com/nav-solutions/nostradamus/actions/workflows/daily.yml)
[![crates.io](https://docs.rs/nostradamus-lib/badge.svg)](https://docs.rs/nostradamus-lib/)
[![crates.io](https://docs.rs/nostradamus-cli/badge.svg)](https://docs.rs/nostradamus-cli/)
[![crates.io](https://img.shields.io/crates/d/nostradamus.svg)](https://crates.io/crates/nostradamus-lib)

[![MRSV](https://img.shields.io/badge/MSRV-1.83.0-orange?style=for-the-badge)](https://github.com/rust-lang/rust/releases/tag/1.83.0)
[![License](https://img.shields.io/badge/license-AGPLv3.0-orange?style=for-the-badge)](https://github.com/nav-solutions/gnss-rtk/blob/main/LICENSE)

`nostradamus` is an open-source framework to simulate and predict satellites states.
It operates a from a user scenario (often-times referred to as the "Observer"). The basic workflow
is to arm `nostradamus` from a user [GPX](https://en.wikipedia.org/wiki/GPX) track and we will recreate
the orbital state that would physically led to each individual states of this track.

The orbital state can be preset from a [RINEX](https://en.wikipedia.org/wiki/RINEX) file,
but the library is powerful enough to be able to arm itself from scratch.

This framework is divided into a library (`nostradamus-lib`) and a command line application
(`nostradamus-cli`). Currently the framework is limited to text file synthesis, it will not generate
a signal snapshot and therefore, is not a constellation simulator as some people would expect.

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

API
===

TODO
