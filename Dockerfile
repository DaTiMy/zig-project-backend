FROM rust:1.88

RUN apt-get update && apt-get install -y libmariadb-dev libmariadb-dev-compat pkg-config

WORKDIR /app
COPY . .

RUN cargo install diesel_cli --no-default-features --features mysql

RUN cargo build --release

CMD ["./target/release/zig-project"]