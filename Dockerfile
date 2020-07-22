
# Docker image for the Mango langauge, with the command-line interface as entrypoint.
#TODO @mark: this image is intended for distribution, but is not ready yet

FROM rust:1.44

ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy

WORKDIR /mango

# Build the dependencies (for image caching)
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && \
    printf 'fn main() { println!("placeholder for compiling dependencies") }' > src/main.rs
RUN cargo build --release

# Now add the actual code
COPY src src
# This makes sure things are rebuilt
RUN touch src/main.rs

# Test the code, including style checks
RUN cargo --offline fmt --all
RUN cargo --offline clippy --release --all-targets --all-features -- -D warnings
RUN cargo --offline test --release --all-targets --all-features --all

# Build the code
RUN cargo --offline build --bin mango --release

# Second stage image to decrease size
FROM rust:1.44-slim

ENV RUST_BACKTRACE=1

WORKDIR /code

COPY README.rst LICENSE.txt ./
COPY --from=0 /mango/target/release/mango /usr/bin/mango

CMD printf "Welcome to the Mango docker image!\nTo use, add 'mango' after your docker run command\n"
