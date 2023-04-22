FROM rust as build

COPY ./ ./

RUN cargo build --release

FROM gcr.io/distroless/static:nonroot

COPY --from=build ./target/release/gitops-tui /bin/gitops-tui

ENTRYPOINT [ "gitops-tui" ]
