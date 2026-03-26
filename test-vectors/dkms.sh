dkms=$DKMS_BIN_PATH

if [ ! -e "$dkms" ]; then
    echo "$dkms bin not found. Please see README"
    exit 1
fi

# Host IP where services are reachable from the host machine.
# On Linux this is the Docker bridge gateway (default: 172.17.0.1).
# On macOS/Windows use host.docker.internal or your machine's LAN IP.
# Override by setting HOST_IP before running a test script.
HOST_IP=${HOST_IP:-172.17.0.1}

WITNESS1_URL=${WITNESS1_URL:-http://${HOST_IP}:3232}
WITNESS2_URL=${WITNESS2_URL:-http://${HOST_IP}:3233}
WITNESS3_URL=${WITNESS3_URL:-http://${HOST_IP}:3234}
WATCHER_URL=${WATCHER_URL:-http://${HOST_IP}:3235}
MESAGKESTO_ADDRESS=${MESAGKESTO_ADDRESS:-http://${HOST_IP}:3236}
