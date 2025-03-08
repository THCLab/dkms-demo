. test-vectors/dkms.sh

if [ -z "$MESAGKESTO_ADDRESS" ]; then
    echo "MESAGKESTO_ADDRESS not set. Please set it to the address of the Mesagkesto service"
    exit 1
fi

$dkms identifier init -a ewa --witness-url http://172.17.0.1:3233/ --watcher-url http://172.17.0.1:3235/

$dkms identifier init -a jan --witness-url http://172.17.0.1:3232/ --watcher-url http://172.17.0.1:3235/
# TODO: do I need to do that?
# $dkms identifier oobi resolve -a alice -f boboobi.json

INFO=$($dkms identifier info jan)
JAN_ID=$(echo $INFO | jq '.id' | tr -d '"')
JAN_OOBI=$($dkms identifier oobi get -a jan)

echo -e "\nJan's KEL before rotation:"
$dkms log kel find -a jan -i $JAN_ID --oobi $JAN_OOBI

$dkms log kel rotate -a jan

echo "\nJan's KEL after rotation:"
$dkms log kel find -a jan -i $JAN_ID --oobi $JAN_OOBI
