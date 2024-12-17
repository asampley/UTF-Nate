FROM rust:latest AS builder

WORKDIR /usr/src/utf-nate

COPY setup .
RUN apt update
RUN yes | setup/debian-build-setup.sh

COPY Cargo.toml src/ templates/ .
RUN cargo install --path .

FROM debian:bookworm-slim

COPY setup/debian-run-setup.sh /tmp/setup/debian-run-setup.sh
RUN apt update
RUN yes | /tmp/setup/debian-run-setup.sh
RUN rm /tmp/setup/debian-run-setup.sh

COPY --from=builder /usr/local/cargo/bin/utf-nate /usr/local/bin/utf-nate

WORKDIR /opt/utf-nate

ENTRYPOINT ["utf-nate"]
