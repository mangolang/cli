
FROM clux/muslrust:nightly-2021-04-24 AS build

ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-deny
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
COPY ./src ./src

# Build (for test)
RUN touch -c src/main.rs &&\
    cargo --offline build --workspace --tests

# Test
RUN cargo --offline test --workspace --all-targets --all-features

# Lint
RUN cargo --offline clippy --workspace --all-targets --all-features -- -D warnings

# Style
RUN cargo --offline fmt --all -- --check

# Dependencies
RUN cargo --offline tree --workspace --all-features > dep.tree
#TODO @mark: deny warnings after https://github.com/anderslanglands/ustr/issues/17
RUN cat dep.tree && cargo --offline audit # --deny warnings
RUN cat dep.tree && cargo --offline deny check advisories
RUN cat dep.tree && cargo --offline deny check licenses
#TODO @mark: enable after https://github.com/anderslanglands/ustr/issues/17
#RUN cat dep.tree && cargo --offline deny check bans
RUN cat dep.tree && cargo --offline outdated --exit-code 1

# Build release
RUN cargo --offline build --workspace --release
