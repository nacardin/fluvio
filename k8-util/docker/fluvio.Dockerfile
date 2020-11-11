ARG RELEASE_FLAG
FROM rust:1-alpine3.12 as fluvio-builder
RUN apk add --no-cache musl-dev
COPY fluvio-src /tmp/fluvio-src
WORKDIR /tmp/fluvio-src
RUN cd src/cli && cargo build $RELEASE_FLAG --no-default-features \
    --features cluster_components_rustls --bin fluvio

FROM alpine:3.12
COPY --from=fluvio-builder /tmp/fluvio-src/target/release/fluvio fluvio
