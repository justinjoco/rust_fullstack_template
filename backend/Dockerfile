FROM rust:1.85.0:alpine3.21 as builder

# Install system dependencies (optional)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config

# Set the working directory
WORKDIR /usr/src/app

# Copy the cargo.toml and cargo.lock to the container to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build the dependencies first (this allows Docker to cache dependencies)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

FROM alpine:3.21.3

# Install any runtime dependencies if needed (e.g., for accessing SSL libraries)
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*


COPY --from=builder /usr/src/app/target/release/app  /bin/app

# Expose any ports if required (e.g., for web servers)
EXPOSE 5000

CMD ["/bin/app"]