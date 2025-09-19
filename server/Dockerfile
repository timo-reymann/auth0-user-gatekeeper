FROM rust:1.90-bookworm AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends sccache && \
    rm -rf /var/lib/apt/lists/*
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_START_SERVER=0

WORKDIR /usr/src/app

RUN --mount=type=cache,target=/root/.cargo,id=cargo \
    --mount=type=cache,target=/root/.cache/sccache,id=sccache \
    --mount=type=bind,source=src/main.rs,target=src/main.rs \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    cargo fetch

COPY . ./
RUN --mount=type=cache,target=/root/.cache/sccache,id=sccache \
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
