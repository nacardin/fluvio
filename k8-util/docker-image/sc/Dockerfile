ARG ALPINE_VERSION
ARG FLUVIO_BUILDER_IMAGE
FROM $FLUVIO_BUILDER_IMAGE as fluvio-builder

FROM alpine:$ALPINE_VERSION
COPY --from=fluvio-builder /fluvio-src/target/release/sc-k8-server /fluvio/sc-k8-server
RUN apk add --no-cache openssl
CMD ["/fluvio/sc-k8-server"]