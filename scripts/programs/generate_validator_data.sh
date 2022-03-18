#!/bin/bash
dialog --title "Info" --msgbox 'Welcome. This Program will generate magic for you!' 6 35

function set_node_name () {
  dialog --inputbox "Enter your node name: " 8 40 2>temp.txt
  if [ "$?" = "1" ]; then
    return 1
  fi

  node_name=`cat temp.txt`
  dialog --title 'Info' --msgbox "Your node name is: $node_name" 5 50
  echo "Node name: $node_name" >> result.txt
  echo "" >> result.txt
}

function generate_node_key () {
  dialog --title "Message"  --yesno "Do you need a node key to be generated?" 6 30
  if [ "$?" = "1" ]; then
    return 1
  fi

  subkey generate-node-key --file node_key_private.txt 2> node_key_public.txt
  node_private_key=`cat node_key_private.txt`
  node_public_key=`cat node_key_public.txt`
  rm node_key_private.txt node_key_public.txt

  echo "Node data: " >> result.txt
  echo -e "Public key: $node_public_key\nPrivate key: $node_private_key" >> result.txt
  echo "" >> result.txt

  dialog --title 'Info' --msgbox "Your node info:\nPublic key:\n$node_public_key\nPrivate key:\n$node_private_key" 10 70
}

function account_selection () {
  dialog --menu "Account Selection" 10 40 5 "1" "Generate new Account" "2" "Use existing Account" 2>temp.txt
  if [ "$?" = "1" ]; then
    return 1
  fi

  answer=`cat temp.txt`

  # Controller (Sr25519) Account: 
  echo "// Controller (Sr25519) Account: " >> result.txt

  if [ "$answer" = "1" ]; then
    echo "subkey generate -w 24" >> result.txt
    subkey generate -w 24 > temp.txt
    cat temp.txt >> result.txt
    controller_sr25519_secret_phrase=`cat temp.txt | grep "Secret phrase" | cut -c 22-`;
  fi

  if [ "$answer" = "2" ]; then
    dialog --inputbox "Enter your secret phrase: " 8 50 2>temp.txt
    controller_sr25519_secret_phrase=`cat temp.txt`;
    echo "subkey inspect $controller_sr25519_secret_phrase" >> result.txt
    subkey inspect $controller_sr25519_secret_phrase > temp.txt
    cat temp.txt >> result.txt
  fi
  echo "" >> result.txt

  #controller_sr25519_secret_seed=`cat temp.txt | grep "Secret seed" | cut -c 22-`;
  controller_sr25519_account_id=`cat temp.txt | grep "Account ID" | cut -c 22-`;

  # Stash (Sr25519) Account: 
  echo "// Stash (Sr25519) Account: " >> result.txt
  echo "subkey inspect $controller_sr25519_secret_phrase//stash" >> result.txt
  subkey inspect "$controller_sr25519_secret_phrase//stash" > temp.txt
  cat temp.txt >> result.txt
  echo "" >> result.txt

  # Controller (Ed25519) Account: 
  echo "// Controller (Ed25519) Account: " >> result.txt
  echo "subkey inspect --scheme Ed25519  $controller_sr25519_secret_phrase" >> result.txt
  subkey inspect --scheme Ed25519  "$controller_sr25519_secret_phrase" > temp.txt
  cat temp.txt >> result.txt
  echo "" >> result.txt

  #controller_ed25519_secret_seed=`cat temp.txt | grep "Secret seed" | cut -c 22-`;
  #controller_ed25519_secret_phrase=`cat temp.txt | grep "Secret phrase" | cut -c 22-`;
  controller_ed25519_account_id=`cat temp.txt | grep "Account ID" | cut -c 22-`;

  dialog --title 'Info' --msgbox "Account Selection Done" 5 50
}

function generate_insert_session_key_script () {
  dialog --title "Message"  --yesno "Do you need the session keys insert script to be generate?" 6 40
  if [ "$?" = "1" ]; then
    return 1
  fi

cat > session_keys_script.sh <<End-of-text
seed="$controller_sr25519_secret_phrase"
sr25519="$controller_sr25519_account_id"
ed25519="$controller_ed25519_account_id"

curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"gran\",\"$seed\",\"$ed25519\"]}"
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"babe\",\"$seed\",\"$sr25519\"]}"
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"imon\",\"$seed\",\"$sr25519\"]}"
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"audi\",\"$seed\",\"$sr25519\"]}"
End-of-text

dialog --title 'Info' --msgbox "Script saved inside session_keys_script.sh file" 5 55
}

function generate_systemctl_script () {
  dialog --title "Message"  --yesno "Do you need the systemctl script to be generate?" 6 40
  if [ "$?" = "1" ]; then
    return 1
  fi

  dialog --menu "Node Selection" 10 40 5 "1" "Script for Validator node" "2" "Script for Public node" 2>temp.txt
  if [ "$?" = "1" ]; then
    return 1
  fi
  answer=`cat temp.txt`

  dialog --inputbox "Enter your chain: " 8 40 2>temp.txt
  if [ "$?" = "1" ]; then
    return 1
  fi

  chain=`cat temp.txt`
  db_path="/block/chain/node-data"

  if [ "$answer" = "1" ]; then
cat > ternoa.service <<End-of-text
[Unit]
Description=Ternoa Public Node By Ternoa.com

[Service]
ExecStart=/usr/bin/ternoa --name $node_name --chain $chain --base-path $db_path --validator  --wasm-execution compiled --prometheus-port 9615 --ws-max-connections 1000 --node-key $node_private_key
WorkingDirectory=/usr/bin
KillSignal=SIGINT
User=root
Restart=on-failure
LimitNOFILE=10240
SyslogIdentifier=ternoa


[Install]
WantedBy=multi-user.target
End-of-text
  fi

  if [ "$answer" = "2" ]; then
cat > ternoa.service <<End-of-text
[Unit]
Description=Ternoa Valdiator Node By Ternoa.com

[Service]
ExecStart=/usr/bin/ternoa --name $node_name --chain $chain --base-path $db_path --ws-external --rpc-external --rpc-cors all
WorkingDirectory=/usr/bin
KillSignal=SIGINT
User=root
Restart=on-failure
LimitNOFILE=10240
SyslogIdentifier=ternoa


[Install]
WantedBy=multi-user.target
End-of-text
  fi

  dialog --title 'Info' --msgbox "Script saved inside ternoa.service file" 5 50
}

function generate_bootnode_address () {
  dialog --title "Message"  --yesno "Do you want the node bootnode address to be generated?" 6 50
  if [ "$?" = "1" ]; then
    return 1
  fi

  dialog --inputbox "Enter your vm ip address: " 8 40 2>temp.txt
  if [ "$?" = "1" ]; then
    return 1
  fi

  local ip_address=`cat temp.txt`
  

  echo "Node bootnode address: " >> result.txt
  echo "/ip4/$ip_address/tcp/30333/p2p/$node_public_key" >> result.txt
  echo "" >> result.txt

  dialog --title 'Info' --msgbox "Your node address: /ip4/$ip_address/tcp/30333/p2p/$node_public_key" 8 80
}

rm result.txt

set_node_name
generate_node_key
generate_bootnode_address
account_selection
generate_insert_session_key_script
generate_systemctl_script

dialog --title 'Info' --msgbox "Program is Done :)" 5 50

rm temp.txt

clear
