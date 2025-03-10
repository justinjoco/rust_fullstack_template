FROM rust:1.85.0-bullseye as builder

# Set the working directory
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the cargo.toml and cargo.lock to the container to cache dependencies
COPY . .

RUN cargo build --release

# Now copy the actual source code

# Build the application
#RUN cargo build --release
FROM debian:bullseye

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