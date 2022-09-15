CONFIG_FILE=config.toml

.PHONY: migrate undo-migrate run-fetcher run-server

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