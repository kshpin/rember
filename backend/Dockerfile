FROM rust:1.88.0-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock .
COPY src src
RUN cargo build --release

FROM alpine:latest AS runner

COPY --from=builder /usr/src/app/target/release/rember-backend /usr/local/bin/rember-backend

EXPOSE 3210
CMD ["rember-backend"]
