# syntax=docker/dockerfile:1

FROM debian:bookworm-slim

EXPOSE 3000

WORKDIR /bin

COPY target/x86_64-unknown-linux-musl/release/ocpp ./
COPY target/x86_64-unknown-linux-musl/release/api ./

CMD ["/bin/ocpp"]