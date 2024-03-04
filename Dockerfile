FROM rust:bookworm AS builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install cmake -y
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --release && \
  cp ./target/release/api-informatics-frs-helper /

FROM debian:bookworm-slim AS final
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser
COPY --from=builder /api-informatics-frs-helper /usr/local/bin
RUN chown appuser /usr/local/bin/api-informatics-frs-helper
COPY --from=builder /app /opt/api-informatics-frs-helper
RUN chown -R appuser /opt/api-informatics-frs-helper
USER appuser
WORKDIR /opt/api-informatics-frs-helper
ENTRYPOINT ["api-informatics-frs-helper"]
EXPOSE 8080/tcp
