# FROM rust:1.70.0 as builder
FROM rust:1.69.0 as builder

RUN apt-get update && apt-get install -y \
binaryen musl-tools && rm -rf /var/lib/apt/lists/*

WORKDIR /usr
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo binstall trunk --version 0.17.5 --locked

# RUN cargo install cargo-wasi

RUN cargo build --release --target x86_64-unknown-linux-musl

WORKDIR /usr/crates/server

RUN trunk build --release -d ./dist
RUN cp robots.txt ./dist/robots.txt
RUN cargo build --release --target x86_64-unknown-linux-musl --features=ssr --bin server --

# Runtime Stage
FROM scratch

EXPOSE 8080
COPY --from=builder /usr/crates/server/dist/ /dist/
COPY --from=builder /usr/target/x86_64-unknown-linux-musl/release/server /server

ENTRYPOINT ["/server"]
CMD ["--dir", "dist"]

