FROM rust:1.75 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian
COPY --from=builder /usr/local/cargo/bin/check-oracle-rust /usr/local/bin/check-oracle-rust
CMD ["check-oracle-rust"]
