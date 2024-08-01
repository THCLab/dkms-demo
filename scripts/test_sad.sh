alias dkms-dev-cli="./target/release/dkms-dev-cli"

echo -n '{"hello":"world","d":""}' > /tmp/said.json

dkms-dev-cli said sad -f /tmp/said.json