FROM rust:1.78.0 AS builder

WORKDIR /usr/src/app

COPY backend/default ./default
COPY backend/Cargo.toml backend/Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

RUN rm -f target/release/deps/backend*

COPY backend/src ./src

RUN cargo build --release

RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y wget curl xz-utils libpq-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/app
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

ENV DOCKERIZE_VERSION v0.6.1

RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
    tar -C /usr/local/bin -xzvf dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
    rm dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /usr/src/app/backend

COPY backend/page ./page
COPY backend/data ./data
COPY backend/migrations ./migrations
COPY backend/diesel.toml .

EXPOSE 8080

COPY entrypoint.sh /usr/src/app/entrypoint.sh
RUN chmod +x /usr/src/app/entrypoint.sh

ENTRYPOINT ["/usr/src/app/entrypoint.sh"]
CMD ["app"]