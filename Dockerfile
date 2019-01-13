FROM rust:1.31

WORKDIR /usr/src/backup-rs
COPY . .

RUN cargo install --path .

ENTRYPOINT ["backup"]
