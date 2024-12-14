FROM rust:latest as builder

WORKDIR /usr/src/utf-nate
COPY . .

RUN setup/debian-build-setup.sh
RUN cargo install --path .

FROM debian:bookworm-slim

COPY --from=builder /usr/src/utf-nate/setup/debian-run-setup.sh /tmp/setup/debian-run-setup.sh
RUN /tmp/setup/debian-run-setup.sh
RUN rm /tmp/setup/debian-run-setup.sh
COPY --from=builder /usr/local/cargo/bin/utf-nate /usr/local/bin/utf-nate

CMD ["utf-nate"]
