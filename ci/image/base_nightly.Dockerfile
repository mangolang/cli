
# Nightly version of `base.Dockerfile`.
# * Only debug mode.
# * Also useful for non-musl checks.

# Nightly is needed for grcov and miri.
FROM buildpack-deps:buster

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_BACKTRACE=1

RUN set -eux; \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

WORKDIR /mango

RUN cargo +nightly install xargo
RUN rustup +nightly component add miri

# Add the files needed to compile dependencies.
COPY --chown=rust Cargo.toml .
COPY --chown=rust Cargo.lock .
RUN mkdir -p src && \
    printf 'fn main() { println!("placeholder for compiling nightly dependencies") }' > src/main.rs

# Build the code (development mode).
RUN cargo build --tests --bin mango

# Build the code with special flags for code coverage.
COPY --chown=rust ci/image/cargo_for_coverage.sh cargo_for_coverage.sh
RUN ./cargo_for_coverage.sh build

# Remove Cargo.toml file, to prevent other images from forgetting to re-add it.
RUN rm -f cargo_for_coverage.sh Cargo.toml

## NOTE!
## Make sure to `touch src/main.rs` after copying source, so that everything is recompiled

