
# Nightly version of `base.Dockerfile`. Only debug mode.

# Nightly is needed for grcov and miri.
FROM ekidd/rust-musl-builder:nightly-2020-07-12

ENV RUST_BACKTRACE=1

WORKDIR /mango

# Add the files needed to compile dependencies.
COPY --chown=rust Cargo.toml .
COPY --chown=rust Cargo.lock .
RUN sudo chown rust:rust -R . && \
    sudo chmod g+s -R . && \
    mkdir -p src && \
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

