# Debian 10.10

2(4) i5-6200U, 8GB, 256GB M.2 @ 0.5 GB/s

```bash
# Non-SSL
ab -k -n 25000 -c 50 'http://127.0.0.1:8080/sample.txt'
ab -k -n 25000 -c 50 'http://127.0.0.1:8080/sample.jpg'
# SSL
ab -k -n 25000 -c 50 'https://127.0.0.1:44300/sample.txt'
ab -k -n 25000 -c 50 'https://127.0.0.1:44300/sample.jpg'
```

|                       | 8 KB (Embedded Text)     | 500 KB (Image File)      |
|-----------------------|--------------------------|--------------------------|
| Node-HTTP             |  8690 `#/s` @  70 `MB/s` |  1796 `#/s` @  898 `MB/s` |
| Node-HTTP-SSL         |  1089 `#/s` @   9 `MB/s` |   705 `#/s` @  353 `MB/s` |
| Node-UWS              | 20907 `#/s` @ 168 `MB/s` |  2968 `#/s` @ 1484 `MB/s` |
| Node-UWS-SSL          |  1538 `#/s` @  12 `MB/s` |   806 `#/s` @  403 `MB/s` |
| Deno*                 |  3132 `#/s` @  25 `MB/s` |   736 `#/s` @  368 `MB/s` |
| Go-NET/HTTP           | 65792 `#/s` @ 535 `MB/s` |  9813 `#/s` @ 4908 `MB/s` |
| Go-NET/HTTP*          | 20498 `#/s` @ 166 `MB/s` |  4858 `#/s` @ 2429 `MB/s` |
| Go-NET/HTTP-SSL       | 35852 `#/s` @ 292 `MB/s` |  2665 `#/s` @ 1332 `MB/s` |
| Go-NET/HTTP-SSL*      |   894 `#/s` @   7 `MB/s` |   694 `#/s` @  347 `MB/s` |
| Go-FastHTTP           | 69538 `#/s` @ 567 `MB/s` | 11371 `#/s` @ 5688 `MB/s` |
| Go-FastHTTP*          | 21696 `#/s` @ 177 `MB/s` |  5175 `#/s` @ 2589 `MB/s` |
| Go-FastHTTP-SSL       | 37331 `#/s` @ 304 `MB/s` |  1537 `#/s` @  769 `MB/s` |
| Go-FastHTTP-SSL*      |   891 `#/s` @   7 `MB/s` |   645 `#/s` @  323 `MB/s` |
| Rust actix-web        | 93304 `#/s` @ 755 `MB/s` |  4651 `#/s` @ 2327 `MB/s` |
| Rust actix-web*       | 21013 `#/s` @ 169 `MB/s` |  3356 `#/s` @ 1679 `MB/s` |
| Rust actix-web SSL    | 42518 `#/s` @ 344 `MB/s` |  2277 `#/s` @ 1139 `MB/s` |
| Rust actix-web SSL*   |   932 `#/s` @   7 `MB/s` |   876 `#/s` @  438 `MB/s` |
| ASP.NET Core 5.0      |  9218 `#/s` @  74 `MB/s` |  3846 `#/s` @ 1923 `MB/s` |
| ASP.NET Core 5.0*     | 12602 `#/s` @ 102 `MB/s` |  2771 `#/s` @ 1386 `MB/s` |
| ASP.NET Core 5.0 SSL  |  1118 `#/s` @   9 `MB/s` |  1210 `#/s` @  605 `MB/s` |
| ASP.NET Core 5.0 SSL* |  1142 `#/s` @   9 `MB/s` |   624 `#/s` @  312 `MB/s` |
| v-web                 |  8197 `#/s` @  66 `MB/s` |  1104 `#/s` @  552 `MB/s` |

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
# Non-SSL
ab -k -n 7500 -c 50 'http://127.0.0.1:8080/sample.txt'
ab -k -n 7500 -c 50 'http://127.0.0.1:8080/sample.jpg'
# SSL
ab -k -n 7500 -c 50 'https://127.0.0.1:44300/sample.txt'
ab -k -n 7500 -c 50 'https://127.0.0.1:44300/sample.jpg'
```

|                       | 8 KB (Embedded Text)     | 500 KB (Image File)      |
|-----------------------|--------------------------|--------------------------|
| Node-HTTP             |  3729 `#/s` @  30 `MB/s` | 1239 `#/s` @  619 `MB/s` |
| Node-HTTP-SSL         |   383 `#/s` @   3 `MB/s` |  268 `#/s` @  134 `MB/s` |
| Node-UWS              | 12453 `#/s` @ 100 `MB/s` | 2525 `#/s` @ 1262 `MB/s` |
| Node-UWS-SSL          |   505 `#/s` @   4 `MB/s` |  377 `#/s` @  189 `MB/s` |
| Deno*                 |  5784 `#/s` @  47 `MB/s` |  768 `#/s` @  384 `MB/s` |
| Go-NET/HTTP           | 33249 `#/s` @ 270 `MB/s` | 5265 `#/s` @ 2633 `MB/s` |
| Go-NET/HTTP*          | 12127 `#/s` @  98 `MB/s` | 3230 `#/s` @ 1615 `MB/s` |
| Go-NET/HTTP-SSL       | 14799 `#/s` @ 120 `MB/s` | 1763 `#/s` @  881 `MB/s` |
| Go-NET/HTTP-SSL*      |   354 `#/s` @   3 `MB/s` |  311 `#/s` @  154 `MB/s` |
| Go-FastHTTP           | 32254 `#/s` @ 263 `MB/s` | 5217 `#/s` @ 2609 `MB/s` |
| Go-FastHTTP*          | 14544 `#/s` @ 118 `MB/s` | 3276 `#/s` @ 1639 `MB/s` |
| Go-FastHTTP-SSL       | 15377 `#/s` @ 125 `MB/s` | 1014 `#/s` @  507 `MB/s` |
| Go-FastHTTP-SSL*      |   357 `#/s` @   3 `MB/s` |  286 `#/s` @  143 `MB/s` |
| Rust actix-web        | 43406 `#/s` @ 351 `MB/s` | 4252 `#/s` @ 2128 `MB/s` |
| Rust actix-web*       | 15019 `#/s` @ 121 `MB/s` | 2650 `#/s` @ 1325 `MB/s` |
| Rust actix-web SSL    | 19955 `#/s` @ 161 `MB/s` | 1487 `#/s` @  744 `MB/s` |
| Rust actix-web SSL*   |   425 `#/s` @   3 `MB/s` |  334 `#/s` @  167 `MB/s` |
| ASP.NET Core 5.0      |  8866 `#/s` @  72 `MB/s` | 3846 `#/s` @ 1923 `MB/s` |
| ASP.NET Core 5.0*     | 10182 `#/s` @  82 `MB/s` | 2596 `#/s` @ 1298 `MB/s` |
| ASP.NET Core 5.0 SSL  |    87 `#/s` @  <1 `MB/s` | 1000 `#/s` @  500 `MB/s` |
| ASP.NET Core 5.0 SSL* |   100 `#/s` @  <1 `MB/s` |   92 `#/s` @   46 `MB/s` |
| v-web                 | ~20% of requests failed  | ~40% of requests failed  |

*) Running Apache Benchmark without `-k` (keep alive connections)