FROM rust:1.81

COPY ./ /app

WORKDIR /app

ENV RUST_LOG=info
ENV HUMUS_SOCKET_ADDRESS="0.0.0.0:3030"

RUN cargo build --release

ENTRYPOINT ["target/release/humus"]

