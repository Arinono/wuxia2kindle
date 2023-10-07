FROM rust:1.73 as rust-builder

WORKDIR /usr/src/app

COPY app .

RUN cargo build --release

##############################################

FROM debian:bookworm-slim

COPY --from=rust-builder /usr/src/app/target/release/wuxia2kindle /app

RUN apt-get update && apt install -y openssl ca-certificates curl unzip parallel make

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN curl -fsSL https://deno.land/x/install/install.sh | bash; \
  /root/.deno/bin/deno upgrade --version 1.37.1

ENV PATH=${PATH}:/root/.deno/bin

WORKDIR /client

COPY client .

WORKDIR /

COPY Makefile .

CMD parallel --linebuffer --halt now,success=1,fail=1 make ::: ingest worker web
