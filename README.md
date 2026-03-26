# DKMS infrastructure

## Purpose

Demonstrate the practical usage of the [DKMS](https://dkms.colossi.network/) network consisting of DKMS Witnesses and Watchers, based on the KERI protocol.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) with Compose v2
- [`jq`](https://jqlang.github.io/jq/) (used in test scripts)
- [`dkms` CLI binary](https://github.com/THCLab/dkms-bin/releases) — download the binary for your platform and note the path

## Infrastructure

The demo spins up five services via Docker Compose:

| Service     | Container name    | Host port |
|-------------|-------------------|-----------|
| Witness 1   | keriox-witness1   | 3232      |
| Witness 2   | keriox-witness2   | 3233      |
| Witness 3   | keriox-witness3   | 3234      |
| Watcher     | keriox-watcher    | 3235      |
| Mesagkesto  | mesagkesto        | 3236      |

### Step 1: Configure the host IP

Services advertise their `public_url` so that external clients can reach them.
By default this is `172.17.0.1` (the Docker bridge gateway on Linux).

**On macOS / Windows** (Docker Desktop) the gateway address is different, so
you need to run the configuration script once before starting Docker Compose:

```bash
cd infrastructure

# Copy the example env file and edit HOST_IP if needed
cp .env.example .env
# e.g. on macOS: HOST_IP=host.docker.internal

# Generate config files from templates
./configure.sh
```

On Linux the default `172.17.0.1` works out of the box — you can skip
`configure.sh` unless you changed `HOST_IP`.

> To find your Docker bridge gateway on any OS:
> ```bash
> docker network inspect bridge | grep Gateway
> ```

### Step 2: Start the network

```bash
cd infrastructure
docker compose up
```

To run in the background:

```bash
docker compose up -d
```

To stop and remove containers:

```bash
docker compose down
```

To also remove persisted data volumes:

```bash
docker compose down -v
```

## Connect to the infrastructure

Interact with the running infrastructure using one of the following client options:

1. **Command Line Interface (CLI):** [`dkms-bin`](https://github.com/THCLab/dkms-bin)
2. **Rust API:** [Keriox Controller Component](https://github.com/THCLab/keriox/tree/master/components/controller)
3. **JavaScript (Node.js):** [DKMS Bindings](https://github.com/THCLab/dkms-bindings/tree/master/bindings/node.js)

## Tests

See [`test-vectors/README.md`](test-vectors/README.md) for full instructions on
running the test scripts.
