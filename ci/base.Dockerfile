
# Docker image that contains the pre-compiled dependencies of Mango.
# Only intended for use in the Mango project itself, to make other images build faster.
# https://hub.docker.com/r/mangocode/mango_daily_base

# Note: this version should match `executable.Dockerfile`
FROM rust:1.44

ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-deny
RUN cargo install cargo-tree
RUN cargo install cargo-udeps
RUN cargo install grcov

WORKDIR /mango

# Add the files needed to compile dependencies.
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && \
    printf 'fn main() { println!("placeholder for compiling dependencies") }' > src/main.rs

# Build the code (development mode)
RUN cargo build --bin mango

# Build the code (release mode)
# Note: sharing dependencies between dev/release does not work yet - https://stackoverflow.com/q/59511731
RUN cargo build --bin mango --release
#TODO: use --out-dir if it ever stabilizes

# Build the code with special flags for code coverage
COPY ci/cargo_for_coverage.sh cargo_for_coverage.sh
RUN ./build_for_coverage.sh && rm -f cargo_for_coverage.sh

# Remove Cargo.toml file, to prevent other images from forgetting to re-add it.
RUN rm -f Cargo.toml

# NOTE!
# Make sure to `touch src/main.rs` after copying source, so that everything is recompiled

