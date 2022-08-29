import json
import sys


def is_ok(config, runner):
    if (config == "ONLINE_NBUSY"):
        if (runner["status"] == "online" and runner["busy"] == False):
            return True
    if (config == "ONLINE_BUSY"):
        if (runner["status"] == "online" and runner["busy"] == True):
            return True
    if (config == "ONLINE"):
        if (runner["status"] == "online"):
            return True
    if (config == "OFFLINE"):
        if (runner["status"] == "offline"):
            return True


scw_machines = [{"name": "Ternoa-Build-Machine-0",
                 "id": "14930b12-a780-4dd0-a131-069f5d6024bd"},
                {"name": "Ternoa-Build-Machine-1",
                 "id": "bd7c96b0-bbb6-4796-977a-703256554787"},
                {"name": "Ternoa-Build-Machine-2",
                 "id": "a1a60d32-19ce-47ee-8f40-635a925ba11d"}]

config = sys.argv[1]
data = json.load(sys.stdin)
to_turn_off = []
for runner in data["runners"]:
    if (not is_ok(config, runner)):
        continue
    for machine in scw_machines:
        if (runner["name"] == machine["name"]):
            to_turn_off.append(machine["id"])
for x in to_turn_off:
    print(x)
