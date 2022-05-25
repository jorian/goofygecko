FROM rust:1.59

RUN USER=root cargo new --bin verusnft
WORKDIR /app

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release --bin verusnft

ENV DATABASE_URL postgres://postgres:password@localhost:5432/test0
ENV DATABASE_URL2 postgres://postgres:password@localhost:5432/test0
ENV DISCORD_TOKEN OTU4MDU4MjgzMDc4Mzk4MDQy.YkHzTg.Wv4jRyS1HXg4uxdKq7PsY6YTtwQ
ENTRYPOINT [ "./target/release/verusnft" ]