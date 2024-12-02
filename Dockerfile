# syntax=docker/dockerfile:1

FROM --platform=linux/arm64 rust:1.82 AS builder

WORKDIR /app
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y pkg-config openssl libssl-dev ca-certificates
RUN \
  apt-get install ca-certificates && \
  apt-get clean

COPY . .

RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build --locked --release --target aarch64-unknown-linux-gnu
RUN cp ./target/aarch64-unknown-linux-gnu/release/ocpp_csms_server /bin/server

FROM --platform=linux/arm64 debian:bookworm-slim AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

EXPOSE 3000

COPY --from=builder /bin/server /bin/

CMD ["/bin/server"]