ARG ALPINE_VERSION
ARG FLUVIO_BUILDER_IMAGE
FROM $FLUVIO_BUILDER_IMAGE as fluvio-builder

FROM alpine:${ALPINE_VERSION}
COPY --from=fluvio-builder /fluvio-src/target/release/spu-server /fluvio/spu-server
CMD ["/fluvio/spu-server"]