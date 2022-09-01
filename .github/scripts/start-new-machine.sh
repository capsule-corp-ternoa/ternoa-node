#!/bin/bash

NOT_BUSY_MACHINES=$(curl -H "Accept: application/vnd.github+json" -H "Authorization: token $GITHUB_BUILD_MACHINE_TOKEN" https://api.github.com/repos/capsule-corp-ternoa/ternoa-node/actions/runners \
| python3 read-scw-machines.py ONLINE_NBUSY)

OFFLINE_MACHINES=$(curl -H "Accept: application/vnd.github+json" -H "Authorization: token $GITHUB_BUILD_MACHINE_TOKEN" https://api.github.com/repos/capsule-corp-ternoa/ternoa-node/actions/runners \
| python3 read-scw-machines.py OFFLINE) 

# If all machines are busy and there are machines that are offline
if [ -z "$NOT_BUSY_MACHINES" ] && [ "$OFFLINE_MACHINES" ]; then
    echo "$OFFLINE_MACHINES" | while read line; do
        if [ "$line" ]; then
            ./toggle-build-machine.sh start $line
            exit 0
        fi
    done
fi