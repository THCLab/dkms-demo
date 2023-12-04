# ambient-infrastructure

## Repository structure

- `config` - folder with configuration files for docker compose,
- `events` - pregenerated keri events,
- `scripts` - folder with scripts, that tests infrastructure,
- `keri-cli` - rust project that provides binary for keri events generation.

## Usage

Run `docker compose up` to start simple network of 3 Witnesses, 1 Watcher and 1 Mesagkesto.
To build rust binary, run `cargo build --release`. Then, scripts from `scripts` folder can be run.