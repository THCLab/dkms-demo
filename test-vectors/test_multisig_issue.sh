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

echo "\n==== Jan incepts group registry ==== \n"
$dkms membership registry -a jan -g group0

# Ewa gets and confirms group event
echo "\n==== Ewa pending ==== \n"
$dkms membership pending -a ewa --pull --time 1
$dkms membership accept -a ewa -i 0

# Jan gets confirmation, publish group event to witness and gets receipts
echo "\n==== Jan pending ==== \n"
$dkms membership pending -a jan --pull --time 3

# Ewa gets receipt
echo "\n==== Ewa pending ==== \n"
$dkms membership pending -a ewa --pull --time 3

# Registry is now ready for issuing
$dkms membership info -a jan -g group0
$dkms membership info -a ewa -g group1

echo "\n==== Jan issue ==== \n"
$dkms membership issue -a jan -g group0 -m '{"hello":"world"}' -b "EIgEk-irWY5zkU8E9zq4B1PU_h4l03ZtQmOTUK0Up-1O"

# Ewa gets and confirms group event
echo "\n==== Ewa pending ==== \n"
$dkms membership pending -a ewa --pull --time 1
$dkms membership pending -a ewa 
$dkms membership accept -a ewa -i 0 

# Jan gets confirmation, publish group event to witness and gets receipts
echo "\n==== Jan pending ==== \n"
$dkms membership pending -a jan --pull --time 3

# Ewa gets receipt
echo "\n==== Ewa pending ==== \n"
$dkms membership pending -a ewa --pull --time 3

$dkms membership info -a jan -g group0
$dkms membership info -a ewa -g group1

