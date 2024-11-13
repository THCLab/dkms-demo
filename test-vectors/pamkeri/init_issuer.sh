alias dkms-dev-cli="./target/release/dkms-dev-cli"
MESAGKESTO_ADDRESS="http://172.17.0.1:3236"

dkms-dev-cli init -a alice -c "./scripts/pamkeri/pamkeri_config.yaml"
dkms-dev-cli tel incept -a alice
echo "\n"
dkms-dev-cli info -a alice