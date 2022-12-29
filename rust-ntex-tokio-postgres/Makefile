build:
	cargo check && cargo build --release

dev:
	cargo watch -x check -x run

run:
	cargo watch -x check -x "run --release"

db:
	psql -U postgres -h localhost rsdict_dev
