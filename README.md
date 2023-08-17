# rrshell

A simple async remote shell application in Rust

## Usage

1. Start the server on the remote machine:

```sh
./server <server_ip> <server_port>
```

2. Connect to the server using the client:

```sh
./client <server_ip> <server_port>
```

## Build

Simply run

```sh
cargo build --release
```

## Compatibility

It should work on Linux, Windows and MacOS.
