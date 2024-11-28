# Ambient infrastructure

## Purpose

Demonstrate the practical usage of the [DKMS](https://dkms.colossi.network/) network consisting of DKMS Witnesses and Watchers, based on the KERI protocol.

## Usage

### Step 1: Run the Infrastructure
Navigate to the `infrastructure` directory and start the network using Docker:

```bash
docker compose up
```
This will set up a simple network consisting of:

- 3 Witnesses
- 1 Watcher
- 1 Mesagkesto

### Step 2: Connect to the Infrastructure
Interact with the running infrastructure using one of the following client (controller) options:

1. **Command Line Interface (CLI):**
   Utilize [`dkms-bin`](https://github.com/THCLab/dkms-bin) for CLI-based interaction.

2. **API Client:**
   - **Rust:** The Rust-based API client is available in the [Keriox Controller Component](https://github.com/THCLab/keriox/tree/master/components/controller).
   - **JavaScript (Node.js):** The Node.js API client is provided in the [DKMS Bindings](https://github.com/THCLab/dkms-bindings/tree/master/bindings/node.js).


## Tests

Navigate to the `test-vectors` dir and run the scripts.
