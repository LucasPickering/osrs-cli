# osrs-cli

A command line tool for doing lookups and calculations related to Oldschool RuneScape. Features include:

- Calculate drop rate
- Calculate XP to a level
- Hiscores lookups
- And more!

This tool is intended for people who are already familiar and comfortable with the command line. If you aren't, then you're probably better off using the wiki and other web-based tools.

### Table of Contents

- [Installation](#installation)
- [Examples](#examples)
  - [Hiscore Lookup](#hiscore-lookup)
  - [Calculators](#calculators)
    - [Drop Rate](#calculate-drop-rate)
    - [XP/Levels](#calculate-xp-to-a-level)
    - [Herb Farming](#calculate-herb-patch-output)
    - [Spicy Stews](#calculate-spicy-stew-boosts)
  - [Wiki Search](#search-the-wiki)
  - [Ping](#ping-a-world)
- [Bug Reports/Feature Requests](#bug-reportsfeature-requests)

## Installation

There are a few ways to install the project:

- From binary
- From [crates.io](https://crates.io)
- From source

### From Binary

Go to [the Releases page](https://github.com/LucasPickering/osrs-cli/releases) and download the latest release for your platform (for Windows, you probably want the `msvc` version). Extract the archive and copy the executable file (either `osrs` or `osrs.exe`) to a directory in your `PATH`.

### From crates.io

You can install this tool from [crates.io](https://crates.io) if you have a working Rust toolchain installed. You'll need a working Rust toolchain set up (Rustup & Cargo), [see here](https://doc.rust-lang.org/cargo/getting-started/installation.html) for instructions. Then run:

```sh
cargo install osrs-cli
```

This will install the latest version of the CLI to your path automatically.

### From Source

This is the most involved option, and you probably only want it if you really want to use an unreleased version of the tool. You'll need a working Rust toolchain for this (see instructions in the "crates.io" section above). Then, clone this repo and run:

```sh
cargo install --path .
```

## Examples

For any command, you can get detailed information about arguments and usage with `--help`, for example:

```
osrs --help
osrs calc --help
osrs calc drop --help
```

### Hiscore lookup

Look up a user's stats and kill counts in the hiscores:

```
osrs hiscore <username>
```

#### Store your username for easier lookups

If you often do a hiscore lookup for your username (or someone else's), you can store that as the default with:

```
osrs config set default_player <username>
```

Then you can just use `osrs hiscore` to do a lookup on the default player. This username will also be used for any other player lookups, e.g. `osrs calc xp`.

### Calculators

The tool has a number of calculators, all under the `osrs calc` subcommand

#### Calculate drop rate

If you're going for a pet with a 1/5000 drop rate and you want to know the odds of getting it in the first 1000 kills:

```
> osrs calc drop -p 1/5000 -n 1000
18.1286% chance of ≥1 successes in 1000 attempts
```

Or if you want to know the odds of getting all 4 pieces of the Angler's Outfit in 40 Fishing Trawler trips:

```
> osrs calc drop -p 1/12 -n 40 -t 4+
43.0149% chance of ≥4 successes in 40 attempts
```

#### Calculate XP to a level

Calculate the XP needed to a target. The source can be a level, XP value, or a skill+player combination (their current XP will be looked up on the hiscores). The target can be a level or XP value.

```
osrs calc xp --from-xp 100000 --to-lvl 80
osrs calc xp --from-lvl 50 --to-lvl 60
osrs calc xp --player <username> --skill smithing --to-xp 123456
```

#### Calculate herb patch output

Picking which herb to grow is complicated. It involves a lot of math and there's a lot of different potential buffs to be applied. This calculator lets you configure your buffs once, then easily check the profitability (as well as XP gain) from all herbs at any time. Start by configuring your herb setup with:

```
osrs config set-herb
```

This will ask a bunch of questions about what patches, gear, and buffs you have. Once that's done, run the calculator with:

```
osrs calc farm herb
```

Here's some example output:

```
Farming level: 50
Patches:
 - Ardougne
 - Catherby (+10% yield)
 - Falador (+10% XP)
 - Port Phasmatys
Magic secateurs: Yes
Farming cape: No
Bottomless bucket: No
Compost: Ultracompost
Anima plant: None

Survival chance is an average across all patches. Yield values take into account survival chance.
+-------------+-----+--------------+-----------+--------+------------+------------+------------+
| Herb        | Lvl | Surv. Chance | Yield/Run | XP/Run | Seed Price | Herb Price | Profit/Run |
+-------------+-----+--------------+-----------+--------+------------+------------+------------+
| Guam leaf   |   9 |        93.1% |    29.086 |  551.6 |          8 |         15 |     -1,997 |
| Marrentill  |  14 |        93.1% |    29.385 |  638.8 |          7 |         17 |     -1,929 |
| Tarromin    |  19 |        93.1% |    29.577 |  740.4 |         29 |        114 |        854 |
| Harralander |  26 |        93.1% |    30.043 |  951.0 |         26 |        775 |     20,778 |
| Goutweed    |  29 |        93.1% |    30.201 | 1923.1 |    889,350 |          — | -3,559,800 |
| Ranarr weed |  32 |        93.1% |    30.201 | 1173.1 |     45,000 |      6,986 |     28,585 |
| Toadflax    |  38 |        93.1% |    30.687 | 1461.5 |      2,325 |      2,145 |     54,122 |
| Irit leaf   |  44 |        93.1% |    31.020 | 1820.5 |         38 |        798 |     22,200 |
| Avantoe     |  50 |        93.1% |    31.406 | 2293.4 |        800 |      1,783 |     50,393 |
+-------------+-----+--------------+-----------+--------+------------+------------+------------+

```

If you unlock a new patch, get new gear, etc., you can easily update the config by running `osrs config set-herb` again.

Note: This calculator assumes you'll plant the same herb in all patches. You _could_ min/max more by putting different herbs in different patches, but that is not supported (yet). If you need that, feel free to request it.

#### Calculate spicy stew boosts

Tired of training for achievement diaries? Ever wondered how many doses of spice you should collect before attempting a spicy stew boost? This calculator will help you out!

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

Not only will it tell you the odds of hitting your desired boost, it will tell you how many doses you should put in each stew to maximize that chance. In this case, if you want a boost of 3 (or more), you should put 3 doses in each stew, to get a 90% chance of hitting that boost at least once (in 8 stews).

### Search the wiki

Search any term on the [Old School RuneScape Wiki](https://oldschool.runescape.wiki/):

```
osrs wiki shark
osrs wiki smithing
```

### Ping a world

Curious how laggy a world will be? Ping it!

```
osrs ping 450
```

## Bug Reports/Feature Requests

Found a bug or have a suggestion for a new feature? [Submit an issue on this repo](https://github.com/LucasPickering/osrs-cli/issues/new).

## Rust Version

See `Cargo.toml` for minimum Rust version. This can be built on both stable/beta and nightly. It optionally uses the following nightly features, which are simply disabled when building on stable/beta:

- Rust Features
  - [backtrace](https://github.com/rust-lang/rust/issues/53487)
- Rustfmt
  - imports_granularity
  - [wrap_comments](https://github.com/rust-lang/rustfmt/issues/3347)

[Here's a handy site for finding new Rust nightly versions](https://rust-lang.github.io/rustup-components-history/).
