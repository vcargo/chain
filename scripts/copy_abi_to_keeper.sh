#!/usr/bin/env bash

# Usage:
# forge build
# ./copy_abi_to_keeper.sh

current_dir=$(basename "$PWD")
if [ "$current_dir" != "dfx-tools" ]; then
    echo "This script can only be run in the 'dfx-tools' directory."
    exit 1
fi

source_path="./artifacts/contracts"
target_path="../keeper/abi"

contracts=(
    "PairsStorage"
    "PairInfos"
    "TradingStorage"
    "TradingStorageReader"
    "Trading"
    "TradingCallbacks"
    "Referrals"
)

for contract in "${contracts[@]}"; do
    source_file=$source_path/$contract.sol/$contract.json
    if [[ -f "$source_file" ]]; then
        cp $source_file $target_path/$contract.json
    fi
done
