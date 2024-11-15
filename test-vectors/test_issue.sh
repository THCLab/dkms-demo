INPUT_DATA_DIR="./payloads"
dkms="./dkms"

if [ ! -e "$dkms" ]; then
    echo "$dkms bin not found. Please see README"
    exit 1
fi

if [ -z "$MESAGKESTO_ADDRESS" ]; then
    echo "MESAGKESTO_ADDRESS not set. Please set it to the address of the Mesagkesto service"
    exit 1
fi

$dkms init -a alice -c "./test-vectors/pamkeri/pamkeri_config.yaml"
$dkms tel incept -a alice

$dkms init -a bob -c "./test-vectors/pamkeri/pamkeri_config.yaml"

INFO=$($dkms whoami alice)
# echo $INFO
ALICE_ID=$(echo $INFO | jq '.id' | tr -d '"')
REGISTRY_ID=$(echo $INFO | jq '.registry' | tr -d '"')
# echo $REGISTRY_ID

TMP_ACDC='{"v":"ACDC10JSON000114_","d":"","i":"'$ALICE_ID'","ri":"'$REGISTRY_ID'","s":"schema","a":{"d":"ECk4Bn6rrC9G0mBJw0gy-DYv_glqBEuEwkVFWiwz-4sd","a":{"number":"123456789"}}}'
echo $TMP_ACDC > tmp_acdc.json
# Compute digest od ACDC
ACDC=$($dkms said sad -f tmp_acdc.json)
ACDC_DIGEST=$(echo $ACDC | jq '.d' | tr -d '"')

$dkms tel issue -a alice -c "$ACDC"
echo "\nACDC issued: $ACDC"

# Passing issued acdc from Alice to bob via mesagkesto
EXN=$($dkms mesagkesto exchange -a alice -r bob -c "$ACDC")

ALICE_OOBI=$($dkms oobi get -a alice)

# Parse JSON using jq and iterate through each element in the list
echo "$ALICE_OOBI" | jq -c '.[]' | while IFS= read -r element; do
	curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

bob_oobi=$($dkms oobi get -a bob)
# Parse JSON using jq and iterate through each element in the list
echo "$bob_oobi" | jq -c '.[]' | while IFS= read -r element; do
	curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

echo "\nSending issued ACDC to bob"
curl -s -X POST $MESAGKESTO_ADDRESS -d $(echo "$EXN")

PULL=$($dkms mesagkesto query -a bob)

# echo "\nPulling bob's messagebox:"
MESAGEBOX_RESPONSE=$(curl -s -X POST $MESAGKESTO_ADDRESS -d $(echo "$PULL"))
ACDC_FROM_MESAGEBOX=$(echo "$MESAGEBOX_RESPONSE" | jq -r '.messages[0]')
echo "\nBob gets ACDC from mesagebox: "
echo "$ACDC_FROM_MESAGEBOX"

# Verify the obtained ACDC
ALICE_OOBI=$($dkms oobi get -a alice) 
ALICE_TEL_OOBI=$($dkms tel oobi -a alice)

echo "\nBob verifies ACDC"
$dkms verify -a bob -m "$ACDC_FROM_MESAGEBOX" -o "$ALICE_OOBI" -o "$ALICE_TEL_OOBI"

rm tmp_acdc.json

