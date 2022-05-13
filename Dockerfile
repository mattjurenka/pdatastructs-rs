# Build Stage
FROM --platform=linux/amd64 ubuntu:22.04 as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update && apt-get install -y cmake clang llvm curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN . $HOME/.cargo/env
RUN ~/.cargo/bin/rustup default nightly
RUN ~/.cargo/bin/cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /pdatastructs/

WORKDIR /pdatastructs/fuzz/
RUN ~/.cargo/bin/cargo fuzz build

FROM --platform=linux/amd64 ubuntu:22.04

COPY --from=builder /pdatastructs/fuzz/target/x86_64-unknown-linux-gnu/release/cms_heap /
COPY --from=builder /pdatastructs/fuzz/target/x86_64-unknown-linux-gnu/release/count_min_sketch /
COPY --from=builder /pdatastructs/fuzz/target/x86_64-unknown-linux-gnu/release/hyper_log_log /
COPY --from=builder /pdatastructs/fuzz/target/x86_64-unknown-linux-gnu/release/lossy_counter /
COPY --from=builder /pdatastructs/fuzz/target/x86_64-unknown-linux-gnu/release/reservoir_sampling /
