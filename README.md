# osrs-cli

A command line tool for doing lookups and calculations related to Oldschool RuneScape. Features include:

- Calculate drop rate
- Calculate XP to a level
- Hiscores lookups
- And more!

## Installation

Currently this is only installable via cargo. **See `Cargo.toml` for minimum Rust version**.

```
cargo install osrs-cli
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

### Calculate drop rate

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

### Calculate XP to a level

Calculate the XP needed to a target. The source can be a level, XP value, or a skill+player combination (their current XP will be looked up on the hiscores). The target can be a level or XP value.

```
osrs calc xp --from-xp 100000 --to-lvl 80
osrs calc xp --from-lvl 50 --to-lvl 60
osrs calc xp --player <username> --skill smithing --to-xp 123456
```

### Calculate herb patch output

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
Farming level: 93
Magic secateurs: Yes
Farming cape: No
Bottomless bucket: Yes
Compost: Ultracompost
Anima plant: None
Patches: Ardougne, Catherby (+10% yield), Falador (+10% XP), Farming Guild (+5% yield), Hosidius (disease-free, +5% yield), Port Phasmatys, Troll Stronghold (disease-free), Weiss (disease-free)

Survival chance is an average across all patches. Yield values take into account survival chance.
+-------------+-----------------+-----------+---------+------------+------------+------------+
| Herb        | Survival Chance | Yield/Run |  XP/Run | Seed Price | Herb Price | Profit/Run |
+-------------+-----------------+-----------+---------+------------+------------+------------+
| Guam leaf   |           95.7% |    69.192 |  1240.9 |         32 |         11 |     -2,555 |
| Marrentill  |           95.7% |    69.192 |  1433.9 |          3 |         16 |     -1,976 |
| Tarromin    |           95.7% |    69.601 |  1668.8 |         10 |        122 |      5,351 |
| Harralander |           95.7% |    69.601 |  2130.4 |         23 |        662 |     42,833 |
| Goutweed    |           95.7% |    69.601 |  4260.1 |    846,763 |          — | -6,777,160 |
| Ranarr weed |           95.7% |    69.601 |  2626.8 |     44,859 |      6,757 |    108,366 |
| Toadflax    |           95.7% |    69.601 |  3239.6 |      3,299 |      2,165 |    121,236 |
| Irit leaf   |           95.7% |    69.601 |  4007.7 |         44 |        762 |     49,624 |
| Avantoe     |           95.7% |    70.016 |  5030.0 |        523 |      1,764 |    116,264 |
| Kwuarm      |           95.7% |    70.016 |  6301.2 |        450 |      1,527 |    100,253 |
| Snapdragon  |           95.7% |    70.016 |  7884.6 |     50,450 |      7,376 |    109,777 |
| Cadantine   |           95.7% |    70.016 |  9541.9 |      1,599 |      1,725 |    104,925 |
| Lantadyme   |           95.7% |    70.435 | 12034.9 |        668 |      1,420 |     91,614 |
| Dwarf weed  |           95.7% |    70.435 | 15175.6 |      1,000 |        959 |     56,488 |
| Torstol     |           95.7% |    70.435 | 17696.7 |     48,907 |      6,901 |     91,757 |
+-------------+-----------------+-----------+---------+------------+------------+------------+
```

If you unlock a new patch, get new gear, etc., you can easily update the config by running `osrs config set-herb` again.

Note: This calculator assumes you'll plant the same herb in all patches. You _could_ min/max more by putting different herbs in different patches, but that is not supported (yet). If you need that, feel free to request it.

#### Store your username for easier lookups

If you often do a hiscore lookup for your username (or someone else's), you can store that as the default with:

```
osrs config set default_player <username>
```

Then you can just use `osrs hiscore` to do a lookup on the default player.

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
