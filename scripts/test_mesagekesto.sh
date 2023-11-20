WITNESS_ADDRESS="http://localhost:3232"
MESAGKESTO_ADDRESS="http://localhost:3236"
INPUT_DATA_DIR="./test_data_generator/generated"

# Setup signing identifier
curl -X POST "$WITNESS_ADDRESS"/process -d $(cat "$INPUT_DATA_DIR"/identifier1/kel)
ICP=$(cat "$INPUT_DATA_DIR"/identifier1/kel | cut -c 1-345)
IXN=$(cat "$INPUT_DATA_DIR"/identifier1/kel | cut -c 438-751)
echo $IXN
EXPECTED_ISSUER_ID=$(echo $ICP | jq -r '.i')
FIRST_EXPECTED_SAID=$(echo $ICP | jq -r '.d')
SECOND_EXPECTED_SAID=$(echo $IXN | jq -r '.d')

# Publish tel of issued ACDC
curl -X POST "$WITNESS_ADDRESS"/process/tel -d $(cat "$INPUT_DATA_DIR"/tel)

MAILBOX=$(curl -X POST "$WITNESS_ADDRESS"/query -d $(cat "$INPUT_DATA_DIR"/identifier1/mailbox_qry_0))

RECEIPTS=$MAILBOX | jq -r '.receipt'

FIRST_RECEIPT_SAID=$(echo $RECEIPTS | cut -c 1-145 | jq -r '.d')
SECOND_RECEIPT_SAID=$(echo $RECEIPTS | cut -c 282-427 | jq -r '.d')

if [ "$FIRST_EXPECTED_SAID" = "$FIRST_RECEIPT_SAID" ] && [" $SECOND_EXPECTED_SAID" = "$SECOND_RECEIPT_SAID" ]
then
	echo "ERROR: Wrong response"
    exit
fi 

# Setup signing identifier
curl -X POST "$WITNESS_ADDRESS"/process -d $(cat "$INPUT_DATA_DIR"/identifier2/kel)

curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d $(cat "$INPUT_DATA_DIR"/identifier1/oobi0)
curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d $(cat "$INPUT_DATA_DIR"/identifier1/oobi1)


# Send ACDC to verifing identifier
curl -X POST $MESAGKESTO_ADDRESS -d $(cat "$INPUT_DATA_DIR"/messagebox/exn)

# Query verifing identifier's mesagkesto
curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d $(cat "$INPUT_DATA_DIR"/identifier2/oobi0)
curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d $(cat "$INPUT_DATA_DIR"/identifier2/oobi1)

MESSAGEBOX_RESPONSE=$(curl -X POST $MESAGKESTO_ADDRESS -d $(cat "$INPUT_DATA_DIR"/messagebox/qry))
echo "mesagebox response: \n"
echo $MESSAGEBOX_RESPONSE

# Verify gotten acdc
TEL=$(curl -X POST "$WITNESS_ADDRESS"/query/tel -d $(cat "$INPUT_DATA_DIR"/messagebox/tel_qry))
VCP=$(echo $TEL | cut -c 1-224)
echo "\n"
echo $VCP
REGISTRY_ID=$(echo $VCP | jq -r '.i' )
REGISTRY_SN=$(echo $VCP | jq -r '.s' )
REGISTRY_DIGEST=$(echo $VCP | jq -r '.d' )

ISSUER=$(echo $VCP | jq -r '.ii')
echo $ISSUER
# Check issuer identifier
if [ "$EXPECTED_ISSUER_ID" != "$ISSUER" ]
then
	echo "ERROR: Wrong issuer identifier, should be: "
	echo $EXPECTED_ISSUER_ID
    exit
fi

BIS=$(echo $TEL | cut -c 297-650)
echo "\n"
echo $BIS
ISSUER=$(echo $BIS | jq -r '.ii')
# Check issuer identifier
if [ "$EXPECTED_ISSUER_ID" != "$ISSUER" ]
then
	echo "ERROR: Wrong issuer identifier, should be: "
	echo $EXPECTED_ISSUER_ID
    exit
fi

REGISTRY_ANCHOR=$(echo $BIS | jq -r '.ra')
echo "\n"
echo $REGISTRY_ANCHOR

if [ $(echo $REGISTRY_ANCHOR | jq -r '.i') != $REGISTRY_ID ] | [ $(echo $REGISTRY_ANCHOR | jq -r '.s') != $REGISTRY_SN ] | [ $(echo $REGISTRY_ANCHOR | jq -r '.d') != $REGISTRY_DIGEST ]
then
	echo "ERROR: Wrong registry anchor"
	exit
fi
