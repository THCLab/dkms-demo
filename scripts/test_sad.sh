alias keri-cli="./target/release/keri-cli"

echo -n '{"hello":"world","d":""}' > /tmp/said.json

keri-cli said sad -f /tmp/said.json