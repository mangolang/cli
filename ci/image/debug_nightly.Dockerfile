
# Nightly version of `debug.Dockerfile`.

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
COPY --chown=rust ci/image/cargo_for_coverage.sh cargo_for_coverage.sh
COPY --chown=rust ci/image/run_tests_with_miri.sh run_tests_with_miri.sh
COPY --chown=rust deny.toml deny.toml
