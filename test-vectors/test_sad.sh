# Test showcase how to calculate Self-Addressing Data

. test-vectors/dkms.sh

# Generate payload "d" attribute is mandatory
echo -n '{"hello":"world","d":""}' > /tmp/said.json

$dkms said sad -j "$(cat /tmp/said.json)"

# Order of the attributes have influance over final hash (sad command does not do cannonical serialization)
echo -n '{"d":"","hello":"world"}' > /tmp/said.json
$dkms said sad -j "$(cat /tmp/said.json)"
