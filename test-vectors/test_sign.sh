# This test showcase how to sign and verify arbitrary message

. test-vectors/dkms.sh

echo "==== Generate Ewa key ===="
echo
# Generate identifier for Ewa
$dkms identifier init -a ewa --witness-url http://172.17.0.1:3234 --watcher-url http://172.17.0.1:3235

# SIGN data payload using Ewa key
SIGNED=$($dkms data sign -a ewa -m '{"hello":"world"}')
echo "==== Signed message by Ewa ===="
echo $SIGNED
echo

echo "==== Generate Jan key ===="
echo
# Generate identifier for Jan
$dkms identifier init -a jan --witness-url http://172.17.0.1:3234 --watcher-url http://172.17.0.1:3235

# Retrive Ewa OOBI for Jan to being able to retrive her Identifier KEL for verification
EWA_OOBI=$($dkms identifier oobi get -a ewa)

# Verify Ewa message by Jan
$dkms data verify -a jan -m "$SIGNED" -o "$EWA_OOBI"

# WRONG='{"hello":"world"}-FABELRfU-deAvyeTz3v-PsysZrwqWC52_piI712KboxoyJS0AAAAAAAAAAAAAAAAAAAAAAAELRfU-deAvyeTz3v-PsysZrwqWC52_piI712KboxoyJS-AABAAB9JZdo8PXF8fI4OUw3qoTf66lElBDPk-YtYlaWcdFOkNI3yTX-OgnhLDVLsGEpDKxDbOrmKgeF8vvv6k2TctQf'
# $dkms verify -a ewa -m "$WRONG"
