# Tox handshake

This is a [Tox protocol](https://zetok.github.io/tox-spec/) client performing a handshake with specified node.

The handshake is carried out according to the diagram available [here](https://zetok.github.io/tox-spec/#handshake-diagram---client).

## Simple usage

The project contains a docker-compose file which sets up a [tox-node](https://hub.docker.com/r/toxrust/tox-node) container used for testing the handshake program.

Testing steps:

0. Docker and docker-compose installed as prerequisites
1. `docker compose up` -  start the tox-node
2. `cargo run` - run a program which is configured to connect to the node started by the docker-compose.

Logs from a successful handshake execution should look like the below:

```
Connecting to socket...
Connected
Sending initial packet
Waiting for the server's response
Handshake completed
```

## Configuration

```$ cargo run -- --help
Tox simple handshake

Usage: tox-handshake [OPTIONS]

Options:
      --server-pk <SERVER_PK>      Server's public key (hex-formatted 32 bytes) [default: 7235a7ac54ccc946192f809e716b8a4bdcd4e407f874d0522a1d7d523ec9e57c]
      --server-addr <SERVER_ADDR>  [default: 127.0.0.1:33445]
  -h, --help                       Print help
```

### Server's public key

Server's public key is by default set to correspond to the server's secret key which is set by the docker-compose file (`TOX_SECRET_KEY` env var). If the program were to connect to another node with different key, one could set the server's public key in client program by using `--server-pk`.

In case of changing the `TOX_SECRET_KEY` env, the corresponding public key can be retrieved from the containers logs.

Example logs:
```
[2023-10-04T15:11:59Z INFO  tox_node] DHT public key: 7235A7AC54CCC946192F809E716B8A4BDCD4E407F874D0522A1D7D523EC9E57C
```

### Server address

A handshake program by default connects to the container set up by the docker-compose file (`127.0.0.1:33445`) what can be changes with the `--server-addr`.