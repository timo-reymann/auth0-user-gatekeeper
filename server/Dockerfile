FROM rust:1.90-bookworm AS builder

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

COPY --from=builder \
     --chown=server:server \
     /usr/src/app/target/release/auth0-user-gatekeeper /bin/auth0-user-gatekeeper

USER server
ENTRYPOINT ["/bin/auth0-user-gatekeeper"]
