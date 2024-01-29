# Actix JWT Actor Stock

This repository provides of:

- Actix REST API
- Actix CORS config
- Error handling
- JWT authentication
- Interaction with the MySql database
- Password encryption
- Payload validation
- OpenAPI Document `http://localhost:8080/docs/`

## Required

- Rust
- Docker and docker-compose or Postgresql server

## Usage

- edit .env
- `cargo run --release` or `debug with vscode`

## Docker build

- `docker buildx build . -t aimdevgroup/sample-actix-webapi  --platform linux/amd64 --push`
