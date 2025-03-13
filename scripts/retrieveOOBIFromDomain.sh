#!/bin/bash

# Check if a domain name is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <domain_name>"
    exit 1
fi

DOMAIN="$1"

# Retrieve TXT records and extract those containing 'oobi='
INPUT=$(dig TXT "$DOMAIN" +short | grep 'oobi=')

# Ensure input is not empty
if [ -z "$INPUT" ]; then
    echo "No TXT records containing 'oobi=' found for $DOMAIN"
    exit 1
fi

# Extract and decode base64 URL-safe encoded values
JSON_ARRAY=$(echo "$INPUT" | grep -o 'oobi=[^ ]*' | cut -d'=' -f2 | \
while read -r encoded; do
    # Fix Base64 padding
    padded_encoded=$(echo -n "$encoded" | awk '{ while (length($0) % 4) $0=$0"="; print }')
    echo -n "$padded_encoded" | python3 -c 'import sys, base64, json; 
try:
    decoded = base64.urlsafe_b64decode(sys.stdin.buffer.read()).decode()
    print(json.dumps(json.loads(decoded), separators=(",", ":")))
except Exception as e:
    print("{}")'
done | jq -s -c '.')

# Output the final JSON array
echo "$JSON_ARRAY"
