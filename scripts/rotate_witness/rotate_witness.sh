alias keri-cli="./target/release/keri-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./payloads"

keri-cli init -a alice -c "./scripts/rotate_witness/alice_config.yaml"

keri-cli kel rotate -a alice -c "./scripts/rotate_witness/rotation_config.yaml"

INFO=$(keri-cli info -a alice)
ALICE_ID=$(echo $INFO | jq '.id' | tr -d '"')
echo "\n$(keri-cli kel get -a alice -i $ALICE_ID)"
