FROM rust:1.62

RUN USER=root cargo new --bin verusnft
WORKDIR /app

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release --bin verusnft

ENTRYPOINT [ "./target/release/verusnft" ]