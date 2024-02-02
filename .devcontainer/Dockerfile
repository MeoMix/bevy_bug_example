FROM rust:1.74-bookworm

RUN apt-get update && \
   apt-get install -y libasound2-dev libudev-dev clang lld mold xorg

# Need to manually ensure nightly is configured when using bookworm
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup component add rustfmt