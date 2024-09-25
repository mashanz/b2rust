FROM rust:1.80.1-bullseye AS develop
WORKDIR /app
COPY . .
RUN cargo install cargo-watch --locked
RUN cargo build
CMD ["cargo", "watch", "-x", "run"]

