FROM rust:1.87 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/portfolio-backend .

ENV PORT=8080

CMD ["./portfolio-backend"]