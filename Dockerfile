ARG RUST_VERSION=1.79
ARG APP_NAME=api-informatics-frs-helper
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install cmake -y
RUN --mount=type=bind,source=src,target=src \
  --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  --mount=type=bind,source=assets,target=assets \
  --mount=type=bind,source=templates,target=templates \
  <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

FROM debian:bullseye-slim AS final
ARG UID=10001
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  appuser
USER appuser
COPY --from=build /bin/server /bin/
COPY /assets /assets
COPY /templates /templates
EXPOSE 8000

CMD ["/bin/server"]
