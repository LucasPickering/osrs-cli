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

#### Store your username for easier lookups

If you often do a hiscore lookup for your username (or someone else's), you can store that as the default with:

```
osrs config set default_player <username>
```

Then you can just use `osrs hiscore` to do a lookup on the default player.

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
Patches: Catherby (+10% yield), Falador (+10% XP), Troll Stronghold (disease-free)
Magic secateurs: Yes
Farming cape: No
Bottomless bucket: Yes
Compost: Ultracompost
Anima plant: None

Survival chance is an average across all patches. Yield values take into account survival chance.
+-------------+-----------------+-----------+--------+------------+------------+------------+
| Herb        | Survival Chance | Yield/Run | XP/Run | Seed Price | Herb Price | Profit/Run |
+-------------+-----------------+-----------+--------+------------+------------+------------+
| Guam leaf   |           95.4% |    25.970 |  465.6 |          7 |         12 |       -758 |
| Marrentill  |           95.4% |    25.970 |  538.0 |          4 |         17 |       -619 |
| Tarromin    |           95.4% |    26.124 |  626.2 |         10 |         88 |      1,221 |
| Harralander |           95.4% |    26.124 |  799.5 |        100 |        646 |     15,528 |
| Goutweed    |           95.4% |    26.124 | 1598.6 |    791,963 |          — | -2,376,936 |
| Ranarr weed |           95.4% |    26.124 |  985.8 |     44,069 |      6,756 |     43,239 |
| Toadflax    |           95.4% |    26.124 | 1215.8 |      3,300 |      2,145 |     45,088 |
| Irit leaf   |           95.4% |    26.124 | 1504.0 |         38 |        760 |     18,691 |
| Avantoe     |           95.4% |    26.280 | 1887.7 |        679 |      1,799 |     44,193 |
| Kwuarm      |           95.4% |    26.280 | 2364.9 |        264 |      1,470 |     36,791 |
| Snapdragon  |           95.4% |    26.280 | 2959.1 |     50,710 |      7,305 |     38,799 |
| Cadantine   |           95.4% |    26.280 | 3581.1 |      1,300 |      1,755 |     41,173 |
| Lantadyme   |           95.4% |    26.438 | 4516.9 |        650 |      1,418 |     34,491 |
| Dwarf weed  |           95.4% |    26.438 | 5695.7 |        993 |        938 |     20,772 |
| Torstol     |           95.4% |    26.438 | 6641.9 |     50,990 |      6,881 |     27,904 |
+-------------+-----------------+-----------+--------+------------+------------+------------+
```

If you unlock a new patch, get new gear, etc., you can easily update the config by running `osrs config set-herb` again.

Note: This calculator assumes you'll plant the same herb in all patches. You _could_ min/max more by putting different herbs in different patches, but that is not supported (yet). If you need that, feel free to request it.

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
