ARG RUST_VERSION=1.84.1
FROM rust:${RUST_VERSION} AS build
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /app/target/release/api-informatics-frs-helper /
COPY --from=build /app/assets /assets
COPY --from=build /app/templates /templates

CMD ["./api-informatics-frs-helper"]
