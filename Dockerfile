ARG BASE
FROM ${BASE}

RUN apk add --no-cache musl-dev || true

COPY . /app
WORKDIR /app

RUN cargo build --release
