
# This image builds the Mango CLI in a slim image.
# This is the image to interact with as a user of Mango.
# https://hub.docker.com/r/mangocode/mango

FROM mango_ci

# Second stage image to decrease size
# Note: this version should match `base.Dockerfile`
FROM rust:1.44-slim

ENV RUST_BACKTRACE=1

WORKDIR /code

COPY README.rst LICENSE.txt ./
COPY --from=0 /mango/target/release/mango /usr/bin/mango

CMD printf "Welcome to the Mango docker image!\nTo use, add 'mango' after your docker run command\n"
