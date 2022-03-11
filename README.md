# matrix-dapnet-bot [![CI](https://github.com/DanNixon/matrix-dapnet-bot/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/DanNixon/matrix-dapnet-bot/actions/workflows/ci.yml)

Matrix bot that allows you to send pages over [DAPNET](https://www.hampager.de/).

## Usage

See `matrix-dapnet-bot --help`.
An example configuration file is provided [here](./examples/config.toml).

## Deployment

A container image is published.
Use it however you like.

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

## Permissions/Authentication

For sending private pages/messages/calls the bot ensures that the sender holds a valid amateur radio license.
This is done by checking the configuration file.

For sending news to a rubric the bot checks that both the sender and bot operator are owners of the rubric.
This preserves the expected access control of only rubric owners being able to publish to them.
This does, however, mean that only rubrics that the bot operator owns can be published to, regardless of the rubrics any other user of the bot owns.
All in all, it is a bit of a hack, but good enough for what it was developed for and better than having to handle credentials for all bot users.

In all cases the owner (sender) of any private messages and rubric news messages will be shown to be the bot operator.
