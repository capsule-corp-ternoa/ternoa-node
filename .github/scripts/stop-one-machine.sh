GITHUB_BUILD_MACHINE_TOKEN=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

NOT_BUSY_MACHINES=$(curl -H "Accept: application/vnd.github+json" -H "Authorization: token $GITHUB_BUILD_MACHINE_TOKEN" https://api.github.com/repos/capsule-corp-ternoa/chain/actions/runners \
| python3 read-scw-machines.py ONLINE_NBUSY)

echo "$NOT_BUSY_MACHINES" | while read line; do
    if [ "$line" ]; then
        ./toggle-build-machine.sh stop $line "--wait"
        exit 0
    fi
done