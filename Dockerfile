# Stage 1: Build
FROM rust:1.92 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /app
# Copy the binary from the builder stage
COPY --from=builder /app/target/release/http-server-tokio .
COPY --from=builder /app/public ./public
EXPOSE 8080
CMD ["./http-server-tokio"]
