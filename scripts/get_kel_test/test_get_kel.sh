alias keri-cli="./target/release/keri-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"
INPUT_DATA_DIR="./payloads"

keri-cli init -a alice -c "./scripts/get_kel_test/alice_config.yaml"

keri-cli init -a bob -c "./scripts/get_kel_test/bobs_config.yaml"

keri-cli oobi get -a bob > boboobi.json 
keri-cli oobi resolve -a alice -f boboobi.json

keri-cli kel rotate -a bob

INFO=$(keri-cli info -a bob)
BOB_ID=$(echo $INFO | jq '.id' | tr -d '"')

# Query kel until it's ready
start=`date +%s%3N`
KEL=$(keri-cli kel query -a alice -i $BOB_ID)
while [ "$KEL" = "Kel not ready yet" ]
do
  KEL=$(keri-cli kel query -a alice -i $BOB_ID)
done

end=`date +%s%3N`
echo "\nQuering time was `expr $end - $start` miliseconds.\n"

echo $KEL

rm boboobi.json