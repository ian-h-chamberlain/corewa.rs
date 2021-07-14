# corewars

[![Latest Github release](https://img.shields.io/github/v/release/corewa-rs/corewars?label=Release&include_prereleases&logo=github)](https://github.com/corewa-rs/corewars/releases)
[![Build status](https://img.shields.io/github/workflow/status/corewa-rs/corewars/ci/develop)](https://github.com/corewa-rs/corewars/actions)

[![Latest corewars release](https://img.shields.io/crates/v/corewars?label=corewars&logo=rust)](https://crates.io/crates/corewars)
[![Latest corewars-core release](https://img.shields.io/crates/v/corewars-core?label=corewars-core&logo=rust)](https://crates.io/crates/corewars-core)
[![Latest corewars-parser release](https://img.shields.io/crates/v/corewars-parser?label=corewars-parser&logo=rust)](https://crates.io/crates/corewars-parser)
[![Latest corewars-sim release](https://img.shields.io/crates/v/corewars-sim?label=corewars-sim&logo=rust)](https://crates.io/crates/corewars-parser)

A Rust implementation of the classic programming battle game
[Core Wars](http://www.koth.org/index.html).

The implementation is based on [this introductory guide](http://vyznev.net/corewar/guide.html) to Redcode, as well as the [pMARS '94 reference](https://corewa.rs/reference/pmars-redcode-94.txt) and an [annotated version](https://corewa.rs/reference/icws94.txt) of the ICWS '94 draft.

## Quick start (command line)

First install cargo via [rustup](https://rustup.rs/).

```sh
$ cargo install corewars
...
Installed package `corewars v0.2.0` (executable `corewars`)

$ ~/.cargo/bin/corewars --version
corewars 0.2.0
```

## Cargo Crates

Latest [documentation](https://corewa.rs/crates/corewars/) published from develop,
with older version docs available on [docs.rs](https://docs.rs/corewars).

* [corewars](https://crates.io/crates/corewars): the binary to run Core Wars
  from the command line.
* [corewars-core](https://crates.io/crates/corewars-core): data structures and
  utilities common to other crates (such as the representation of a "core").
* [corewars-parser](https://crates.io/crates/corewars-parser): the parser used to
  read and error-check Redcode files. Output from this crate will be used as input
  for the MARS simulation itself.
* [corewars-sim](https://crates.io/crates/corewars-sim): simulation of a core.
  This is the main logic used to pit warriors against one another (the MARS).

## Other tools

* A VSCode syntax highlighting
  [plugin](https://marketplace.visualstudio.com/items?itemName=corewa-rs.redcode)
  for Redcode
