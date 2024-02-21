#!/bin/bash
set -x

delay_sec=$1
amount=$2

account_id=$(grep account_id /home/ubuntu/.hkt/shardnet/validator_key.json | awk -F'"' '{print $4}')
staking_key=$(grep public_key /home/ubuntu/.hkt/shardnet/validator_key.json | awk -F'"' '{print $4}')

while true; do
  skip=0
  hkt_ENV=shardnet hkt --nodeUrl=http://127.0.0.1:3030 proposals | grep ${account_id}
  if [ $? -eq 0 ]; then
    # Already in the proposals.
    echo "$(date): Found in the proposals"
    skip=1
  fi
  hkt_ENV=shardnet hkt --nodeUrl=http://127.0.0.1:3030 validators current | grep ${account_id}
  if [ $? -eq 0 ]; then
    # Is currently a validator.
    echo "$(date): Currently a validator"
    skip=1
  fi
  if [ ${skip} -eq 0 ]; then
    # Not skipping, do the staking.
    echo "$(date): Doing restaking"
    hkt_ENV=shardnet hkt --nodeUrl=http://127.0.0.1:3030 stake ${account_id} ${staking_key} ${amount}
  fi
  echo "$(date): Sleeping for ${delay_sec} seconds"
  sleep ${delay_sec}
done