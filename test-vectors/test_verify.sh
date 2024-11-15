dkms="./dkms"

if [ ! -e "$dkms" ]; then
    echo "$dkms bin not found. Please see README"
    exit 1
fi

$dkms init -a alice -c "./scripts/infra_config.yaml"
# $dkms init -a alice -c "./scripts/pamkeri/pamkeri_config.yaml"
$dkms tel incept -a alice

SIGNED=$($dkms sign -a alice -d '{"hello":"world"}')
echo "Signed message: \n"
echo $SIGNED


$dkms init -a bob -c "./scripts/infra_config.yaml"
# $dkms init -a bob -c "./scripts/pamkeri/pamkeri_config.yaml"

ALICE_OOBI=$($dkms oobi get -a alice) 
$dkms verify -a bob -m "$SIGNED" -o "$ALICE_OOBI"

# WRONG='{"hello":"world"}-FABELRfU-deAvyeTz3v-PsysZrwqWC52_piI712KboxoyJS0AAAAAAAAAAAAAAAAAAAAAAAELRfU-deAvyeTz3v-PsysZrwqWC52_piI712KboxoyJS-AABAAB9JZdo8PXF8fI4OUw3qoTf66lElBDPk-YtYlaWcdFOkNI3yTX-OgnhLDVLsGEpDKxDbOrmKgeF8vvv6k2TctQf'
# $dkms verify -a alice -m "$WRONG"

# ======================================================


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

ALICE_OOBI=$($dkms oobi get -a alice) 
ALICE_TEL_OOBI=$($dkms tel oobi -a alice)

echo Bob verifies ACDC
$dkms verify -a bob -m "$ACDC" -o "$ALICE_OOBI" -o "$ALICE_TEL_OOBI"