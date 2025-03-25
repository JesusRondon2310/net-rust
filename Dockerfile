FROM rust:1.85-slim

#Install system dependencies INSIDE THE CONTAINER
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    pkg-config \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["/app/target/release/rust_code"]
