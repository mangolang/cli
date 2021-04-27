
FROM clux/muslrust:nightly-2021-04-24 AS build

ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-tree

# Add the files needed to compile dependencies.
COPY ./Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
    printf '\nfn main() {\n\tprintln!("placeholder for compiling dependencies")\n}\n' | tee src/main.rs

# Build the dependencies, remove Cargo files so they have to be re-added.
RUN cargo build --workspace --tests &&\
    cargo build --workspace --release &&\
    rm -f Cargo.lock Cargo.toml src/main.rs

# Copy the actual code.
COPY ./Cargo.toml ./Cargo.lock ./deny.toml ./rustfmt.toml ./

# Build (for test)
RUN touch -c src/main.rs &&\
    cargo --offline build --workspace --tests
