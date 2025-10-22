FROM clux/muslrust:stable AS builder
WORKDIR /usr/src/app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release
RUN ls -lh target

FROM alpine:latest
COPY --from=builder /usr/src/app/target/*-unknown-linux-musl/release/rust-mcp-filesystem-parsers /usr/local/bin/rust-mcp-fs-parser

RUN adduser -D -s /bin/sh rust-mcp-user
USER rust-mcp-user

ENTRYPOINT ["/usr/local/bin/rust-mcp-fs-parser"]