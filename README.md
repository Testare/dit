# DIT

## Main concept

A game spec that creates validatable game files using the Proof-of-work concept similar to blockchain, using actions similar to that in the redux.js spec.
The game supports multiple sub-specs called "Modes".
The game can support multiple interfaces, so long as they create files that follow the spec.
The first interface will be a CLI built in rust.
Gameplay in Mode A is loosely built on DnD.

This crate should be (for now):
* The core DIT code (Implementation information that is the same regardless of mode)
* Mode A spec
* The CLI interface to Mode A

Over time I will probably split those things up into different crates.

The CLI interface will likely support commands for actions and viewing the state, as well as an interactive mode using [crossterm](https://crates.io/crates/crossterm)