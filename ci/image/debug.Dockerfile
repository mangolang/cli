
# This image builds the Mango CLI with all dependencies.
# It is used as an intermediary image during build pipeline; subsequent images will use it to do further checks.
# For the releasable version, see `executable.Dockerfile`

FROM mangocode/mango_daily_base:latest

# Now add the actual code
COPY rustfmt.toml Cargo.toml Cargo.lock ./
COPY src src

# This makes sure things are rebuilt
RUN touch src/main.rs

# Build the code (debug mode)
RUN cargo build --bin mango

# Build the code (release mode)
RUN cargo build --bin mango --release

# Build the code with special flags for code coverage.
COPY ci/image/cargo_for_coverage.sh cargo_for_coverage.sh
RUN ./cargo_for_coverage.sh build

# Miscellaneous other files
COPY ci/image/run_tests_with_miri.sh run_tests_with_miri.sh
COPY deny.toml deny.toml
