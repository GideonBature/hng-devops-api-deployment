# HNG DevOps API Deployment

This project is a simple REST API built with Rust and Axum for the HNG DevOps Stage 1 task.

The application exposes three GET endpoints and is designed to run behind Nginx, with the API process listening on a private local port.

## Live Deployment

Base URL: `https://gideonbature.duckdns.org`

## Run Locally

### Prerequisites

- Rust
- Cargo

### Start the API

```bash
cargo run
```

The app listens on `127.0.0.1:3000` by default.

To run on a different port:

```bash
PORT=4000 cargo run
```

### Run Tests

```bash
cargo test
```

## API Endpoints

### `GET /`

Response:

```json
{"message":"API is running"}
```

### `GET /health`

Response:

```json
{"message":"healthy"}
```

### `GET /me`

Response:

```json
{"name":"Gideon Bature","email":"infoaboutgideon@gmail.com","github":"https://github.com/GideonBature"}
```

## Example Requests

```bash
curl https://gideonbature.duckdns.org/
curl https://gideonbature.duckdns.org/health
curl https://gideonbature.duckdns.org/me
```

## Deployment Notes

- API framework: Axum
- Runtime: Tokio
- Reverse proxy: Nginx
- Process manager on server: systemd
