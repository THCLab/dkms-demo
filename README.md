# Purpose

Demonstrate the practical usage of the [DKMS](https://dkms.colossi.network/) network consisting of KERI Witnesses and Watchers.

## Usage

Run `docker compose up` to start a simple network of 3 Witnesses, 1 Watcher, and 1 Mesagkesto.
To build a Rust binary, run `cargo build --release`. Then, you can run scripts from the `scripts` folder.

# Repository structure

- `config` - folder with configuration files for `docker compose`,
- `events` - pre-generated keri events,
- `scripts` - folder with scripts that test infrastructure,
- `dkms-dev-cli` - Rust-based project providing a binary for generating KERI events.
