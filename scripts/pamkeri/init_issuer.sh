alias keri-cli="./target/release/keri-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"

keri-cli init -a alice -c "./scripts/pamkeri/pamkeri_config.yaml"
keri-cli tel incept -a alice
echo "\n"
keri-cli info -a alice