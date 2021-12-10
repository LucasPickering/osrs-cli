---
layout: default
---

# Usage Guide

For any command, you can get detailed information about arguments and usage with `--help`, for example:

```
osrs --help
osrs calc --help
osrs calc drop --help
```

## Hiscore lookup

Look up a user's stats and kill counts in the hiscores:

```
osrs hiscore <username>
```

### Store your username for easier lookups

If you often do a hiscore lookup for your username (or someone else's), you can store that as the default with:

```
osrs config set default_player <username>
```

Then you can just use `osrs hiscore` to do a lookup on the default player. This username will also be used for any other player lookups, e.g. `osrs calc xp`.

## Calculators

The tool has a number of calculators, all under the `osrs calc` subcommand

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
Farming level: 94
Patches:
 - Ardougne
 - Catherby (+10% yield)
 - Falador (+10% XP)
 - Farming Guild (+5% yield)
 - Hosidius (disease-free, +5% yield)
 - Port Phasmatys
 - Troll Stronghold (disease-free)
 - Weiss (disease-free)
Magic secateurs: Yes
Farming cape: No
Bottomless bucket: Yes
Resurrect crops: No
Compost: Ultracompost
Anima plant: None

Survival chance is an average across all patches. Yield values take into account survival chance.
+-------------+-----+-------+-----------+---------+---------+-------+------------+
| Herb        | Lvl | Surv% | Yield/Run |  XP/Run |   Seed$ | Herb$ | Profit/Run |
+-------------+-----+-------+-----------+---------+---------+-------+------------+
| Guam leaf   |   9 | 95.7% |    69.601 |  1246.0 |      22 |    20 |     -1,308 |
| Marrentill  |  14 | 95.7% |    69.601 |  1440.0 |       7 |    18 |     -1,328 |
| Tarromin    |  19 | 95.7% |    69.601 |  1668.8 |      10 |   121 |      5,817 |
| Harralander |  26 | 95.7% |    69.601 |  2130.4 |      20 |   653 |     42,765 |
| Goutweed    |  29 | 95.7% |    69.601 |  4260.1 | 949,414 |     — | -7,597,832 |
| Ranarr weed |  32 | 95.7% |    69.601 |  2626.8 |  44,236 | 7,340 |    154,462 |
| Toadflax    |  38 | 95.7% |    70.016 |  3255.6 |   3,429 | 2,127 |    118,970 |
| Irit leaf   |  44 | 95.7% |    70.016 |  4027.8 |      40 |   636 |     41,685 |
| Avantoe     |  50 | 95.7% |    70.016 |  5030.0 |     464 | 1,699 |    112,719 |
| Kwuarm      |  56 | 95.7% |    70.016 |  6301.2 |     587 | 1,404 |     91,083 |
| Snapdragon  |  62 | 95.7% |    70.016 |  7884.6 |  54,367 | 8,262 |    141,010 |
| Cadantine   |  67 | 95.7% |    70.016 |  9541.9 |   1,180 | 1,682 |    105,802 |
| Lantadyme   |  73 | 95.7% |    70.435 | 12034.9 |   1,438 | 1,335 |     80,004 |
| Dwarf weed  |  79 | 95.7% |    70.435 | 15175.6 |     578 |   914 |     57,231 |
| Torstol     |  85 | 95.7% |    70.435 | 17696.7 |  54,036 | 7,695 |    107,185 |
+-------------+-----+-------+-----------+---------+---------+-------+------------+
```

If you unlock a new patch, get new gear, etc., you can easily update the config by running `osrs config set-herb` again.

Note: This calculator assumes you'll plant the same herb in all patches. You _could_ min/max more by putting different herbs in different patches, but that is not supported (yet). If you need that, feel free to request it.

### Calculate spicy stew boosts

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

## Search the wiki

Search any term on the [Old School RuneScape Wiki](https://oldschool.runescape.wiki/):

```
osrs wiki shark
osrs wiki smithing
```

## Ping a world

Curious how laggy a world will be? Ping it!

```
osrs ping 450
```
