# Ambient infrastructure

## Purpose

Demonstrate the practical usage of the [DKMS](https://dkms.colossi.network/) network consisting of KERI Witnesses and Watchers.

## Usage

1. Run infrastructure using `docker compose up`. It starts a simple network of 3 Witnesses, 1 Watcher, and 1 Mesagkesto.
2. Build CLI binary, run `cargo build --release`.

### Tests

Navigate to the `scripts` dir and run the scripts.

# Repository structure

- `config` - folder with configuration files for `docker compose`,
- `events` - pre-generated keri events,
- `scripts` - folder with scripts that test infrastructure,
- `dkms-dev-cli` - Rust-based project providing a binary for generating KERI events.
