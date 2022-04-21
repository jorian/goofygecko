# Verusnft

## Setup

This uses Docker. In order to make it run, run `cargo sqlx prepare` to prepare the `query!()` statements into a sqlx-data.json.
Then, start docker:

`docker build --tag verusnft --file Dockerfile .`
`docker run --network="host" --name=verusnft verusnft`

Make sure postgres is running in its own container and has port 5432 mapped.