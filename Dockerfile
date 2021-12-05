FROM rust:latest as builder
WORKDIR /usr/src/dbday
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get upgrade -y && apt-get install sqlite3 -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/dbday /usr/local/bin/dbday
CMD ["dbday"]