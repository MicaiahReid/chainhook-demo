#!/bin/bash

# Check if all required arguments are provided
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <WALLET_NAME> <TO_ADDRESS> <CONF_PATH>"
    exit 1
fi

WALLET_NAME="$1"
TO_ADDRESS="$2"
CONF_PATH="$3"

# Generate a new wallet
/Volumes/violin/regtest/bitcoin-25.0/bin/bitcoin-cli --conf="$CONF_PATH" createwallet "$WALLET_NAME"

# Generate a new address for the wallet and display it
NEW_ADDRESS=$(/Volumes/violin/regtest/bitcoin-25.0/bin/bitcoin-cli --conf="$CONF_PATH" --rpcwallet="$WALLET_NAME" getnewaddress)
echo "New address generated: $NEW_ADDRESS"

# Add funds to the address (example: sending 0.1 BTC)
/Volumes/violin/regtest/bitcoin-25.0/bin/bitcoin-cli --conf="$CONF_PATH" --rpcwallet="$WALLET_NAME" generatetoaddress 101 "$NEW_ADDRESS"

# Generate another new address for the wallet
CHANGE_ADDRESS=$(/Volumes/violin/regtest/bitcoin-25.0/bin/bitcoin-cli --conf="$CONF_PATH" --rpcwallet="$WALLET_NAME" getnewaddress)
echo "Change address generated: $CHANGE_ADDRESS"

UTXOS=$(/Volumes/violin/regtest/bitcoin-25.0/bin/bitcoin-cli --conf="$CONF_PATH" --rpcwallet="$WALLET_NAME" listunspent)
echo "UTXOS $UTXOS"
