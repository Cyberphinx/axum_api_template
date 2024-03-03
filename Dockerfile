# Stage 1: Building
FROM rust:1.74-alpine as builder

# Install musl-tools
RUN apk add --no-cache musl-dev build-base

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY . .

# Build the binary with musl target
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime image
FROM alpine:latest

# Create an application user
RUN addgroup -S app && adduser -S app -G app

USER app
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/api /app/api

# Run the app
CMD ["./api"]
