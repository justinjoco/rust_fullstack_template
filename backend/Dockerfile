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
RUN cargo build --release


# Build the application
#RUN cargo build --release
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl1.1 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/app  /bin/app

ENV RUST_LOG=info

# Expose any ports if required (e.g., for web servers)
EXPOSE 5000

CMD ["/bin/app"]