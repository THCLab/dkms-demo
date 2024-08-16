alias cli="./target/release/dkms-dev-cli"

cli init --alias alice -c "./scripts/get_kel_test/alice_config.yaml"

cli init -a bob -c "./scripts/get_kel_test/bobs_config.yaml"

cli oobi get -a bob > boboobi.json 

cli kel get --alias bob

INFO=$(cli info -a bob)
echo $INFO
BOB_ID=$(echo $INFO | jq '.id' | tr -d '"')
echo $BOB_ID

BOB_OOBI=$(cli oobi get -a bob)
cli kel get -i $BOB_ID --oobi $BOB_OOBI

# cli verify --alias alias -p payload --wit_oobi witnessoobi 


# cli kel get --alias bob

# cli info --alias chloe 
# { i: "krzak", "...", witnesses: "", watchers: "" }

rm boboobi.json