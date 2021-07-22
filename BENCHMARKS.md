# Debian 10.10

2(4) i5-6200U, 8GB, 256GB M.2 @ 0.5 GB/s

```bash
ab -k -n 25000 -c 50 'http://127.0.0.1:8080/sample.txt'
ab -k -n 25000 -c 50 'http://127.0.0.1:8080/sample.jpg'
```

|                  | 8 KB (Embedded Text)     | 500 KB (Image File)      |
|------------------|--------------------------|--------------------------|
| Node-HTTP        |  8690 `#/s` @  70 `MB/s` | 1796 `#/s` @  898 `MB/s` |
| Node-UWS         | 20907 `#/s` @ 168 `MB/s` | 2968 `#/s` @ 1484 `MB/s` |
| Deno*            |  3132 `#/s` @  25 `MB/s` |  736 `#/s` @  368 `MB/s` |
| Go               | 19593 `#/s` @ 158 `MB/s` | 4592 `#/s` @ 2297 `MB/s` |
| Rust actix-web   | 65787 `#/s` @ 532 `MB/s` | 1010 `#/s` @  506 `MB/s` |
| Rust actix-web*  | 23168 `#/s` @ 187 `MB/s` | 3042 `#/s` @ 1522 `MB/s` |
| ASP.NET Core 5.0 | 11082 `#/s` @  90 `MB/s` | 3443 `#/s` @ 1722 `MB/s` |
| v-web            |  8197 `#/s` @  66 `MB/s` | 1104 `#/s` @  552 `MB/s` |

*) Running Apache Benchmark without `-k` (keep alive connections)

# Windows 10

6(12) x i7-, 32 GB, 512 GB NVMe @ 2 GB/s

```powershell
ab -k -n 25000 -c 50 'http://127.0.0.1:8080/sample.txt'
ab -k -n 25000 -c 50 'http://127.0.0.1:8080/sample.jpg'
```

|                  | 8 KB (Embedded Text)     | 500 KB (Image File)      |
|------------------|--------------------------|--------------------------|
| Node-HTTP        | ? `#/s` @ ? `MB/s` | ? `#/s` @ ? `MB/s` |
| Node-UWS         | ? `#/s` @ ? `MB/s` | ? `#/s` @ ? `MB/s` |
| Deno*            | ? `#/s` @ ? `MB/s` | ? `#/s` @ ? `MB/s` |
| Go               | ? `#/s` @ ? `MB/s` | ? `#/s` @ ? `MB/s` |
| Rust actix-web   | ? `#/s` @ ? `MB/s` | ? `#/s` @ ? `MB/s` |
| Rust actix-web*  | ? `#/s` @ ? `MB/s` | ? `#/s` @ ? `MB/s` |
| ASP.NET Core 5.0 | ? `#/s` @ ? `MB/s` | ? `#/s` @ ? `MB/s` |
| v-web            | ? `#/s` @ ? `MB/s` | ? `#/s` @ ? `MB/s` |

*) Running Apache Benchmark without `-k` (keep alive connections)

# MacOS

2(4) x i5-7360U, 8 GB, 500 GB NVMe @ 2 GB/s

Limited number of requests due to [port starvation](https://stackoverflow.com/questions/1216267/ab-program-freezes-after-lots-of-requests-why) on macOS
```zsh
ab -k -n 7500 -c 50 'http://127.0.0.1:8080/sample.txt'
ab -k -n 7500 -c 50 'http://127.0.0.1:8080/sample.jpg'
```

|                   | 8 KB (Embedded Text)     | 500 KB (Image File)      |
|-------------------|--------------------------|--------------------------|
| Node-HTTP         |  3729 `#/s` @  30 `MB/s` | 1239 `#/s` @  619 `MB/s` |
| Node-UWS          | 12453 `#/s` @ 100 `MB/s` | 2525 `#/s` @ 1262 `MB/s` |
| Deno*             |  5784 `#/s` @  47 `MB/s` |  768 `#/s` @  384 `MB/s` |
| Go-NET/HTTP       | 33249 `#/s` @ 270 `MB/s` | 5265 `#/s` @ 2633 `MB/s` |
| Go-NET/HTTP*      | 12127 `#/s` @  98 `MB/s` | 3230 `#/s` @ 1615 `MB/s` |
| Go-SSL-NET/HTTP   | 14799 `#/s` @ 120 `MB/s` | 1763 `#/s` @  881 `MB/s` |
| Go-SSL-NET/HTTP*  |   354 `#/s` @   3 `MB/s` |  311 `#/s` @  154 `MB/s` |
| Go-FastHTTP       | 32254 `#/s` @ 263 `MB/s` | 5217 `#/s` @ 2609 `MB/s` |
| Go-FastHTTP*      | 14544 `#/s` @ 118 `MB/s` | 3276 `#/s` @ 1639 `MB/s` |
| Go-SSL-FastHTTP   | 15377 `#/s` @ 125 `MB/s` | 1014 `#/s` @  507 `MB/s` |
| Go-SSL-FastHTTP*  |   357 `#/s` @   3 `MB/s` |  286 `#/s` @  143 `MB/s` |
| Rust actix-web    | 33447 `#/s` @ 271 `MB/s` | 2929 `#/s` @ 1465 `MB/s` |
| Rust actix-web*   | 15019 `#/s` @ 121 `MB/s` | 2351 `#/s` @ 1176 `MB/s` |
| ASP.NET Core 5.0  |  8866 `#/s` @  72 `MB/s` | 3846 `#/s` @ 1923 `MB/s` |
| ASP.NET Core 5.0* | 10182 `#/s` @  82 `MB/s` | 2596 `#/s` @ 1298 `MB/s` |
| v-web             | ~20% of requests failed  | ~40% of requests failed  |

*) Running Apache Benchmark without `-k` (keep alive connections)