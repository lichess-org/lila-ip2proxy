lila-ip2proxy
=============

Webservice to query an IP2Proxy BIN database.
See https://github.com/niklasf/ip2proxy-rust for a library to read the database
files.

Usage
-----

```
cargo run -- --port 1929 data/IP2PROXY-IP-PROXYTYPE-COUNTRY-REGION-CITY-ISP.SAMPLE.BIN
```

HTTP API
--------

### `GET /`

```
curl http://localhost:1929/?ip=1.0.0.1
```

name | type | description
--- | --- | ---
ip | ip | IP address to look up

```javascript
{
  "proxy_type": "DCH", // VPN, TOR, DCH, PUB, WEB, SES, -
  "country_short": "AU", // ISO 3166
  "country_long": "Australia" // ISO 3166
}
```

[More fields available](https://docs.rs/ip2proxy/1.0/ip2proxy/struct.Row.html)
depending on the columns of the IPProxy BIN database.

### `GET /status`

```
curl http://localhost:1929/status
```

```javascript
{
  "px": 2, // database format
  "day": 28, // 28th
  "month": 4, // April
  "year": 20, // 2020
  "rows_ipv4": 3948749,
  "rows_ipv6": 4065169
}
```

License
-------

lila-ip2proxy is licensed under the GNU Affero General Public License, version 3
or any later version, at your option.

This application is not intended to serve the database publically.
When serving data obtained from https://www.ip2location.com/, carefully
read the licensing conditions.
