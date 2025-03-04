FROM gcr.io/distroless/base-debian12:nonroot

WORKDIR /kenja

COPY --chown=nonroot:nonroot target/release/kenja /kenja/kenja

USER nonroot:nonroot

CMD [ "/kenja/kenja" ]
