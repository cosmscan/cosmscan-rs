FROM rust:1.63-alpine3.15

WORKDIR /app
COPY . .

RUN apk update && apk add --no-cache libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install --path .

RUN cargo build --release