FROM rust:1.71 as builder

WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/axum-stupid-api /usr/local/bin/axum-stupid-api
CMD ["axum-stupid-api"]