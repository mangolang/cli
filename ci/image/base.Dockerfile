
# Docker image that contains the pre-compiled dependencies of Mango.
# Only intended for use in the Mango project itself, to make other images build faster.
# https://hub.docker.com/r/mangocode/mango_daily_base

# Note: this version should match `executable.Dockerfile`
FROM ekidd/rust-musl-builder:1.50.0

ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-deny
RUN cargo install cargo-tree
RUN cargo install cargo-bloat

WORKDIR /mango

# Add the files needed to compile dependencies.
COPY --chown=rust Cargo.toml .
COPY --chown=rust Cargo.lock .
RUN sudo chown rust:rust -R . && \
    sudo chmod g+s -R . && \
    mkdir -p cli/src daemon/src common/src && \
    printf 'fn main() {\n\tprintln!("placeholder for compiling stable dependencies")\n}' | tee cli/src/main.rs | tee daemon/src/main.rs | tee common/src/lib.rs
COPY --chown=rust cli/Cargo.toml ./cli/Cargo.toml
COPY --chown=rust daemon/Cargo.toml ./daemon/Cargo.toml
COPY --chown=rust common/Cargo.toml ./common/Cargo.toml

# Build the code (development mode).
RUN cargo build --tests

# Build the code (release mode).
# Note: sharing dependencies between dev/release does not work yet - https://stackoverflow.com/q/59511731
RUN cargo build --tests --release
#TODO: use --out-dir if it stabilizes

# Remove Cargo.toml files, to prevent other images from forgetting to re-add it.
RUN rm -f cargo_for_coverage.sh Cargo.toml cli/Cargo.toml daemon/Cargo.toml common/Cargo.toml

## NOTE!
## Make sure to `touch cli/src/main.rs`, `touch daemon/src/main.rs` and `touch common/src/lib.rs` after copying source, so that everything is recompiled

