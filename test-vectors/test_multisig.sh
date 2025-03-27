. test-vectors/dkms.sh

echo "\n==== Generate Ewa key ====\n"
$dkms identifier init -a ewa --witness-url http://172.17.0.1:3232 --watcher-url http://172.17.0.1:3235

echo "\n==== Generate Jan key ==== \n"
$dkms identifier init -a jan --witness-url http://172.17.0.1:3232 --watcher-url http://172.17.0.1:3235

INFO=$($dkms identifier info ewa)
EWA_ID=$(echo $INFO | jq '.id' | tr -d '"')
EWA_OOBI=$($dkms identifier oobi get -a ewa) 
# echo $EWA_OOBI

INFO=$($dkms identifier info jan)
JAN_ID=$(echo $INFO | jq '.id' | tr -d '"')
JAN_OOBI=$($dkms identifier oobi get -a jan) 
# echo $JAN_OOBI

# Jan initiates group with Ewa
echo "\n==== Jan initiate multisig group ==== \n"
$dkms membership add -a jan -p $EWA_ID -g group0 -o $EWA_OOBI 

echo "\n==== Jan finalize multisig group ==== \n"
$dkms membership finalize -a jan --group-alias group0 --group-threshold 2 --witness-url http://172.17.0.1:3232 --witness-threshold 1

# Ewa gets and confirms group event
echo "\n==== Ewa pending ==== \n"
$dkms membership pending -a ewa --pull --time 3

echo "\n==== Ewa accepts==== \n"
$dkms membership accept -a ewa -i 0 --group-alias group1

# Jan gets confirmation, publish group event to witness and gets receipts
echo "\n==== Jan pending ==== \n"
$dkms membership pending -a jan --pull --time 3

# Ewa gets receipt
echo "\n==== Ewa pending ==== \n"
$dkms membership pending -a ewa --pull --time 3

echo "\n==== Jan signs messege ==== \n"
JAN_MSG=$($dkms membership sign -a jan -g group0 --message '{"msg": "hi"}')
echo $JAN_MSG

echo "\n==== Ewa signs messege ==== \n"
EWA_MSG=$($dkms membership sign -a ewa -g group1 --message '{"msg": "hi"}')

echo $EWA_MSG

# Combine two messages
# Extract prefix before "-AAB" from the first line
prefix=$(echo "$JAN_MSG" | awk -F"-AAB" '{print $1}')

# Extract suffix after "-AAB" from both lines
suffix1=$(echo "$JAN_MSG" | awk -F"-AAB" '{print $2}')
suffix2=$(echo "$EWA_MSG" | awk -F"-AAB" '{print $2}')

echo "\n==== Combined message ==== \n"
# Combine with "AAC" replacing the "AAB" in the merged suffix
COMBINED="${prefix}-AAC${suffix1}${suffix2}"
echo $COMBINED

GROUP_OOBI=$($dkms membership oobi -a ewa -g group1)

$dkms data verify -a jan --message "$COMBINED" --oobi $GROUP_OOBI
