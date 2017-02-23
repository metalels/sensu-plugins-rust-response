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
  URLS: URL,URL,...      set target url(s: use [,] to joins multi urls)

Options:
  -p, --prefix PREFIX    set prefix to output keys
  -t, --timeout TIMEOUT  set timeout secounds to http1/2 client
  -2, --http2            enable to http2 access(current not support)
  -i, --ignore           enable to ignore ssl certification error
  -d, --debug            print debug logs
  -h, --help             print this help menu
```

## Authors ##

[metalels](https://github.com/metalels)

