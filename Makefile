all:
	cargo clippy
	cargo run

generate_secret_key:
	openssl rand -base64 32

db_reset:
	psql -f ./src/sql/teardown.sql -d rs_party
	psql -f ./src/sql/setup.sql -d rs_party
