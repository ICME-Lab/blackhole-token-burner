FROM rust:1.92-bullseye
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app

