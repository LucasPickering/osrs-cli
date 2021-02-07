# osrs-cli

A command line tool for doing lookups and calculations related to Oldschool RuneScape. Features include:

- Calculate drop rate
- Calculate XP to a level
- Hiscores lookups
- And more!

## Installation

Currently this is only installable via cargo. **Requires Rust 1.46 or higher**.

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

### Calculate drop rate

If you're going for a pet with a `1/5000` drop rate and you want to know the odds of getting it in the first 1000 kills:

```
osrs calc drop -p 1/5000 -n 1000
```

Or if you want to know how many kills you need to have a 50% chance of getting it:

```
osrs calc drop
```

### Calculate XP to a level

```
osrs calc xp --from-xp 100000 --to-lvl 80
osrs calc xp --from-lvl 50 --to-lvl 60
osrs calc xp --player <username> --skill smithing --to-lvl 90
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

### And more!
