# Test vectors

Integration tests for the DKMS infrastructure using the `dkms` CLI.

## Prerequisites

- The infrastructure must be running — see the [main README](../README.md)
- [`dkms` binary](https://github.com/THCLab/dkms-bin/releases) downloaded for your platform
- `jq` installed (`apt install jq` / `brew install jq`)
- `curl` installed (usually pre-installed)

## Environment variables

| Variable           | Required | Default             | Description |
|--------------------|----------|---------------------|-------------|
| `DKMS_BIN_PATH`    | Yes      | —                   | Absolute path to the `dkms` binary |
| `HOST_IP`          | No       | `172.17.0.1`        | IP / hostname where services are reachable from the host. On Linux this is the Docker bridge gateway. On macOS/Windows use `host.docker.internal` or your LAN IP. |
| `WITNESS1_URL`     | No       | `http://$HOST_IP:3232` | Override URL for witness 1 |
| `WITNESS2_URL`     | No       | `http://$HOST_IP:3233` | Override URL for witness 2 |
| `WITNESS3_URL`     | No       | `http://$HOST_IP:3234` | Override URL for witness 3 |
| `WATCHER_URL`      | No       | `http://$HOST_IP:3235` | Override URL for the watcher |
| `MESAGKESTO_ADDRESS` | No     | `http://$HOST_IP:3236` | Override URL for the Mesagkesto service |

Setting `HOST_IP` is enough in most cases — all service URLs are derived from it automatically.

## Running tests

All scripts must be run **from the repository root**:

```bash
export DKMS_BIN_PATH=/path/to/dkms

# Optional: override host IP (Linux default is 172.17.0.1)
# export HOST_IP=host.docker.internal   # macOS / Windows

bash test-vectors/test_sign.sh
```

## Available tests

| Script | Description |
|--------|-------------|
| `test_sign.sh` | Sign and verify an arbitrary message with a single identifier |
| `test_multisig.sh` | Create a multisig group, sign a message, and verify the combined signature |
| `test_multisig_issue.sh` | Create a multisig group and issue an ACDC credential collaboratively |
| `test_issue.sh` | Issue, exchange, and verify an ACDC credential between two identifiers via Mesagkesto |
| `test_sad.sh` | Calculate Self-Addressing Data (SAID) hashes |
| `get_kel_test/test_get_kel.sh` | Retrieve a Key Event Log (KEL) locally and via the watcher |
| `get_kel_test/test_get_rotated_kel.sh` | Retrieve a KEL after a key rotation event |
| `rotate_witness/rotate_witness.sh` | Rotate an identifier's witness configuration using a YAML config file |

### Example: full sign/verify flow

```bash
export DKMS_BIN_PATH=/path/to/dkms
bash test-vectors/test_sign.sh
```

### Example: issue credential via Mesagkesto

`test_issue.sh` requires the Mesagkesto service to be reachable.
`MESAGKESTO_ADDRESS` defaults to `http://$HOST_IP:3236` so it is already set
if you have `HOST_IP` configured correctly:

```bash
export DKMS_BIN_PATH=/path/to/dkms
bash test-vectors/test_issue.sh
```

Or override explicitly:

```bash
export DKMS_BIN_PATH=/path/to/dkms
export MESAGKESTO_ADDRESS=http://172.17.0.1:3236
bash test-vectors/test_issue.sh
```

## Cleaning up state between runs

Each test creates local identifier state under the working directory.
Remove it between runs to start fresh:

```bash
rm -rf ~/.dkms   # default dkms state directory (confirm with your binary version)
```
