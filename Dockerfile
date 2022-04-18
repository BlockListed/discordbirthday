FROM rust:1.59 as builder
WORKDIR /usr/src/dbday
COPY . .
RUN cargo install --path .

FROM s6on/debian
COPY --from=builder /usr/local/cargo/bin/dbday /usr/local/bin/dbday
COPY rootfs/ /
RUN apt-get update && apt-get upgrade -y && apt-get install libpq5 -y && rm -rf /var/lib/apt/lists/*
