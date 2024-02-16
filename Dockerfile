FROM rust:1.73 as rust-builder

WORKDIR /usr/src/app

COPY app .

RUN cargo build --release

##############################################

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=rust-builder /usr/src/app/target/release/wuxia2kindle /app/bin

RUN apt-get update && apt install -y openssl ca-certificates curl

# Latest releases available at https://github.com/aptible/supercronic/releases
ENV SUPERCRONIC_URL=https://github.com/aptible/supercronic/releases/download/v0.2.29/supercronic-linux-amd64 \
    SUPERCRONIC=supercronic-linux-amd64 \
    SUPERCRONIC_SHA1SUM=cd48d45c4b10f3f0bfdd3a57d054cd05ac96812b

RUN curl -fsSLO "$SUPERCRONIC_URL" \
 && echo "${SUPERCRONIC_SHA1SUM}  ${SUPERCRONIC}" | sha1sum -c - \
 && chmod +x "$SUPERCRONIC" \
 && mv "$SUPERCRONIC" "/usr/local/bin/${SUPERCRONIC}" \
 && ln -s "/usr/local/bin/${SUPERCRONIC}" /usr/local/bin/supercronic

# You might need to change this depending on where your crontab is located
COPY crontab crontab


SHELL ["/bin/bash", "-o", "pipefail", "-c"]

CMD tail -f /dev/null
