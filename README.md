# client

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

*An ASCII client for exploring hexagram30 worlds, based upon rustrogueliketutorial*

[![Project Logo][logo]][logo-large]

## Project Status

* [x] Phase 1: Rust roguelike tutorial basics
* [x] Phase 2: Rust roguelike tutorial intermediate steps, highly-customized
* [ ] Phase 3: Rust roguelike tutorial advanced
* [ ] Phase 4: Persisted levels to disk
* [ ] Phase 5: Level generation from world biome data

## Background

This repo started as part of a UMass UWW course, Spring Semester 2020 (UWW 310:
Experiential Reflections on Technology), a semester's project whose aim was to
provide a means of exploring the generated worlds of Hexagram30 via a roguelike
ASCII character interface. The code follows the 
[Rust roguelike tutorial][rustrogueliketutorial], though differs in
configuration and code structure (and will contnue to diverge more, as time
passes).


<!-- Named page links below: /-->

[logo]: https://raw.githubusercontent.com/hexagram30/resources/master/branding/logo/h30-logo-2-long-with-text-x695.png
[logo-large]: https://raw.githubusercontent.com/hexagram30/resources/master/branding/logo/h30-logo-2-long-with-text-x3440.png
[build]: https://github.com/hexagram30/client/actions?query=workflow%3Abuild+
[build-badge]: https://github.com/hexagram30/client/workflows/build/badge.svg
[crate]: https://crates.io/crates/hxgm30-client
[crate-badge]: https://img.shields.io/crates/v/hxgm30-client.svg
[docs]: https://docs.rs/hxgm30-client/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag]: https://github.com/hexagram30/client/tags
[tag-badge]: https://img.shields.io/github/v/tag/hexagram30/client.svg?sort=semver
[rustrogueliketutorial]: http://bfnightly.bracketproductions.com/rustbook/chapter_0.html
