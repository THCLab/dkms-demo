curl -X POST "http://localhost:3232/process" -d $(cat "./test_data_generator/generated/identifier1/kel")
FIRST_EXPECTED_SAID=$(cat "./test_data_generator/generated/identifier1/kel" | cut -c 1-345 | jq -r '.d')
SECOND_EXPECTED_SAID=$(cat "./test_data_generator/generated/identifier1/kel" | cut -c 438-751 | jq -r '.d')

curl -X POST "http://localhost:3232/process/tel" -d $(cat "./test_data_generator/generated/tel")

echo $FIRST_EXPECTED_SAID
echo $SECOND_EXPECTED_SAID

MAILBOX=$(curl -X POST "http://localhost:3232/query" -d $(cat "./test_data_generator/generated/identifier1/mailbox_qry_0"))
echo $MAILBOX

RECEIPTS=$MAILBOX | jq -r '.receipt'

FIRST_RECEIPT_SAID=$(echo $RECEIPTS | cut -c 1-145 | jq -r '.d')
SECOND_RECEIPT_SAID=$(echo $RECEIPTS | cut -c 282-427 | jq -r '.d')
echo $FIRST_RECEIPT_SAID
echo $SECOND_RECEIPT_SAID

cat "./test_data_generator/generated/exn"
curl -X POST "http://localhost:3236/" -d $(cat "./test_data_generator/generated/exn")

# curl -X POST "http://localhost:3236/" -d $(cat "./test_data_generator/generated/qry")