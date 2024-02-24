FROM rust:1.73 as rust-builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

##############################################

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=rust-builder /usr/src/app/target/release/wuxia2kindle /app/bin

RUN apt-get update && apt install -y openssl ca-certificates curl

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

CMD ["/app/bin"]
