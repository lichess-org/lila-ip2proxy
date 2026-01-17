#!/bin/sh -e

DB_FILE="IP2PROXY-IP-PROXYTYPE-COUNTRY.BIN"
UPDATE_FILE="${LILA_IP2PROXY_UPDATE_FILE:-PX2BIN}"

mkdir -p "$LILA_IP2PROXY_DATA_DIR"
cd "$LILA_IP2PROXY_DATA_DIR"

wget -q "https://www.ip2location.com/download?token=${LILA_IP2PROXY_UPDATE_TOKEN}&file=${UPDATE_FILE}" -O "${DB_FILE}.ZIP"
mv "$DB_FILE" "${DB_FILE}.bak" 2>/dev/null || true
unzip -q -o "${DB_FILE}.ZIP"
rm -f "${DB_FILE}.ZIP"
