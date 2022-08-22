FROM rust:1.62 as builder

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release --bin verusnft

FROM debian:bullseye-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/verusnft verusnft

ENV DATABASE_URL postgres://postgres:password@localhost:5432/test0
ENV DATABASE_URL2 postgres://postgres:password@localhost:5432/test0
ENV DISCORD_TOKEN OTU4MDU4MjgzMDc4Mzk4MDQy.YkHzTg.Wv4jRyS1HXg4uxdKq7PsY6YTtwQ
ENTRYPOINT [ "./verusnft" ]
