
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
RUN mkdir -p src common/src daemon/src cli/src && \
    printf '\nfn main() {\n\tprintln!("placeholder for compiling dependencies")\n}\n' | tee common/src/lib.rs | tee daemon/src/main.rs | tee cli/src/main.rs
COPY common/Cargo.toml ./common/
COPY daemon/Cargo.toml ./daemon/
COPY cli/Cargo.toml ./cli/

# Build the dependencies, remove Cargo files so they have to be re-added.
RUN cargo build --workspace --tests &&\
    cargo build --workspace --release &&\
    rm -rf Cargo.toml Cargo.lock common/ daemon/ cli/

# Copy the actual code.
COPY ./Cargo.toml ./Cargo.lock ./deny.toml ./rustfmt.toml ./
COPY ./common/ ./common
COPY ./daemon/ ./daemon
COPY ./cli/ ./cli

# Build (for test)
RUN find . -name target -prune -o -type f &&\
    touch -c common/src/lib.rs daemon/src/main.rs cli/src/main.rs &&\
    cargo --offline build --workspace --tests

# Test
RUN cargo --offline test --workspace --all-targets --all-features

# Lint
RUN cargo --offline clippy --workspace --all-targets --all-features --tests -- -D warnings

# Style
RUN cargo --offline fmt --all -- --check

# Dependencies
RUN cargo --offline tree --workspace --all-features > dep.tree
#TODO @mark: re-enable dependency checks
#RUN cat dep.tree && cargo --offline audit --deny warnings
#RUN cat dep.tree && cargo --offline deny check advisories
RUN cat dep.tree && cargo --offline deny check licenses
#RUN cat dep.tree && cargo --offline deny check bans
RUN cat dep.tree && cargo --offline outdated --exit-code 1

# Build release
RUN cargo --offline build --workspace --release

# A find is needed here for it to work with multiple platforms (musl uses different path)
RUN find . -wholename '*/release/*' -name 'mango' -type f -executable -print -exec cp {} /mango_exe \; &&\
    find . -wholename '*/release/*' -name 'mangod' -type f -executable -print -exec cp {} /mangod_exe \;

# Second stage image to decrease size
FROM scratch AS executable

ENV PATH=/
ENV RUST_BACKTRACE=1
ENV RUST_LOG='debug,ws=warn,mio=warn'
ENV MANGO_USER_CACHE_PATH='/cache'
ENV MANGO_USER_CONFIG_PATH='/conf'
WORKDIR /code

# It's really just the executable; other files are part of the Github release, but not Docker image.
#COPY README.rst LICENSE.txt ./
COPY --from=build /mango_exe /mango
COPY --from=build /mangod_exe /mangod

# Smoke test
RUN ["mango", "--help"]
RUN ["mango", "daemon", "start"]
RUN ["mangod", "--help"]

ENTRYPOINT ["mango"]

