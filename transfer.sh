#!/bin/bash

# Check if all required arguments are provided
if [ "$#" -ne 7 ]; then
    echo "Usage: $0 <WALLET_NAME> <TO_ADDRESS> <CONF_PATH> <FROM_ADDRESS> <CHANGE_ADDRESS> <UTXO_TXID> <UTXO_VOUT>"
    exit 1
fi

WALLET_NAME="$1"
TO_ADDRESS="$2"
CONF_PATH="$3"
FROM_ADDRESS="$4"
CHANGE_ADDRESS="$5"
UTXO_TXID="$6"
UTXO_VOUT="$7"

#Create a raw transaction
RAW_TRANSACTION=$(/Volumes/violin/regtest/bitcoin-25.0/bin/bitcoin-cli --conf="$CONF_PATH" --rpcwallet="$WALLET_NAME" createrawtransaction '[{"txid": "'$UTXO_TXID'","vout": '$UTXO_VOUT'}]' '{ "'$FROM_ADDRESS'": 1.462399, "'$CHANGE_ADDRESS'": 0.1 }')

# Sign the transaction
SIGNED_RAW_TRANSACTION=$(/Volumes/violin/regtest/bitcoin-25.0/bin/bitcoin-cli --conf="$CONF_PATH" --rpcwallet="$WALLET_NAME" signrawtransactionwithwallet "$RAW_TRANSACTION")

# Broadcast the transaction
TX_ID=$(/Volumes/violin/regtest/bitcoin-25.0/bin/bitcoin-cli --conf="$CONF_PATH" sendrawtransaction "$(echo "$SIGNED_RAW_TRANSACTION" | jq -r '.hex')")

# Display the transaction ID
echo "Transaction ID: $TX_ID"
