FROM docker.io/debian:bookworm-slim

WORKDIR /app

## copy the pi binary
COPY /bin/clipboard-server_aarch64-unknown-linux-gnu ./

COPY Rocket.toml ./

CMD [ "./clipboard-server_aarch64-unknown-linux-gnu" ]
