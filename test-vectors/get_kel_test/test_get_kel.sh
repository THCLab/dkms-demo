. test-vectors/dkms.sh

# Ewa would have watcher which would observe any interaction for her
$dkms identifier init -a ewa --witness-url http://172.17.0.1:3233/ --watcher-url http://172.17.0.1:3235/

# Jan would NOT have a watcher means he would be able to resolve someone else
# KEL only locally, by providing KEL direction from witness of the entity which
# he interact with.
$dkms identifier init -a jan --witness-url http://172.17.0.1:3233/


INFO=$($dkms identifier info jan)
echo $INFO
JAN_ID=$(echo $INFO | jq '.id' | tr -d '"')

echo -e "\nLocal Jan's KEL: "
$dkms log kel find -a jan -i $JAN_ID

echo -e "\nJan's KEL from watcher: "
JAN_OOBI=$($dkms identifier oobi get -a jan)
# Watcher is retrieved from Ewa's identifier
$dkms log kel find -a ewa -i $JAN_ID --oobi $JAN_OOBI
