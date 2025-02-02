all:
	cargo clippy
	cargo run

generate_secret_key:
	openssl rand -base64 32
