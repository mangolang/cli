
# This image builds the Mango CLI in a slim image.
# This is the image to interact with as a user of Mango.
# https://hub.docker.com/r/mangocode/mango

FROM mango_ci

# Probably still up-to-date, just just in case.
RUN cargo build --bin mango --release

# Second stage image to decrease size
# Note: this version should match `base.Dockerfile`
FROM scratch

ENV RUST_BACKTRACE=1

WORKDIR /

# It's really just the executable; other files are part of the Github release, but not Docker image.
#COPY README.rst LICENSE.txt ./
COPY --from=0 /mango/target/release/mango /mango

RUN find /  #TODO @mark: TEMPORARY! REMOVE THIS!

CMD printf "Welcome to the Mango docker image!\nTo use, add 'mango' after your docker run command\n"
