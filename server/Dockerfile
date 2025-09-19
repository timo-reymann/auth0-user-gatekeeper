FROM rust:1.90-bookworm AS builder

WORKDIR /usr/src/app

RUN --mount=type=cache,target=/root/.cargo,id=cargo \
    --mount=type=bind,source=src/main.rs,target=src/main.rs \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    cargo fetch

COPY . ./
RUN --mount=type=cache,target=/root/.cargo,id=cargo \
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
