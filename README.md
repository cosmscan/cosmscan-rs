## Cosmoscout
Cosmoscout is an explorer service for cosmos based blockchain.

It's inherently built for enterprise-level.

## Setting up the Database.
### Install diesel
```shell
# if you're on linux OS, must install prerequisite of postgresql dev tools,
$ sudo apt install libpq-dev

# install diesel
$ cargo install diesel_cli --no-default-features --features postgres
```

### Create set of tables.
```shell
$ cd models && diesel migration run && cd ..
```

If you want to create tables to remote database, not local, Change the DSN in `models/.env` file before running the above command

### Revoke migration
```shell
$ diesel migration redo
```

## Run on localhost for test
```shell
# this command runs simple gaiad app & postgres database
# when you run services via this scripts, sample tx will be automatically sent every seconds.
$ docker-compose -f docker-compose.flood.yml up

# migrate schema
$ cd models && diesel migration run && cd ..

# this start to run explorer runtime application
$ RUST_LOG=info cargo run --bin engine -- --filename config.toml
```

## Contribution Guidelines
If you need more feature, Please open the issue.

![Database](./docs/images/db.png)