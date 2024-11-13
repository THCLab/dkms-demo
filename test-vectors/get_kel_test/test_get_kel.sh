alias cli="./target/release/dkms-dev-cli"

cli init --alias alice -c "./scripts/get_kel_test/alice_config.yaml"

cli init -a bob -c "./scripts/get_kel_test/bobs_config.yaml"

cli oobi get -a bob > boboobi.json 

echo "\nLocal bob's KEL: "
cli kel get --alias bob

INFO=$(cli info -a bob)
echo $INFO
BOB_ID=$(echo $INFO | jq '.id' | tr -d '"')
echo $BOB_ID

echo "\nBob's KEL from watcher: "
BOB_OOBI=$(cli oobi get -a bob)
WATCHER_OOBI='{"eid":"BF2t2NPc1bwptY1hYV0YCib1JjQ11k9jtuaZemecPF5b","scheme":"http","url":"http://172.17.0.1:3235/"}'
cli kel get -i $BOB_ID --oobi $BOB_OOBI -w $WATCHER_OOBI

rm boboobi.json