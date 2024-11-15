dkms="./dkms"

if [ ! -e "$dkms" ]; then
    echo "$dkms bin not found. Please see README"
    exit 1
fi

$dkms init -a alice -c "./test-vectors/pamkeri/pamkeri_config.yaml"
$dkms tel incept -a alice

SIGNED=$($dkms sign -a alice -d '{"hello":"world"}')
echo "Signed message: \n"
echo $SIGNED


$dkms init -a bob -c "./test-vectors/pamkeri/pamkeri_config.yaml"

ALICE_OOBI=$($dkms oobi get -a alice) 
echo "\n"
$dkms verify -a bob -m "$SIGNED" -o "$ALICE_OOBI"

# WRONG='{"hello":"world"}-FABELRfU-deAvyeTz3v-PsysZrwqWC52_piI712KboxoyJS0AAAAAAAAAAAAAAAAAAAAAAAELRfU-deAvyeTz3v-PsysZrwqWC52_piI712KboxoyJS-AABAAB9JZdo8PXF8fI4OUw3qoTf66lElBDPk-YtYlaWcdFOkNI3yTX-OgnhLDVLsGEpDKxDbOrmKgeF8vvv6k2TctQf'
# $dkms verify -a alice -m "$WRONG"
