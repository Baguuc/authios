FROM rust:latest AS build

# Install musl-tools for cross-compilation to Alpine
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY . .

# Set up musl target
RUN rustup target add x86_64-unknown-linux-musl

# Build for musl
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy the binary to a standard location (not /bin)
RUN mkdir -p /usr/local/bin

# Final stage: Alpine
FROM alpine:latest AS final

WORKDIR /app

COPY --from=build /build/target/x86_64-unknown-linux-musl/release/authin-presentation /usr/local/bin/authin

CMD ["authin", "run"]
