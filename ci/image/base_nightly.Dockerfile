
# Nightly version of `base.Dockerfile`.
# * Only debug mode.
# * Also useful for non-musl checks.
# Based on https://github.com/rust-lang/docker-rust-nightly/blob/master/buster/Dockerfile

# Nightly is needed for grcov and miri.
FROM buildpack-deps:buster

RUN apt-get -y update \
    && apt-get install -y zip \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_BACKTRACE=1 \
    RUSTC_WRAPPER="" \
    CARGO_INCREMENTAL=0 \
    RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort -Zmacro-backtrace"

RUN set -eux; \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly-2020-07-12; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

WORKDIR /mango

RUN cargo install xargo
RUN rustup component add rust-src
RUN rustup component add miri
RUN cargo install grcov
RUN cargo install cargo-udeps

# Add the files needed to compile dependencies.
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir -p cli/src daemon/src common/src && \
    printf 'fn main() {\n\tprintln!("placeholder for compiling stable dependencies")\n}' | tee cli/src/main.rs | tee daemon/src/main.rs | tee common/src/lib.rs
COPY --chown=rust cli/Cargo.toml ./cli/Cargo.toml
COPY --chown=rust daemon/Cargo.toml ./daemon/Cargo.toml
COPY --chown=rust common/Cargo.toml ./common/Cargo.toml

# Build the code (development mode).
RUN cargo build --tests

# Build the code with special flags for code coverage.
COPY ci/image/cargo_for_coverage.sh cargo_for_coverage.sh
RUN ./cargo_for_coverage.sh build

# Remove Cargo.toml files, to prevent other images from forgetting to re-add it.
RUN rm -f cargo_for_coverage.sh Cargo.toml

## NOTE!
## Make sure to `touch cli/src/main.rs`, `touch daemon/src/main.rs` and `touch common/src/lib.rs` after copying source, so that everything is recompiled

