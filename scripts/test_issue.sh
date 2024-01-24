
alias keri-cli="./target/release/keri-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./payloads"

keri-cli init -a alice
keri-cli tel incept -a alice

keri-cli init -a bob 

keri-cli oobi get -a bob > boboobi.json 
keri-cli oobi resolve -a alice -f boboobi.json

INFO=$(keri-cli info -a alice)
ALICE_ID=$(echo $INFO | jq '.id' | tr -d '"')
REGISTRY_ID=$(echo $INFO | jq '.registry' | tr -d '"')

TMP_ACDC='{"v":"ACDC10JSON000114_","d":"","i":"'$ALICE_ID'","ri":"'$REGISTRY_ID'","s":"schema","a":{"d":"ECk4Bn6rrC9G0mBJw0gy-DYv_glqBEuEwkVFWiwz-4sd","a":{"number":"123456789"}}}'
echo $TMP_ACDC > tmp_acdc.json
# Compute digest od ACDC
ACDC=$(keri-cli said sad -f tmp_acdc.json)
ACDC_DIGEST=$(echo $ACDC | jq '.d' | tr -d '"')

keri-cli tel issue -a alice -c "$ACDC"
echo "\nACDC issued: $ACDC"

EXN=$(keri-cli mesagkesto exchange -a alice -r bob -c "$ACDC")

ALICE_OOBI=$(keri-cli oobi get -a alice) 
echo $ALICE_OOBI > aliceoobi.json

# Parse JSON using jq and iterate through each element in the list
echo "$ALICE_OOBI" | jq -c '.[]' | while IFS= read -r element; do
	curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

bob_oobi=$(cat "boboobi.json")
# Parse JSON using jq and iterate through each element in the list
echo "$bob_oobi" | jq -c '.[]' | while IFS= read -r element; do
	curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

echo "\nSending issued acdc to bob"
curl -X POST $MESAGKESTO_ADDRESS -d $(echo "$EXN")

PULL=$(keri-cli mesagkesto query -a bob)

echo "\nPulling bob's messagebox:"
curl -X POST $MESAGKESTO_ADDRESS -d $(echo "$PULL")

keri-cli oobi resolve -a bob -f aliceoobi.json

echo "\nQuering for TEL of ACDC"
TEL_STATE=$(keri-cli tel query -a bob -i $ALICE_ID -r $REGISTRY_ID -s $ACDC_DIGEST)

case "$TEL_STATE" in 
  *Issued*)
	echo "ACDC is valid";;
  *) echo "Error: Unexpected TEL state: $TEL_STATE"
esac

rm boboobi.json
rm aliceoobi.json
rm tmp_acdc.json