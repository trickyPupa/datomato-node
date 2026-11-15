FROM rust:1.100.0@sha256:ebc54bef33ec8f9a5cd05799366010f35efbb73b354fb4f2cb086ec6b9460ef5
RUN rustup component add --toolchain 1.100.0 rustfmt clippy \
    && rustc --version | grep '^rustc 1.100.0 ' \
    && cargo --version | grep '^cargo 1.100.0 ' \
    && command -v git && command -v cc
ENV CARGO_BUILD_JOBS=2
WORKDIR /work
