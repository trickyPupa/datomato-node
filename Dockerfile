FROM rust:1.93.0@sha256:bbde3ca426faeebab3eb58b8005d3d6da70607817a14bb92a55c94611d4d905f
RUN rustup component add --toolchain 1.93.0 rustfmt clippy \
    && rustc --version | grep '^rustc 1.93.0 ' \
    && command -v git && command -v cc
ENV CARGO_BUILD_JOBS=2
WORKDIR /work
