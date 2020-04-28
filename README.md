lila-ip2proxy
=============

Webservice to query an IP2Proxy BIN database.
See https://github.com/niklasf/ip2proxy-rust for a library to read the database
files.

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
  "proxy_type": "DCH",
  "country_short": "AU",
  "country_long": "Australia"
}
```

More fields available depending on the columns of the IPProxy BIN database.

License
-------

lila-i2proxy is licensed under the GNU Affero General Public License, version 3
or any later version, at your option.

This application is not intended to serve the database publically.
When serving data obtained from https://www.ip2location.com/, carefully
read the licensing conditions.
