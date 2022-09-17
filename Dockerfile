FROM rust:1.63-buster AS build-env

WORKDIR /app
COPY . .

RUN apt update && apt install -y libpq-dev build-essential

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release