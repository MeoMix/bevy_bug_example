FROM rust:1.76-bookworm

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
   apt-get install -y libasound2-dev libudev-dev libxkbcommon-x11-0 clang lld mold xorg

# Need to manually ensure nightly is configured when using bookworm
RUN rustup toolchain install nightly
RUN rustup component add rustfmt
RUN cargo install cargo-watch

# Ensure hardware acceleration works for native builds when running on Windows host machine
ENV LD_LIBRARY_PATH=/usr/lib/wsl/lib
RUN apt update && apt install -y vainfo mesa-va-drivers
ENV LIBVA_DRIVER_NAME=d3d12