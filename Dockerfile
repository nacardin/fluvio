FROM rust:1-alpine3.13 as builder

RUN apk add pkgconfig openssl-dev musl-dev

RUN mkdir /fluvio

COPY src /fluvio/src
COPY examples /fluvio/examples
COPY tests /fluvio/tests
COPY .cargo /fluvio/.cargo
COPY VERSION Cargo.* /fluvio/

WORKDIR /fluvio

RUN mkdir target

RUN --mount=type=tmpfs,target=/tmp \
    --mount=type=cache,target=/fluvio/target \
    X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_NO_VENDOR=1 \
    cargo build --bin fluvio-run --release && \
    cp -r /fluvio/target/release/fluvio-run /fluvio-run

FROM scratch as fluvio

COPY --from=builder /fluvio-run .