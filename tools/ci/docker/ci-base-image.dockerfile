FROM --platform=linux/amd64 ubuntu:20.04
# FROM ubuntu:20.04
ENV DEBIAN_FRONTEND=noninteractive
LABEL maintainer="Frequency"
LABEL description="Frequency CI base image"

WORKDIR /ci
RUN apt-get update && \
	apt-get install -y curl protobuf-compiler build-essential libclang-dev git file jq clang cmake && \
	curl -fsSL https://get.docker.com -o get-docker.sh && sh get-docker.sh && \
	rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/home/runner/.cargo/bin:/root/.cargo/bin:${PATH}"
ENV RUSTUP_HOME="/root/.cargo"
ENV CARGO_HOME="/root/.cargo"

RUN git config --system --add safe.directory /__w/frequency/frequency
