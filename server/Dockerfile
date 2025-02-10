FROM docker.io/debian:bookworm-slim

WORKDIR /app

## copy the pi binary
COPY /bin/clipboard-server_aarch64-unknown-linux-gnu ./

## copy runtime assets which may or may not exist
COPY Rocket.tom[l] ./static

CMD [ "./clipboard-server_aarch64-unknown-linux-gnu" ]
