dkms="./dkms"

if [ ! -e "$dkms" ]; then
    echo "$dkms bin not found. Please see README"
    exit 1
fi

MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./payloads"

$dkms init -a alice 
$dkms tel incept -a alice

$dkms init -a bob -c "./scripts/pamkeri/pamkeri_config.yaml"

$dkms oobi get -a bob > boboobi.json 
$dkms oobi resolve -a alice -f boboobi.json

$dkms tel oobi -a alice > alice_tel.json

INFO=$($dkms info -a alice)
echo $INFO
ALICE_ID=$(echo $INFO | jq '.id' | tr -d '"')
REGISTRY_ID=$(echo $INFO | jq '.registry' | tr -d '"')
echo $REGISTRY_ID

TMP_ACDC='{"v":"ACDC10JSON000114_","d":"","i":"'$ALICE_ID'","ri":"'$REGISTRY_ID'","s":"schema","a":{"d":"ECk4Bn6rrC9G0mBJw0gy-DYv_glqBEuEwkVFWiwz-4sd","a":{"number":"123456789"}}}'
echo $TMP_ACDC > tmp_acdc.json
# Compute digest od ACDC
ACDC=$($dkms said sad -f tmp_acdc.json)
ACDC_DIGEST=$(echo $ACDC | jq '.d' | tr -d '"')

$dkms tel issue -a alice -c "$ACDC"
echo "\nACDC issued: $ACDC"

EXN=$($dkms mesagkesto exchange -a alice -r bob -c "$ACDC")

ALICE_OOBI=$($dkms oobi get -a alice) 
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

PULL=$($dkms mesagkesto query -a bob)

echo "\nPulling bob's messagebox:"
curl -X POST $MESAGKESTO_ADDRESS -d $(echo "$PULL")

$dkms oobi resolve -a bob -f aliceoobi.json
$dkms oobi resolve -a  bob -f alice_tel.json

echo "\nQuering for TEL of ACDC\n"
echo $REGISTRY_ID
TEL_STATE=$($dkms tel query -a bob -i $ALICE_ID -r $REGISTRY_ID -s $ACDC_DIGEST)
echo $TEL_STATE

TEL_STATE=$($dkms tel query -a bob -i $ALICE_ID -r $REGISTRY_ID -s $ACDC_DIGEST)
echo $TEL_STATE


case "$TEL_STATE" in 
  *Issued*)
	echo "ACDC is valid";;
  *) echo "Error: Unexpected TEL state: $TEL_STATE"
esac

rm boboobi.json
rm alice_tel.json
rm aliceoobi.json
rm tmp_acdc.json
