# [osrs-cli](https://osrs.lucaspickering.me)

[![Test Status](https://github.com/LucasPickering/osrs-cli/actions/workflows/test.yml/badge.svg)](https://github.com/LucasPickering/osrs-cli/actions/workflows/test.yml)
[![crates.io version](https://img.shields.io/crates/v/osrs-cli.svg)](https://crates.io/crates/osrs-cli)

A command line tool for doing lookups and calculations related to Oldschool RuneScape. Features include:

- Calculate drop rate
- Calculate XP to a level
- Hiscores lookups
- And more!

This tool is intended for people who are already familiar and comfortable with the command line. If you aren't, then you're probably better off using the wiki and other web-based tools.

## Installation

[See the website](https://osrs.lucaspickering.me/install)

## Examples

Here are some simple examples. For more detail, see the [user guide on the website](https://osrs.lucaspickering.me/guide).

### Hiscore lookup

```
osrs hiscore <username>
```

### Calculate drop rate

```
> osrs calc drop --probability 1/5000 --kills 1000
18.1286% chance of ≥1 successes in 1000 attempts
```

#### Calculate XP to a level

```
osrs calc xp --from-xp 100000 --to-lvl 80
osrs calc xp --from-lvl 50 --to-lvl 60
```

### Calculate spicy stew boosts

```
> osrs calc stew --doses 25 --boost 3

+------------+-------+-------+-------+-------+-------+
| Doses/Stew |   ≥+1 |   ≥+2 |   ≥+3 |   ≥+4 |   ≥+5 |
+------------+-------+-------+-------+-------+-------+
|          1 | 99.9% |  0.0% |  0.0% |  0.0% |  0.0% |
|          2 | 99.6% | 96.8% | 79.9% |  0.0% |  0.0% |
|          3 | 98.7% | 96.1% | 90.0% | 76.8% | 50.0% |
+------------+-------+-------+-------+-------+-------+
```

### Search the wiki

```
osrs wiki shark
osrs wiki smithing
```

### Ping a world

```
osrs ping 450
```

## Bug Reports/Feature Requests

Found a bug or have a suggestion for a new feature? [Submit an issue on this repo](https://github.com/LucasPickering/osrs-cli/issues/new).

## Development

Interested in contributing? Here's some basic steps for setup:

### CLI

The CLI is written entirely in Rust.

Required tools:

- [rustup](https://rustup.rs/)

```sh
cargo run -- help # Run any command
cargo test # Run unit tests
```

#### Rust Version

See `Cargo.toml` for minimum Rust version. This can be built on both stable/beta and nightly. It optionally uses the following nightly features, which are simply disabled when building on stable/beta:

- Rust Features
  - [backtrace](https://github.com/rust-lang/rust/issues/53487)
- Rustfmt
  - imports_granularity
  - [wrap_comments](https://github.com/rust-lang/rustfmt/issues/3347)

[Here's a handy site for finding new Rust nightly versions](https://rust-lang.github.io/rustup-components-history/).

### Website

The website is HTML/CSS, compiled using the 11ty framework.

Required tools:

- [nvm](https://github.com/nvm-sh/nvm)

```sh
cd website
npm install
npm run start
```

Change files and you should see the site refresh in your browser.
