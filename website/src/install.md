---
layout: default
---

# Installation

There are a few ways to install the OSRS CLI:

- From binary (the easiest)
- From [crates.io](https://crates.io)
- From source

## From Binary

Go to [the latest release](https://github.com/LucasPickering/osrs-cli/releases/latest) and download the release for your platform (for Windows, you probably want the `msvc` version). Extract the archive and copy the executable file (either `osrs` or `osrs.exe`) to a directory in your `PATH`.

If you want to download a newer version of the app, you'll have to follow these same instructions again with the newest release.

## From crates.io

You can install this tool from [crates.io](https://crates.io/crates/osrs-cli) if you have a working Rust toolchain installed. [See here](https://doc.rust-lang.org/cargo/getting-started/installation.html) for setup instructions. Then run:

```sh
cargo install osrs-cli
```

This will install the latest version of the CLI to your path automatically. The advantage of this approach is you can easily download new versions using the same command.

## From Source

This is the most involved option, and you probably only want it if you really want to use an unreleased version of the tool. You'll need a working Rust toolchain for this (see instructions in the "crates.io" section above). Then, clone [the repo](https://github.com/LucasPickering/osrs-cli) and run:

```sh
cargo install --path .
```
