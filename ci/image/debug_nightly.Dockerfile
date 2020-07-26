
# Nightly version of `base.Dockerfile`.
# * Only debug mode.
# * Also useful for non-musl checks.

FROM mangocode/mango_nightly_base:latest

# Now add the actual code
COPY --chown=rust rustfmt.toml Cargo.toml Cargo.lock ./
COPY --chown=rust src src
RUN ls -als  #TODO @mark: TEMPORARY! REMOVE THIS!

# This makes sure things are rebuilt
RUN touch src/main.rs

# Build the code (debug mode)
RUN cargo build --bin mango

# Build the code (release mode)
RUN cargo build --bin mango --release

# Miscellaneous other files
COPY --chown=rust ci/image/cargo_for_coverage.sh deny.toml ./
