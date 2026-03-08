# Build stage
FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/portfolio-backend .

ENV PORT=8080

EXPOSE 8080

CMD ["./portfolio-backend"]