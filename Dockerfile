FROM rust:1.63-buster AS build-env

WORKDIR /app

COPY . .

RUN apt update && apt install -y libpq-dev build-essential netcat

RUN cargo install diesel_cli --no-default-features --features postgres

FROM build-env AS build

WORKDIR /app

RUN cargo build --release