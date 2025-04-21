# WS-RS

WebSocket server written in Rust.

## Build Docker image

```sh
docker build . -t ws-rs-test -f ./Dockerfile
```

## Run a container

```sh
docker run -p 8000:8000 ws-rs-test
```

You should kill container by some way to reuse the port 8000.
