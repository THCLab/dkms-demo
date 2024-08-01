alias dkms-dev-cli="./target/release/dkms-dev-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./payloads"

dkms-dev-cli init -a alice -c "./scripts/get_kel_test/alice_config.yaml"

dkms-dev-cli init -a bob -c "./scripts/get_kel_test/bobs_config.yaml"

dkms-dev-cli oobi get -a bob > boboobi.json 
dkms-dev-cli oobi resolve -a alice -f boboobi.json

dkms-dev-cli kel rotate -a bob

INFO=$(dkms-dev-cli info -a bob)
BOB_ID=$(echo $INFO | jq '.id' | tr -d '"')

# Query kel until it's ready
start=`date +%s%3N`
KEL=$(dkms-dev-cli kel query -a alice -i $BOB_ID)
while [ "$KEL" = "Kel not ready yet" ]
do
  KEL=$(dkms-dev-cli kel query -a alice -i $BOB_ID)
done
  
end=`date +%s%3N`
echo "\nQuering time was `expr $end - $start` miliseconds.\n"

echo "\n$KEL"

rm boboobi.json