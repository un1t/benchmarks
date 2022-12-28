build:
	cargo check && cargo build --release

dev:
	cargo watch -x check -x run

run: build
	./target/release/server

db:
	psql -U postgres -h localhost rsdict_dev
