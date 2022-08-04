## Cosmoscout
Cosmoscout is an explorer service for cosmos based blockchain.

It's inherently built for enterprise-level.

### Install diesel
```shell
# if you're on linux OS, must install prerequisite of postgresql dev tools,
$ sudo apt install libpq-dev

# install diesel
$ cargo install diesel_cli --no-default-features --features postgres

# or 
$ cargo install diesel_cli --no-default-features --features sqlite

$ cd models && diesel migration run && cd ..

# or
$ diesel migration redo
```

### Run on localhost for test
```shell
# this command runs simple gaiad app & postgres database
$ docker-compose up

# this start to run explorer runtime application
$ RUST_LOG=info cargo run --bin fetcher -- --filename config.toml
```