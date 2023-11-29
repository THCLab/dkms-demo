alias keri-cli="./target/release/keri-cli"
MESAGKESTO_ADDRESS="http://localhost:3236"
INPUT_DATA_DIR="./events"

keri-cli init -a alice
keri-cli init -a bob 
keri-cli oobi -a bob > boboobi.json 
keri-cli resolve -a alice -f boboobi.json 
keri-cli issue -a alice -c '{"d":"EK2345cpZZ_8DHSY2ItqOmAuTideCPHqLgVaKHNZZ1s7","m":"hello"}'

EXN=$(keri-cli exchange -a alice -r bob -c "$(cat "$INPUT_DATA_DIR"/acdc)")
echo "$EXN"
echo "\n"

keri-cli oobi -a alice > aliceoobi.json 

# Read the JSON file
json=$(cat "aliceoobi.json")

# Parse JSON using jq and iterate through each element in the list
echo "$json" | jq -c '.[]' | while IFS= read -r element; do
	curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

# curl -X POST $MESAGKESTO_ADDRESS -d $(echo "$EXN")
