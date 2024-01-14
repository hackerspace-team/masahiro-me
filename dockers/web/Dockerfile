# FROM rust:1.70.0 as builder
FROM rust:1.70.0-bookworm

RUN apt-get update && apt-get install -y \
  binaryen \
  musl-tools \
  make \
  yarn \
  && rm -rf /var/lib/apt/lists/*


# Yarnリポジトリの追加とインストール
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \
  && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list \
  && apt-get update && apt-get install -y yarn


WORKDIR /usr/src/app
COPY . .

# RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --version 0.17.5 --locked

EXPOSE 8080
