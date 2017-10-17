# For dev and CI convenience.

FROM ruby:jessie

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    bison \
    build-essential \
    ca-certificates \
    curl \
    g++ \
    git \
    libssl-dev \
    pkg-config \
    ragel \
  && rm -rf /var/lib/apt/lists/*

ARG RUST_VERSION=1.21.0
ARG RUST_NAME=rust-$RUST_VERSION-x86_64-unknown-linux-gnu
ARG RUST_ARCHIVE=$RUST_NAME.tar.gz
ARG RUST_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN curl -sO $RUST_URL \
  && tar -xzf $RUST_ARCHIVE \
  && ./$RUST_NAME/install.sh --without=rust-docs \
  && rm -rf $RUST_NAME $RUST_ARCHIVE

WORKDIR /src
ENV CARGO_HOME=/src/.cargo
