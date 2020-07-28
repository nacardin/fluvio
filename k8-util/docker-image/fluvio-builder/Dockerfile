ARG RUST_BUILDER_IMAGE
FROM ${RUST_BUILDER_IMAGE}
COPY . /fluvio-src
RUN apk add --no-cache \
        libc-dev \
        openssl openssl-dev
RUN cd /fluvio-src && OPENSSL_NO_VENDOR=1 cargo build --release