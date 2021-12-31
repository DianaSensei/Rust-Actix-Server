#-----------------------------
#   BUILD STAGE
#-----------------------------

FROM rust:latest AS base

ENV USER=root

RUN apt-get update && apt-get install libsasl2-dev

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch

COPY src /code/src
COPY migrations /code/migrations
COPY locales /code/locales
COPY build.rs /code/build.rs

RUN cargo test

FROM base AS builder

RUN cargo build --release

#---------------------------------
#   FINAL STAGE
#---------------------------------

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /code/target/release/backend_rust /usr/bin/backend_rust

ENV RUSTFLAGS=-Awarnings

EXPOSE 3000

ENTRYPOINT [ "/usr/bin/backend_rust" ]