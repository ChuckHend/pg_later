FROM postgres:18-bookworm as builder

RUN apt-get update \
    && apt-get install -y \
	ca-certificates \
	clang \
	curl \
	gcc \
	git \
	libssl-dev \
	make \
	pkg-config \
	postgresql-server-dev-18

COPY . .
# install pgmq
RUN make install-pgmq PGRX_PG_CONFIG=$(which pg_config)

# Install Rust dependencies
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN $HOME/.cargo/bin/rustup default stable

# install pgrx
ARG PGRX_VER=0.16.1
RUN $HOME/.cargo/bin/cargo install cargo-pgrx --version=$PGRX_VER --locked
RUN $HOME/.cargo/bin/cargo pgrx init --pg18 $(which pg_config)

# install pglater
RUN $HOME/.cargo/bin/cargo pgrx install --pg-config=$(which pg_config)

FROM postgres:18-bookworm

COPY --from=builder /usr/share/postgresql/18/extension /usr/share/postgresql/18/extension
COPY --from=builder /usr/lib/postgresql/18/lib /usr/lib/postgresql/18/lib

RUN apt-get update \
    && apt-get install -y ca-certificates

COPY images/pglater-pg/postgresql.conf /usr/share/postgresql/18/postgresql.conf.sample

USER postgres
CMD ["postgres"]
