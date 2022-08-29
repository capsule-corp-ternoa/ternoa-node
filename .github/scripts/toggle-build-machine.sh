#!/bin/bash 

OP=$1
ID=$2
WAIT=$3

curl -o ./scw -L "https://github.com/scaleway/scaleway-cli/releases/download/v2.5.1/scaleway-cli_2.5.1_linux_amd64"
chmod u+x scw

./scw init secret-key=$SCW_SECRET_KEY zone=fr-par-2 install-autocomplete=false send-telemetry=true

echo "Updating machine. OP: $OP,  ID: $ID, Wait: $WAIT"
./scw instance server $OP $ID $WAIT