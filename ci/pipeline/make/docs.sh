#!/usr/bin/env bash

CHECK_NIGHTLY bash -c "\
cargo --offline doc --no-deps --all-features --release; \
cp -r target/doc /release/api-doc"
