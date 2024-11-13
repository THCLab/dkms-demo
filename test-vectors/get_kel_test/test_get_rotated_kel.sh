alias dkms-dev-cli="./target/release/dkms-dev-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./payloads"
WATCHER_OOBI='{"eid":"BF2t2NPc1bwptY1hYV0YCib1JjQ11k9jtuaZemecPF5b","scheme":"http","url":"http://172.17.0.1:3235/"}'

dkms-dev-cli init -a alice -c "./scripts/get_kel_test/alice_config.yaml"

dkms-dev-cli init -a bob -c "./scripts/get_kel_test/bobs_config.yaml"

dkms-dev-cli oobi get -a bob > boboobi.json 
dkms-dev-cli oobi resolve -a alice -f boboobi.json

INFO=$(dkms-dev-cli info -a bob)
BOB_ID=$(echo $INFO | jq '.id' | tr -d '"')
BOB_OOBI=$(dkms-dev-cli oobi get -a bob)
# echo $BOB_OOBI

echo "\nBob's KEL before rotation:"
dkms-dev-cli kel get -i $BOB_ID --oobi $BOB_OOBI -w $WATCHER_OOBI

dkms-dev-cli kel rotate -a bob

echo "\nBob's KEL after rotation:"
dkms-dev-cli kel get -i $BOB_ID --oobi $BOB_OOBI -w $WATCHER_OOBI

rm boboobi.json