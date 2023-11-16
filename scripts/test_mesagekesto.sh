WITNESS_ADDRESS="http://localhost:3232"
MESAGKESTO_ADDRESS="http://localhost:8080"
INPUT_DATA_DIR="./test_data_generator/generated"

# Setup signing identifier
curl -X POST "$WITNESS_ADDRESS"/process -d $(cat "$INPUT_DATA_DIR"/identifier1/kel)
FIRST_EXPECTED_SAID=$(cat "$INPUT_DATA_DIR"/identifier1/kel | cut -c 1-345 | jq -r '.d')
SECOND_EXPECTED_SAID=$(cat "$INPUT_DATA_DIR"/identifier1/kel | cut -c 438-751 | jq -r '.d')

# Publish tel of issued ACDC
curl -X POST "$WITNESS_ADDRESS"/process/tel -d $(cat "$INPUT_DATA_DIR"/tel)

echo $FIRST_EXPECTED_SAID
echo $SECOND_EXPECTED_SAID

MAILBOX=$(curl -X POST "$WITNESS_ADDRESS"/query -d $(cat "$INPUT_DATA_DIR"/identifier1/mailbox_qry_0))

RECEIPTS=$MAILBOX | jq -r '.receipt'

FIRST_RECEIPT_SAID=$(echo $RECEIPTS | cut -c 1-145 | jq -r '.d')
SECOND_RECEIPT_SAID=$(echo $RECEIPTS | cut -c 282-427 | jq -r '.d')
echo $FIRST_RECEIPT_SAID
echo $SECOND_RECEIPT_SAID

# Setup signing identifier
curl -X POST "$WITNESS_ADDRESS"/process -d $(cat "$INPUT_DATA_DIR"/identifier2/kel)

curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d $(cat "$INPUT_DATA_DIR"/identifier1/oobi0)
curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d $(cat "$INPUT_DATA_DIR"/identifier1/oobi1)


# Send ACDC to verifing identifier
curl -X POST $MESAGKESTO_ADDRESS -d $(cat "$INPUT_DATA_DIR"/messagebox/exn)

# Query verifing identifier's mesagkesto
curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d $(cat "$INPUT_DATA_DIR"/identifier2/oobi0)
curl -X POST "$MESAGKESTO_ADDRESS"/resolve -d $(cat "$INPUT_DATA_DIR"/identifier2/oobi1)

curl -X POST $MESAGKESTO_ADDRESS -d $(cat "$INPUT_DATA_DIR"/messagebox/qry)
echo "\n"

# Verify gotten acdc
curl -X POST "$WITNESS_ADDRESS"/query/tel -d $(cat "$INPUT_DATA_DIR"/messagebox/tel_qry)


