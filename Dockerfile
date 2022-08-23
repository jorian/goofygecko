FROM lukemathwalker/cargo-chef:latest-rust-1.62.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release --bin verusnft

FROM debian:bullseye-slim AS runtime

WORKDIR /app

COPY config config 
COPY --from=builder /app/target/release/verusnft verusnft

ENV APP_ENVIRONMENT production

ENTRYPOINT [ "./verusnft" ]
