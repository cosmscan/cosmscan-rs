CONFIG_FILE=config.toml

.PHONY: install-diesel-postgres-linux migrate undo-migrate run-fetcher run-server

install-diesel-postgres-linux:
	@echo "intall libpq-dev if it is not installed"
	dpkg -S libpq-dev || sudo apt-get install libpq-dev

	@echo "install diesel with cargo command"
	cargo install diesel_cli --no-default-features --features postgres	

migrate: 
	@echo "Migrating database..."
	cd models && diesel migration run && cd ..

undo-migrate: 
	@echo "Migrating database..."
	cd models && diesel migration redo && cd ..

run-fetcher:
	@echo "Running fetcher..."
	RUST_LOG=info cargo run --bin engine -- --filename $(CONFIG_FILE)

run-server:
	@echo "Running server..."
	RUST_LOG=info cargo run --bin server -- --filename $(CONFIG_FILE)