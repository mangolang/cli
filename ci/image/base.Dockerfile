
# Docker image that contains the pre-compiled dependencies of Mango.
# Only intended for use in the Mango project itself, to make other images build faster.
# https://hub.docker.com/r/mangocode/mango_daily_base

# Note: this version should match `executable.Dockerfile`
# Note: also update nightly version, installed later in image
FROM ekidd/rust-musl-builder:1.44.1

ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-deny
RUN cargo install cargo-tree
RUN cargo install cargo-udeps
RUN cargo install cargo-bloat

# Nightly is needed for grcov and miri.
#TODO @mark: separate image for nightly
#ENV NIGHTLY_VERSION=nightly-2020-07-22
#RUN rustup toolchain install $NIGHTLY_VERSION

WORKDIR /mango

# Add the files needed to compile dependencies.
COPY --chown=rust Cargo.toml .
COPY --chown=rust Cargo.lock .
RUN sudo chown rust:rust -R . && \
    sudo chmod g+s -R . && \
    mkdir -p src && \
    printf 'fn main() { println!("placeholder for compiling dependencies") }' > src/main.rs

# Build the code (development mode).
RUN cargo build --tests --bin mango

# Build the code (release mode).
# Note: sharing dependencies between dev/release does not work yet - https://stackoverflow.com/q/59511731
RUN cargo build --tests --bin mango --release
#TODO: use --out-dir if it stabilizes

# Build the code with special flags for code coverage.
#TODO @mark: separate image for nightly
#COPY --chown=rust ci/image/cargo_for_coverage.sh cargo_for_coverage.sh
#RUN ./cargo_for_coverage.sh build

# Remove Cargo.toml file, to prevent other images from forgetting to re-add it.
RUN rm -f cargo_for_coverage.sh Cargo.toml

## NOTE!
## Make sure to `touch src/main.rs` after copying source, so that everything is recompiled

