
# This image builds the Mango CLI.
# It is used as an intermediary image during build pipeline; subsequent images will use it to do further checks.

FROM mangocode/mango_daily_base:latest

# Now add the actual code
COPY Cargo.toml Cargo.toml
COPY src src

# This makes sure things are rebuilt
RUN touch src/main.rs

# Build the code (debug mode)
RUN cargo build --bin mango

# Build the code (release mode)
RUN cargo build --bin mango --release

