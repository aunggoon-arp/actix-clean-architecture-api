# ---------------------------------------------------
# 1 - Build Stage
# ---------------------------------------------------
FROM rust:1.75.0 as build

# Setup working directory
WORKDIR /usr/src/sample_actix_webapi
COPY . .
COPY .env.prod .env

# Build application
RUN cargo install --path .

# ---------------------------------------------------
# 2 - Deploy Stage
# ---------------------------------------------------
FROM debian:bookworm-slim

# Set the architecture argument (arm64, i.e. aarch64 as default)
ARG ARCH=x86_64

RUN apt-get update && apt-get install openssl -y

# Application files
COPY --from=build /usr/local/cargo/bin/sample_actix_webapi /usr/local/bin/sample_actix_webapi
COPY --from=build /usr/src/sample_actix_webapi/.env /.env
COPY --from=build /usr/src/sample_actix_webapi/upload /upload

EXPOSE 8080

CMD ["sample_actix_webapi"]