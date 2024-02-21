#!/bin/bash
set -eux

account_id=$(grep account_id /home/ubuntu/.hkt/shardnet/validator_key.json | awk -F'"' '{print $4}')
mkdir -p /home/ubuntu/.hkt-credentials/shardnet/
printf '{"account_id":"hkt","public_key":"%s","private_key":"%s"}' \
    "${1:?}" "${2:?}" > /home/ubuntu/.hkt-credentials/shardnet/hkt.json
pk=$(grep public_key /home/ubuntu/.hkt/shardnet/validator_key.json | awk -F'"' '{print $4}')
cp /home/ubuntu/.hkt/shardnet/validator_key.json /home/ubuntu/.hkt-credentials/shardnet/"$account_id".json
hkt_ENV=shardnet hkt --nodeUrl=http://127.0.0.1:3030 \
        create-account "$account_id" --masterAccount hkt \
        --initialBalance 1000000 --publicKey "$pk"
