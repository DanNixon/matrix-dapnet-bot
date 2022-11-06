FROM docker.io/library/rust:alpine3.15 as builder

RUN apk add \
  cmake \
  g++ \
  libc-dev \
  make \
  openssl-dev

COPY . .
RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo install \
  --path . \
  --root /usr/local

FROM docker.io/library/alpine:3.15

RUN apk add \
  libgcc \
  libstdc++ \
  tini

COPY --from=builder \
  /usr/local/bin/matrix-dapnet-bot \
  /usr/local/bin/matrix-dapnet-bot

RUN mkdir /config

ENV OBSERVABILITY_ADDRESS "0.0.0.0:9090"
EXPOSE 9090

ENTRYPOINT ["/sbin/tini", "--", "/usr/local/bin/matrix-dapnet-bot", "--config-file", "/config/config.toml"]
