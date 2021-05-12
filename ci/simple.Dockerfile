
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
    printf '\nfn main() {\n\tprintln!("something went wrong while building the docker image")\n}\n' | tee src/main.rs

# Build the dependencies, remove Cargo files so they have to be re-added.
RUN cargo build --tests &&\
    cargo build --release &&\
    rm -rf Cargo.toml Cargo.lock src/

# Copy the actual code.
COPY ./Cargo.toml ./Cargo.lock ./deny.toml ./rustfmt.toml ./
COPY ./src/ ./src

# Build (for test)
RUN find . -name target -prune -o -type f &&\
    touch -c src/main.rs &&\
    cargo --offline build --tests

#TODO @mark: move up^
ENV RUST_LOG='debug,ws=warn,mio=warn'

# Test
RUN cargo --offline test --all-targets --all-features

# Lint
RUN cargo --offline clippy --all-targets --all-features --tests -- -D warnings

# Style
RUN cargo --offline fmt --all -- --check

# Custom checks
COPY ./ci/extra_checks.sh ./extra_checks.sh
RUN sh extra_checks.sh && rm extra_checks.sh

# Dependencies
RUN cargo --offline tree --all-features > dep.tree
#TODO @mark: re-enable dependency checks
#RUN cat dep.tree && cargo --offline audit --deny warnings
#RUN cat dep.tree && cargo --offline deny check advisories
RUN cat dep.tree && cargo --offline deny check licenses
#RUN cat dep.tree && cargo --offline deny check bans
RUN cat dep.tree && cargo --offline outdated --exit-code 1

# Build release
RUN cargo --offline build --release

# A find is needed here for it to work with multiple platforms (musl uses different path)
RUN find . -wholename '*/release/*' -name 'mango' -type f -executable -print -exec cp {} /mango_exe \;

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

# Smoke test
RUN ["mango", "--help"]
RUN ["mango", "daemon", "start"]
RUN ["mango", "run-as-daemon", "--help"]

ENTRYPOINT ["mango"]

