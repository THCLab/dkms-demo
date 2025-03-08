. test-vectors/dkms.sh

if [ -z "$MESAGKESTO_ADDRESS" ]; then
    echo "MESAGKESTO_ADDRESS not set. Please set it to the address of the Mesagkesto service"
    exit 1
fi

$dkms identifier init -a ewa --witness-url http://172.17.0.1:3233/ --watcher-url http://172.17.0.1:3235/

echo -e "\n==== Ewa's config before rotation: ====\n"
$dkms identifier info ewa

$dkms log kel rotate -a ewa -c "./test-vectors/rotate_witness/rotation_config.yaml"

echo -e "\n==== Ewa's config after rotation: ====\n"
$dkms identifier info ewa

echo -e "\n==== Ewa's KEL after rotation: ====\n"

INFO=$($dkms identifier info ewa)
EWA_ID=$(echo $INFO | jq '.id' | tr -d '"')
$dkms log kel find -a ewa -i $EWA_ID
