FROM rust:latest

WORKDIR /app

# Copy Cargo files and dummy bin to make use of docker's caching and avoid
# re-downloading and re-compiling dependencies on every change to files in 'src'
COPY  Cargo.toml .
COPY  Cargo.lock .
COPY  ./bin/dummy.rs /app/bin/
RUN   cargo build --bin dummy

COPY  ./src /app/
COPY  Makefile .
