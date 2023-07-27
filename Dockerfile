FROM rust:slim-bookworm as builder

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update -y && apt upgrade -y
RUN apt install build-essential cmake -y

WORKDIR /usr/src/codectrl
COPY . .
RUN cargo install --no-default-features --features server-only --path .

CMD ["codectrl"]
