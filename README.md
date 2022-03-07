# matrix-dapnet-bot [![CI](https://github.com/DanNixon/matrix-dapnet-bot/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/DanNixon/matrix-dapnet-bot/actions/workflows/ci.yml)

Matrix bot that allows you to send pages over [DAPNET](https://www.hampager.de/).

## Usage

See `matrix-dapnet-bot --help`.
An example configuration file is provided [here](./examples/config.toml).

## Deployment

A container image is published.
Us it however you like.

e.g. via Podman:
```sh
podman run \
  --rm -it \
  -e RUST_LOG=debug \
  -e DAPNET_USERNAME="<username>" \
  -e DAPNET_PASSWORD="<password>" \
  -e MATRIX_USERNAME="@<username>:matrix.org" \
  -e MATRIX_PASSWORD="<password>" \
  -v ./examples:/config \
  ghcr.io/DanNixon/matrix-dapnet-bot:latest
```
