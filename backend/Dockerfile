FROM rust:1.85.0-slim-bullseye as builder

# Set the working directory
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the cargo.toml and cargo.lock to the container to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now copy the actual source code
COPY . .

RUN cargo install sqlx-cli --no-default-features --features postgres

RUN cargo build --release


# Build the application
#RUN cargo build --release
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    netcat \
    libssl1.1 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*


COPY --from=builder /usr/src/app/docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY --from=builder /usr/src/app/migrations /app/migrations

COPY --from=builder /usr/src/app/target/release/app  /bin/app

ENV RUST_LOG=info

WORKDIR /app

# Expose any ports if required (e.g., for web servers)
EXPOSE 5000

RUN chmod +x /usr/local/bin/docker-entrypoint.sh
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
