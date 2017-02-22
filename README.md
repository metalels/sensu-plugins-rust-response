# Sensu-Plugins-Rust-response

Sensu plugins uses Rust.

## Installation ##

  1. git clone https://github.com/metalels/sensu-plugins-rust-response.git
  2. execute metrics/metrics-response-url or metrics/metrics-response-socket

## Dependencies of compile ##

* Rust
* Cargo
* and see Cargo.toml

## Usage ##

```
Usage: metrics-response-url URLS [options]

Requires:
  URLS: URL,URL,...  set target url(s: use [,] to joins multi urls)

Options:
  -p, --prefix PREFIX set prefix to output keys
  -t, --http2         enable to ignore http2 access (current not supported)
  -i, --ignore        enable to ignore ssl certification error
  -d, --debug         print debug logs
  -h, --help          print this help menu
```

## Authors ##

[metalels](https://github.com/metalels)

