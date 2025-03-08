# This test showcase how to issue, exchange and verify ACDC between Ewa and Jan using Mesagkesto service

. test-vectors/dkms.sh

if [ -z "$MESAGKESTO_ADDRESS" ]; then
    echo "MESAGKESTO_ADDRESS not set. Please set it to the address of the Mesagkesto service"
    exit 1
fi
echo -e "\n==== Generate Ewa key ====\n"
$dkms identifier init -a ewa --witness-url http://172.17.0.1:3234 --watcher-url http://172.17.0.1:3235

echo -e "\n==== Generate Jan key ==== \n"
$dkms identifier init -a jan --witness-url http://172.17.0.1:3232 --watcher-url http://172.17.0.1:3235

INFO=$($dkms identifier info ewa)
EWA_ID=$(echo $INFO | jq '.id' | tr -d '"')

TMP_ACDC='{"v":"ACDC10JSON000114_","d":"","i":"'$EWA_ID'","ri":"","s":"schema","a":{"d":"ECk4Bn6rrC9G0mBJw0gy-DYv_glqBEuEwkVFWiwz-4sd","a":{"number":"123456789"}}}'
ISSUED_ACDC=$($dkms data issue -a ewa -m "$TMP_ACDC")
echo -e "\nACDC issued: $ISSUED_ACDC"

echo -e "\n==== Passing issued ACDC from Alice to Bob via mesagkesto ====\n"
EXN=$($dkms mesagkesto exchange -a ewa -r jan -c "$ISSUED_ACDC")

EWA_OOBI=$($dkms identifier oobi get -a ewa)

# Provide Mesagkesto with the Alice's OOBI (witness)
# Parse JSON using jq and iterate through each element in the list
echo -e "\n==== Sending Alice's OOBI to mesagkesto ====\n"
echo "$EWA_OOBI" | jq -c '.[]' | while IFS= read -r element; do
    echo -e "Element: $element"
	curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

jan_oobi=$($dkms identifier oobi get -a jan)
# Parse JSON using jq and iterate through each element in the list
echo -e "\n==== Sending Bob's OOBI to mesagkesto ====\n"
echo "$jan_oobi" | jq -c '.[]' | while IFS= read -r element; do
	# echo -e "Element: $element"
    curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d "$element"
done

echo -e "\n==== Sending issued ACDC to jan ===="
curl -s -X POST $MESAGKESTO_ADDRESS -d $(echo "$EXN")

PULL=$($dkms mesagkesto query -a jan)

MESAGEBOX_RESPONSE=$(curl -s -X POST $MESAGKESTO_ADDRESS -d $(echo "$PULL"))
ACDC_FROM_MESAGEBOX=$(echo "$MESAGEBOX_RESPONSE" | jq -r '.messages[0]')
echo -e "\n==== Bob gets ACDC from mesagebox: "
echo "$ACDC_FROM_MESAGEBOX"

# Verify the obtained ACDC
EWA_OOBI=$($dkms identifier oobi get -a ewa)
EWA_TEL_OOBI=$($dkms log tel oobi -a ewa)

echo -e "\n==== Bob verifies ACDC ===="
$dkms data verify -a jan -m "$ACDC_FROM_MESAGEBOX" -o "$EWA_OOBI" -o "$EWA_TEL_OOBI"
