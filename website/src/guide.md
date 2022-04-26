---
layout: default
---

# Usage Guide

## Table of Contents

- [Commands](#commands)
  - [Hiscore Lookup](#hiscore-lookup)
  - [Price Lookup](#price-lookup)
  - [Calculators](#calculators)
    - [Drop Rate](#calculate-drop-rate)
    - [XP/Levels](#calculate-xp-to-a-level)
    - [Spicy Stews](#calculate-spicy-stew-boosts)
  - [Wiki Search](#search-the-wiki)
  - [Ping](#ping-a-world)
- [Configuration](#configuration)

## Commands

For any command, you can get detailed information about arguments and usage with `--help`, for example:

```
osrs --help
osrs calc --help
osrs calc drop --help
```

### Hiscore lookup

Look up a user's stats and kill counts in the hiscores:

```
> osrs hiscore Swampletics
Skills
+--------------+-----------+-------+-------------+
| Skill        |      Rank | Level |          XP |
+--------------+-----------+-------+-------------+
| Total        |   387,328 | 1,796 | 121,473,079 |
|              (output abbreviated)              |
| Hunter       |    19,900 |    99 |  13,640,216 |
| Construction |   726,562 |    55 |     174,955 |
+--------------+-----------+-------+-------------+

Minigames
+----------------------+-----------+-------+
| Minigame             |      Rank | Score |
+----------------------+-----------+-------+
| Clue Scroll (Medium) | 1,056,506 |     1 |
| Barrows Chests       |     1,518 | 2,183 |
| Theatre of Blood     |    23,547 |   125 |
+----------------------+-----------+-------+
```

You can [store your own username in your config file](#storing-your-username-for-easier-lookups) so that the `hiscore` subcommand, and others that require fetching skill information, can use your RSN when none is provided.

### Price Lookup

Look up prices on the Grand Exchange using the `price` subcommand:

```
$ osrs price bandos godsword
+------------------------------+------------+
| Item                         |      Price |
+------------------------------+------------+
| Bandos godsword              | 17,611,983 |
| Bandos godsword ornament kit |  5,800,944 |
+------------------------------+------------+
```

Alternatively, you can use the `osrs ge` alias.

### Calculators

The tool has a number of calculators, all under the `osrs calc` subcommand

#### Calculate drop rate

If you're going for a pet with a 1/5000 drop rate and you want to know the odds of getting it in the first 1000 kills:

```
$ osrs calc drop -p 1/5000 -n 1000
18.1286% chance of ≥1 successes in 1000 attempts
```

Or if you want to know the odds of getting all 4 pieces of the Angler's Outfit in 40 Fishing Trawler trips:

```
$ osrs calc drop -p 1/12 -n 40 -t 4+
43.0149% chance of ≥4 successes in 40 attempts
```

Some bosses have multiple loot table rolls. For example, calculating the chance of getting 4+ Black tourmaline core drops in 221 kills from [Grostesque Guardians](https://oldschool.runescape.wiki/w/Grotesque_Guardians#Drops):

```
$ osrs calc drop -p 1/1000 --rolls 2 --kc 221 -t 4+
0.1108% chance of ≥4 successes in 221 attempts, with 2 roll(s)/attempt
```

#### Calculate XP to a level

Calculate the XP needed to a target. The source can be a level, XP value, or a skill+player combination (their current XP will be looked up on the hiscores). The target can be a level or XP value.

```
$ osrs calc xp --from-xp 100000 --to-lvl 80
100,000 XP (Level 49) => 1,986,068 XP (Level 80) = 1,886,068 XP

$ osrs calc xp --from-lvl 50 --to-lvl 60
101,333 XP (Level 50) => 273,742 XP (Level 60) = 172,409 XP

$ osrs calc xp --player swampletics --skill smithing --to-xp 12345678
1,039,361 XP (Level 73) => 12,345,678 XP (Level 98) = 11,306,317 XP
```

Alternatively, want to know what level you'll be after gaining a certain amount of XP? Maybe you want to know what level you'll have after a quest?

```
$ osrs calc xp --from-lvl 1 --plus-xp 13750
0 XP (Level 1) => 13,750 XP (Level 30) = 13,750 XP
```

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
osrs wiki one small favour
```

### Ping a world

Curious how laggy a world will be? Ping it!

```
osrs ping 450
```

## Configuration

OSRS CLI supports persistent configuration to store common inputs. Configuration can be read and modified via the `osrs config` subcommand family. Some examples:

```sh
osrs config get # Get the entire config
osrs config get default_player # Get the default_player field
osrs config set default_player Lynx Titan # Set the default_player
```

#### Storing your username for easier lookups

If you often do a hiscore lookup for your username (or someone else's), you can store that as the default with:

```
osrs config set default_player <username>
```

Then you can just use `osrs hiscore` to do a lookup on the default player. This username will also be used for any other player lookups, e.g. `osrs calc xp --skill smithing`.
