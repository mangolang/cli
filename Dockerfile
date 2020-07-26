
#TODO @mark: TEMPORARY! REMOVE THIS!
#TODO @mark: TEMPORARY! REMOVE THIS!
#TODO @mark: TEMPORARY! REMOVE THIS!

# This image builds the Mango CLI in a slim image.
# This is the image to interact with as a user of Mango.
# https://hub.docker.com/r/mangocode/mango

FROM ekidd/rust-musl-builder:1.45.0 AS build

ENV RUST_BACKTRACE=1

COPY --chown=rust rustfmt.toml Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN RUSTFLAGS='-Copt-level=0' cargo build --bin mango --release

COPY --chown=rust src src

# Probably still up-to-date, just just in case.
RUN RUSTFLAGS='-Copt-level=0' cargo build --bin mango --release

# A find is needed here for it to work with multiple platforms (musl uses different path)
RUN find . -wholename '*/release/*' -name 'mango' -type f -executable -print -exec cp {} ./mango \;

RUN ls -als . && pwd

# Second stage image to decrease size
# Note: this version should match `base.Dockerfile`
FROM scratch

ENV RUST_BACKTRACE=1

WORKDIR /

# It's really just the executable; other files are part of the Github release, but not Docker image.
#COPY README.rst LICENSE.txt ./
COPY --from=build /home/rust/src/mango /mango

ENTRYPOINT ["/mango"]
CMD ["/mango"]
