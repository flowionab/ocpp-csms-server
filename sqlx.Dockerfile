FROM rust:1.83

WORKDIR /app

RUN cargo install sqlx-cli

COPY migrations/ migrations/

ENTRYPOINT ["cargo", "sqlx", "migrate", "run"]