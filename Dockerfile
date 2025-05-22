FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /kenja

COPY --chown=nonroot:nonroot target/release/kenja /kenja/kenja

USER nonroot:nonroot

ENTRYPOINT [ "/kenja/kenja" ]
