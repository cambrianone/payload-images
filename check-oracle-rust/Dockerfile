FROM rust:1.86 as builder
WORKDIR /usr/src/app
COPY . /usr/src/app/
WORKDIR /usr/src/app/check-oracle-rust
RUN cargo install --path .

FROM debian
COPY --from=builder /usr/local/cargo/bin/check-oracle-rust /usr/local/bin/check-oracle-rust
CMD ["check-oracle-rust"]
