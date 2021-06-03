lila-ip2proxy
=============

Webservice to query an IP2Proxy BIN database. This is a thin wrapper around
https://github.com/niklasf/ip2proxy-rust, a library to read these database
files.

License
-------

lila-ip2proxy is licensed under the GNU Affero General Public License, version 3
or any later version, at your option.

:warning: **This application is not intended to serve the database publically.
When serving data obtained from https://www.ip2location.com/, carefully
read the licensing conditions.**

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
ip | string | IP address to look up

* `200 OK`

  ```javascript
  {
    "proxy_type": "DCH", // VPN, TOR, DCH, PUB, WEB, SES, -
    "country_short": "AU", // ISO 3166
    "country_long": "Australia" // ISO 3166
  }
  ```

  [More fields available](https://docs.rs/ip2proxy/1.0/ip2proxy/struct.Row.html)
  depending on the columns of the IP2Proxy BIN database.

* `404 Not found`

  No record for this IP, so probably not a proxy.
  It appears to be more common that a record exists, but `-` explicitly
  indicates that it is not a proxy.

* `500 Internal Server Error`

  Corrupted database file or unexpected format

### `GET /batch`

```
curl http://localhost:1929/batch?ips=1.0.0.1,2a00:1450:4001:809::200e,80.129.73.200
```

name | type | description
---- | --- | --
ips | string | Comma separated list of IP addresses to look up

```javascript
[
  {
    "proxy_type": "DCH",
    "country_short": "US",
    "country_long": "United States of America"
  },
  {
    "proxy_type": "DCH",
    "country_short":"DE",
    "country_long":"Germany"
  },
  {
    "proxy_type": "-", // not a proxy
    "country_short": "-",
    "country_long": "-"
  }
]
```

Response is an array in the same order. Entries can be null, theoretically,
corresponding to a 404 in the `GET /` endpoint.

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
