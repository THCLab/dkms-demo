alias dkms-dev-cli="./target/release/dkms-dev-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./payloads"

dkms-dev-cli init -a alice -c "./scripts/rotate_witness/alice_config.yaml"

dkms-dev-cli kel rotate -a alice -c "./scripts/rotate_witness/rotation_config.yaml"

INFO=$(dkms-dev-cli info -a alice)
ALICE_ID=$(echo $INFO | jq '.id' | tr -d '"')
echo "\n$(dkms-dev-cli kel get -a alice -i $ALICE_ID)"
