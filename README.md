# kiwirail
## Problem Statement
The local commuter railroad services a number of towns in Kiwiland. Because of
monetary concerns, all of the tracks are 'one-way.' That is, a route from Kaitaia to Invercargill
does not imply the existence of a route from Invercargill to Kaitaia. In fact, even if both of these
routes do happen to exist, they are distinct and are not necessarily the same distance!
The purpose of this problem is to help the railroad provide its customers with information about
the routes. In particular, you will compute the distance along a certain route, the number of
different routes between two towns, and the shortest route between two towns.

## Analysis
Given the particularities fo the _one way_ tracks, this problem is best modeled with a _directed_ graph with _weighed_ edges. 

Objective is to provide APIs for:
  1. computing distance along a given route (or path)
  2. counting the number of possible routes between two towns
  3. shortest route between two towns

Api 1. is basically a traversal along a given path on the graph while accounting edges distances (cost)
Api 2. is, according to my initial intuition is a modifided BFS (Breath First Search) where different paths a counted
Api 3. is the standard application of _Dijkstra's Algorithm_


[![Crates.io](https://img.shields.io/crates/v/kiwirail.svg)](https://crates.io/crates/kiwirail)
[![Docs.rs](https://docs.rs/kiwirail/badge.svg)](https://docs.rs/kiwirail)
[![CI](https://github.com/albinocordeiro/kiwirail/workflows/Continuous%20Integration/badge.svg)](https://github.com/albinocordeiro/kiwirail/actions)
[![Coverage Status](https://coveralls.io/repos/github/albinocordeiro/kiwirail/badge.svg?branch=main)](https://coveralls.io/github/albinocordeiro/kiwirail?branch=main)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install kiwirail`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
