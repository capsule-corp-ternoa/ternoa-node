#!/bin/bash
dialog --title "Info" --msgbox 'Welcome. This Program will generate magic for you!' 6 35

function set_node_name () {
  dialog --inputbox "Enter your node name: " 8 40 2>node_name.txt
  if [ "$?" = "1" ]; then
    rm node_name.txt
    return 1
  fi

  node_name=`cat node_name.txt`
  dialog --title 'Info' --msgbox "Your node name is: $node_name" 5 50
}

function generate_node_key () {
  dialog --title "Message"  --yesno "Do you need a node key to be generated?" 6 30
  if [ "$?" = "1" ]; then
    return 1
  fi

  subkey generate-node-key --file node_key_private.txt 2> node_key_public.txt
  local private=`cat node_key_private.txt`
  local public=`cat node_key_public.txt`
  echo -e "Public key: $public\nPrivate key: $private" > node_key.txt
  rm node_key_private.txt node_key_public.txt

  clear
  node_key="$private $public"
  echo $node_key
}

function account_selection () {
  account_found=false

  dialog --menu "Account Selection" 10 40 5 "1" "Generate new Account" "2" "Use existing Account" 2>temp.txt
  if [ "$?" = "1" ]; then
    return 1
  fi

  answer=`cat temp.txt`

  # Controller (Sr25519) Account: 
  echo "// Controller (Sr25519) Account: " > account.txt

  if [ "$answer" = "1" ]; then
    echo "subkey generate -w 24" >> account.txt
    subkey generate -w 24 > temp.txt
    cat temp.txt >> account.txt
    controller_sr25519_secret_phrase=`cat temp.txt | grep "Secret phrase" | cut -c 22-`;
  fi

  if [ "$answer" = "2" ]; then
    dialog --inputbox "Enter your secret phrase: " 8 50 2>temp.txt
    controller_sr25519_secret_phrase=`cat temp.txt`;
    echo "subkey inspect $controller_sr25519_secret_phrase" >> account.txt
    subkey inspect $controller_sr25519_secret_phrase > temp.txt
    cat temp.txt >> account.txt
  fi

  #controller_sr25519_secret_seed=`cat temp.txt | grep "Secret seed" | cut -c 22-`;
  controller_sr25519_account_id=`cat temp.txt | grep "Account ID" | cut -c 22-`;

  # Stash (Sr25519) Account: 
  echo "" >> account.txt
  echo "// Stash (Sr25519) Account: " >> account.txt
  echo "subkey inspect $controller_sr25519_secret_phrase//stash" >> account.txt
  subkey inspect "$controller_sr25519_secret_phrase//stash" > temp.txt
  cat temp.txt >> account.txt

  # Controller (Ed25519) Account: 
  echo "" >> account.txt
  echo "// Controller (Ed25519) Account: " >> account.txt
  echo "subkey inspect --scheme Ed25519  $controller_sr25519_secret_phrase" >> account.txt
  subkey inspect --scheme Ed25519  "$controller_sr25519_secret_phrase" > temp.txt
  cat temp.txt >> account.txt

  #controller_ed25519_secret_seed=`cat temp.txt | grep "Secret seed" | cut -c 22-`;
  #controller_ed25519_secret_phrase=`cat temp.txt | grep "Secret phrase" | cut -c 22-`;
  controller_ed25519_account_id=`cat temp.txt | grep "Account ID" | cut -c 22-`;

  rm temp.txt
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

}

set_node_name
generate_node_key
account_selection
generate_insert_session_key_script
clear


# clear

# dialog --title "Hello" --msgbox "${node_name}" 6 35

# dialog --backtitle "System Information" \
# --title "About" \
# --msgbox 'This is an entirely open source software.' 10 30

# clear

# function start_menu () {
#     local options=$1;
#     for i in "${options[@]}"
#     do
#         echo "$i"
#     done

#     echo "Hello WOrld!";
# }


# echo "Hello world. This Program will generate a script for you that you can run on the validator machine."
# echo "Let's start with the questions."
# echo "What's your node name:"
# read node_name

# op=("TACO", "Banana")
# start_menu op




