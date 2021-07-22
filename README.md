# Web Server - Benchmark

## Node (HTTP)

```bash
cd ./servers/node
node ./serve-http.js
```

## Node (UWS)

```bash
cd ./servers/node
npm install
node ./serve-uws.js
```

## Deno

```bash
cd ./servers/deno
deno run --allow-net --allow-read --unstable ./serve.ts
```

## golang

```bash
cd ./servers/golang
go run ./serve-nethttp.go # Non-SSL
go run ./servessl-nethttp.go # SSL
```

## Rust

```bash
cd ./servers/rust
cargo run --release --bin=serve # Non-SSL
cargo run --release --bin=servessl # SSL
```

## .NET Core

```bash
cd ./servers/dotnet
dotnet run --release
```

## vlang

[Memory Leak](https://github.com/vlang/v/issues/3897)

Requires OpenSSL Development Package!

```bash
cd ./servers/vlang
v -prod run ./serve.v
```