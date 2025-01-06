# syntax=docker/dockerfile:1

FROM debian:bookworm-slim

EXPOSE 3000

CMD ["/bin/server"]