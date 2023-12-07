alias keri-cli="./target/release/keri-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./payloads"

keri-cli init -a alice
keri-cli init -a bob 

keri-cli oobi get -a bob > boboobi.json 
keri-cli oobi resolve -a alice -f boboobi.json 

ACDC=$(cat "$INPUT_DATA_DIR"/acdc)

keri-cli issue -a alice -c "$ACDC"

EXN=$(keri-cli mesagkesto exchange -a alice -r bob -c "$ACDC")

ALICE_OOBI=$(keri-cli oobi get -a alice) 

# Parse JSON using jq and iterate through each element in the list
echo "$ALICE_OOBI" | jq -c '.[]' | while IFS= read -r element; do
	curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

# Read the JSON file
json=$(cat "boboobi.json")

# Parse JSON using jq and iterate through each element in the list
echo "$json" | jq -c '.[]' | while IFS= read -r element; do
	curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

curl -X POST $MESAGKESTO_ADDRESS -d $(echo "$EXN")

PULL=$(keri-cli mesagkesto query -a bob)
echo "\nPulling bob's messagebox:"

curl -X POST $MESAGKESTO_ADDRESS -d $(echo "$PULL")

rm boboobi.json
