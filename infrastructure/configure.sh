#!/usr/bin/env bash
# Generates infrastructure config files from templates.
# Run this script once before `docker compose up` whenever HOST_IP changes.
#
# Usage:
#   ./configure.sh                 # uses .env or .env.example defaults
#   HOST_IP=192.168.1.10 ./configure.sh
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="$SCRIPT_DIR/config"

# Load .env if present, otherwise fall back to .env.example defaults
if [ -f "$SCRIPT_DIR/.env" ]; then
    echo "Loading configuration from .env"
    set -a && source "$SCRIPT_DIR/.env" && set +a
elif [ -f "$SCRIPT_DIR/.env.example" ]; then
    echo "No .env found, loading defaults from .env.example"
    set -a && source "$SCRIPT_DIR/.env.example" && set +a
fi

# HOST_IP can also be passed as an environment variable directly
HOST_IP="${HOST_IP:-172.17.0.1}"

echo "Generating config files with HOST_IP=${HOST_IP}"
echo ""

for tmpl in "$CONFIG_DIR"/*.yml.tmpl; do
    out="${tmpl%.tmpl}"
    envsubst < "$tmpl" > "$out"
    echo "  Generated: $(basename "$out")"
done

echo ""
echo "Done. You can now run: docker compose up"
