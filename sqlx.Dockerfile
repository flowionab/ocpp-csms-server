FROM rust:1.87

WORKDIR /app

RUN cargo install sqlx-cli

COPY migrations/ migrations/

ENTRYPOINT ["cargo", "sqlx", "migrate", "run"]