alias keri-cli="./target/release/keri-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./events"

keri-cli init -a alice
keri-cli init -a bob 

keri-cli oobi -a bob > boboobi.json 
keri-cli resolve -a alice -f boboobi.json 

ACDC=$(cat "$INPUT_DATA_DIR"/acdc)

keri-cli issue -a alice -c "$ACDC"

EXN=$(keri-cli exchange -a alice -r bob -c "$ACDC")
# echo "$EXN"
# echo "\n"

ALICE_OOBI=$(keri-cli oobi -a alice) 

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

PULL=$(keri-cli pull -a bob)
# echo "$PULL"
echo "\n"

curl -X POST $MESAGKESTO_ADDRESS -d $(echo "$PULL")

rm boboobi.json
