> ⚠️ Caution ⚠️
> 
> Cosmscan is not ready to go in production.
> 
> This is very rapidly changed and under developement.

## Cosmscan
Opensource indexer engine & explorer web for cosmos blockchain.

## Motivation
In ethereum, Blockscout is best opensource blockchain explorer, [as you can see here](https://blockscout.com/eth/mainnet/)
It seems super-duper enterprise level blockchain explorer, and the cool thing is that it's open source!

But, In cosmos ecosystem there is no fansy opensource explorer.
So then many teams rely on validators who built un-opened blockchain explorer such as mintscan.

Therefore, I wished to build opensource blockchain explorer and anyone can install their own server.
It would be very helpful for teams to only focus on developing appchain.

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
$ RUST_LOG=info cargo run --bin indexer -- --filename config.toml
```

## Contribution Guidelines
Feel free to open an issue or pull request.

![Database](./docs/images/db.png)
