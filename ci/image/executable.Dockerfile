
# This image builds the Mango CLI in a slim image.
# This is the image to interact with as a user of Mango.
# https://hub.docker.com/r/mangocode/mango

FROM mango_ci AS build

# Probably still up-to-date, just just in case.
RUN cargo build --bin mango --release
#RUN echo "FIND!" && pwd && find / -type f -maxdepth 3 -name '*mango*' && echo "----" && find /mango/target/release -type f -maxdepth 1  #TODO @mark: TEMPORARY! REMOVE THIS!

# Second stage image to decrease size
# Note: this version should match `base.Dockerfile`
FROM scratch

ENV RUST_BACKTRACE=1

WORKDIR /

# It's really just the executable; other files are part of the Github release, but not Docker image.
#COPY README.rst LICENSE.txt ./
COPY --from=build /mango/target/release/mango /mango
RUN echo "FIND!" && find . && ls -als  #TODO @mark: TEMPORARY! REMOVE THIS!

CMD printf "Welcome to the Mango docker image!\nTo use, add 'mango' after your docker run command\n"
