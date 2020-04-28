#!/bin/sh -e
cargo +stable build --release
ssh "root@$1.lichess.ovh" mv /usr/local/bin/lila-ip2proxy /usr/local/bin/lila-ip2proxy.bak || (echo "first deploy on this server? set up service and comment out this line" && false)
scp ./target/release/lila-ip2proxy "root@$1.lichess.ovh":/usr/local/bin/lila-ip2proxy
ssh "root@$1.lichess.ovh" systemctl restart lila-ip2proxy
