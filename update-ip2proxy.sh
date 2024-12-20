#!/bin/sh -e
mkdir -p /var/local/ip2proxy
cd /var/local/ip2proxy
wget -q "https://www.ip2location.com/download?token=[REDACTED]&file=PX2BIN" -O PX2-IP-PROXYTYPE-COUNTRY.BIN.ZIP
mv IP2PROXY-IP-PROXYTYPE-COUNTRY.BIN IP2PROXY-IP-PROXYTYPE-COUNTRY.BIN.bak || echo First download
unzip -q -o PX2-IP-PROXYTYPE-COUNTRY.BIN.ZIP
systemctl restart lila-ip2proxy.service
