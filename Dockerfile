FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM alpine:3.21

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/squaredle-cli /usr/local/bin/squaredle-cli

ENTRYPOINT ["/usr/local/bin/squaredle-cli"]
