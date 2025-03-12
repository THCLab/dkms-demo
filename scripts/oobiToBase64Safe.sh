#!/bin/bash

# Check if input is provided via a file argument or stdin
if [ "$#" -eq 1 ] && [ -f "$1" ]; then
    INPUT=$(cat "$1")
elif [ -p /dev/stdin ]; then
    INPUT=$(cat -)
else
    echo "Usage: $0 <input_json_file> OR echo '<json>' | $0"
    exit 1
fi

# Process the JSON input and encode each object to Base64 URL-safe
ENCODED_JSON=$(echo "$INPUT" | jq -c '.[]' | \
while read -r line; do
    echo -n "$line" | python3 -c 'import sys, base64; print(base64.urlsafe_b64encode(sys.stdin.buffer.read()).decode())'
done | jq -R -s 'split("\n") | map(select(. != ""))')

# Output the encoded JSON array
echo "$ENCODED_JSON"
