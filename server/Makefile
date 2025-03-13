all:
	cargo clippy
	cargo run

generate_secret_key:
	openssl rand -base64 32

db_reset:
	psql -h localhost -U postgres -d rs_party -f ./src/sql/teardown.sql -d rs_party
	psql -h localhost -U postgres -d rs_party -f ./src/sql/setup.sql -d rs_party

rfc: ./cs594-docs/draft-rfc.md
	kramdown-rfc ./cs594-docs/draft-rfc.md > cs594-docs/draft-rfc.xml
	xml2rfc ./cs594-docs/draft-rfc.xml
