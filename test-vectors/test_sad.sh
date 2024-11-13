dkms="./dkms"

if [ ! -e "$dkms" ]; then
    echo "$dkms bin not found. Please see README"
    exit 1
fi

echo -n '{"hello":"world","d":""}' > /tmp/said.json

$dkms said sad -f /tmp/said.json
