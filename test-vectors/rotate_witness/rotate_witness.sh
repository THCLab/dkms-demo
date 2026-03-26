. test-vectors/dkms.sh

if [ -z "$MESAGKESTO_ADDRESS" ]; then
    echo "MESAGKESTO_ADDRESS not set. Please set it to the address of the Mesagkesto service"
    exit 1
fi

$dkms identifier init -a ewa --witness-url $WITNESS2_URL/ --watcher-url $WATCHER_URL/

echo -e "\n==== Ewa's config before rotation: ====\n"
$dkms identifier info ewa

# Generate rotation config from template, substituting the current WITNESS1_URL
ROTATION_CONFIG=$(mktemp /tmp/rotation_config.XXXXXX.yaml)
envsubst < "./test-vectors/rotate_witness/rotation_config.yaml.tmpl" > "$ROTATION_CONFIG"
$dkms log kel rotate -a ewa -c "$ROTATION_CONFIG"
rm -f "$ROTATION_CONFIG"

echo -e "\n==== Ewa's config after rotation: ====\n"
$dkms identifier info ewa

echo -e "\n==== Ewa's KEL after rotation: ====\n"

INFO=$($dkms identifier info ewa)
EWA_ID=$(echo $INFO | jq '.id' | tr -d '"')
$dkms log kel find -a ewa -i $EWA_ID
