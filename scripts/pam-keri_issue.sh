alias keri-cli="./target/release/keri-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"

keri-cli init -a alice -c "./scripts/pamkeri_config.yaml"
keri-cli tel incept -a alice

keri-cli init -a bob -c "./scripts/pamkeri_config.yaml"
BOB_INFO=$(keri-cli info -a bob)
BOB_ID=$(echo $BOB_INFO | jq '.id' | tr -d '"')

INFO=$(keri-cli info -a alice)
ALICE_ID=$(echo $INFO | jq '.id' | tr -d '"')
REGISTRY_ID=$(echo $INFO | jq '.registry' | tr -d '"')

TMP_ACDC='{"v":"ACDC10JSON000133_","d":"EG9kClwtClse9J7eaQgByM7prbx1NDmEdqHT-HgCeSpf","i":"'$ALICE_ID'","ri":"'$REGISTRY_ID'","s":"EPtdQc35vLxszRMw3-uyBg3JY0_7uQ0xqZlkCfD0VSB5","a":{"i":"'$BOB_ID'","d":"EHuwAoa8v25gJHrGntWyoKd4h_VAOzLaT4R8OaLtEInE","a":{"passed":true}}}'

echo $TMP_ACDC > tmp_acdc.json
# Compute digest od ACDC
ACDC=$(keri-cli said sad -f tmp_acdc.json)
ACDC_DIGEST=$(echo $ACDC | jq '.d' | tr -d '"')

keri-cli tel issue -a alice -c "$ACDC"

echo "\nACDC issued: $ACDC"

ALICE_OOBI=$(keri-cli oobi get -a alice) 
OOBI1=$(echo "$ALICE_OOBI" | jq -c '.[0]') 
OOBI2=$(echo "$ALICE_OOBI" | jq -c '.[1]')

EXN=$(keri-cli mesagkesto exchange -a alice -r mach -c "$OOBI2$ACDC")
echo "\n exn: $EXN"

# # echo $OOBI2 > aliceoobi.json

curl -XPOST -d $(echo $OOBI1) "$MESAGKESTO_ADDRESS"/resolve 
curl -XPOST -d $(echo $OOBI2) "$MESAGKESTO_ADDRESS"/resolve

curl -XPOST -d $(echo $EXN) "$MESAGKESTO_ADDRESS"
