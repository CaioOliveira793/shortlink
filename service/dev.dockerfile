ARG RUST_VERSION=1.69

# rust project setup
FROM docker.io/library/rust:${RUST_VERSION}-slim-bullseye as setup

RUN echo "Dir::Cache \"\";\nDir::Cache::archives \"\";" | \
	tee /etc/apt/apt.conf.d/00_disable-cache-directories && \
	apt update --quiet && \
	apt install --quiet -y git curl

RUN useradd shortlink --create-home --home /home/shortlink --user-group

USER shortlink:shortlink

WORKDIR /home/shortlink

# NOTE: work around in slow `cargo fetch --locked`.
# https://github.com/rust-lang/cargo/issues/9177
RUN mkdir .cargo && \
	echo "[net]\ngit-fetch-with-cli = true" > .cargo/config.toml

COPY --chown=shortlink:shortlink Cargo.toml Cargo.lock .


# service-app dev build
FROM setup as service-app

COPY --chown=shortlink:shortlink crate/service-app/Cargo.toml crate/service-app/Cargo.toml

RUN cargo fetch --locked

COPY --chown=shortlink:shortlink crate/service-app crate/service-app

RUN cargo build --profile dev --bin service-app

HEALTHCHECK CMD curl --fail "http://localhost:$PORT/health" || exit 1

STOPSIGNAL SIGTERM

CMD target/debug/service-app
