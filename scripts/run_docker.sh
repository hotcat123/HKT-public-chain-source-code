#!/bin/sh
set -e

hkt_HOME=${hkt_HOME:-/srv/hkt}
export hkt_HOME

if [ -n "$INIT" ]; then
    hktd init ${CHAIN_ID:+--chain-id="$CHAIN_ID"} \
               ${ACCOUNT_ID:+--account-id="$ACCOUNT_ID"}
fi

if [ -n "$NODE_KEY" ]; then
    cat << EOF > "$hkt_HOME/node_key.json"
{"account_id": "", "public_key": "", "secret_key": "$NODE_KEY"}
EOF
fi

ulimit -c unlimited

echo "Telemetry: ${TELEMETRY_URL}"
echo "Bootnodes: ${BOOT_NODES}"

exec hktd run ${TELEMETRY_URL:+--telemetry-url="$TELEMETRY_URL"} \
               ${BOOT_NODES:+--boot-nodes="$BOOT_NODES"} "$@"
