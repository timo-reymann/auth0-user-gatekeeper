FROM rust:1.93-bookworm AS builder

ARG SCCACHE_VERSION=0.10.0
ENV SCCACHE_DIR=/root/.cache/sccache
RUN curl -L "https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz" -o /tmp/sccache-archive.tar.gz \
    && tar -xvzf /tmp/sccache-archive.tar.gz -C /tmp \
    && mv /tmp/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache

ENV RUSTC_WRAPPER=sccache

WORKDIR /usr/src/app

RUN --mount=type=cache,target=/root/.cargo,id=cargo \
    --mount=type=cache,target=/root/.cache/sccache,id=sccache \
    --mount=type=bind,source=src/main.rs,target=src/main.rs \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    cargo fetch

COPY . ./
RUN --mount=type=cache,target=/root/.cargo,id=cargo \
    --mount=type=cache,target=/root/.cache/sccache,id=sccache \
    cargo build --release

FROM cgr.dev/chainguard/wolfi-base
RUN adduser -D server \
    && mkdir -p /etc/auth0-user-gatekeeper \
    && chown server:server /etc/auth0-user-gatekeeper
RUN apk add --no-cache tini
COPY --from=builder \
     --chown=server:server \
     /usr/src/app/target/release/auth0-user-gatekeeper /bin/auth0-user-gatekeeper

LABEL 
LABEL org.opencontainers.image.licenses="unlicense"org.opencontainers.image.title="auth0-user-gatekeeper"
LABEL org.opencontainers.image.description="Admit users to applications through auth0 based on mail address"
LABEL org.opencontainers.image.ref.name="main"
LABEL org.opencontainers.image.licenses='Unlicense'
LABEL org.opencontainers.image.vendor="Timo Reymann <mail@timo-reymann.de>"
LABEL org.opencontainers.image.authors="Timo Reymann <mail@timo-reymann.de>"
LABEL org.opencontainers.image.url="https://github.com/timo-reymann/auth0-user-gatekeeper"
LABEL org.opencontainers.image.documentation="https://github.com/timo-reymann/auth0-user-gatekeeper"
LABEL org.opencontainers.image.source="https://github.com/timo-reymann/auth0-user-gatekeeper.git"

USER server
ENTRYPOINT ["tini", "--", "/bin/auth0-user-gatekeeper"]
CMD ["--config", "/etc/auth0-user-gatekeeper/config.yml"]
