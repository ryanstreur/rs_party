POSTGRES_HOST=localhost
POSTGRES_USER=postgres
POSTGRES_PORT=5432
POSTGRES_DB=rs_party

NODE_MODULES_DIR=client/node_modules
RUST_TARGET_DIR=server/target

TAR_FILE=rs_party.tar.gz

all:
	docker compose up

client_run: client_setup
	cd client; \
	npm run dev

client_setup:
	cd client; \
	npm install

server_setup:
	cd server; \
	cargo doc --no-deps; \
	cargo run --bin migrate

server_run:
	cd server; \
	cargo run

server_test:
	cd server; \
	cargo check; \
	cargo clippy; \
	cargo fmt; \
	cargo test

db_setup:
	psql -h $(POSTGRES_HOST) -U $(POSTGRES_USER) -p $(POSTGRES_PORT) \
		-c "CREATE DATABASE $(POSTGRES_DB);"

db_teardown:
	psql -h $(POSTGRES_HOST) -U $(POSTGRES_USER) -p $(POSTGRES_PORT) \
		-c "DROP DATABASE $(POSTGRES_DB)"

clean: db_teardown
	rm -r $(NODE_MODULES_DIR); \
	rm -r $(RUST_TARGET_DIR)

tar:
	tar -cvaf $(TAR_FILE) \
		--exclude server/target \
		--exclude client/node_modules \
		--exclude .git \
		--exclude server/.vscode \
		--exclude client/.vscode \
		--exclude cs586-docs \
		--exclude cs594-docs \
		--exclude cs523-docs \
		--exclude volumes \
		--exclude proposal.pdf \
		--exclude *.tar \
		--exclude *.tar.gz \
		.

