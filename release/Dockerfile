FROM charliesome/musl-cross:r1

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
    ruby \
  && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh /dev/stdin -y
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl

WORKDIR /workspace
ENV CARGO_HOME=/workspace/.cargo

