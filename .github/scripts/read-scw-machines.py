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
                 "id": "704d19ea-2648-4594-8ef4-28bdd1321e65"},
                {"name": "Ternoa-Build-Machine-1",
                 "id": "a3ab8cb9-cf33-4172-9ab0-fa802e6d38c9"}]

config = sys.argv[1]
data = json.load(sys.stdin)
to_turn_off: list[str] = []
for runner in data["runners"]:
    if (not is_ok(config, runner)):
        continue
    for machine in scw_machines:
        if (runner["name"] == machine["name"]):
            to_turn_off.append(machine["id"])
for x in to_turn_off:
    print(x)
