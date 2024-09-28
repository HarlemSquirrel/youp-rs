[![Verify](https://github.com/HarlemSquirrel/youp-rs/actions/workflows/verify.yml/badge.svg)](https://github.com/HarlemSquirrel/youp-rs/actions/workflows/verify.yml)

# youp-rs

Simple HTTP uptime CLI monitor written in Rust.

It's "youp" as in quickly saying, "you up?" to a URL.

```sh
Usage: youp [OPTIONS] <URL_STRING>

Arguments:
  <URL_STRING>  URL to make the request to

Options:
  -c, --concurrency <CONCURRENCY>  Number of workers to make concurrent requests [default: 1]
  -d, --delay <DELAY>              Delay in milliseconds between requests in each worker [default: 1000]
  -i, --iterations <ITERATIONS>    Number of requests to make from each worker [default: 10]
  -h, --help                       Print help
```

```sh
cargo run -- https://github.com
```
![image](https://github.com/user-attachments/assets/83d37cdc-d581-4cb1-b5b2-42af8de24c2f)
