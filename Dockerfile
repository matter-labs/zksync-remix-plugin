FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
ENV DEBCONF_NONINTERACTIVE_SEEN=true
ENV HARDHAT_ENV_DOCKER_IMAGE="hardhat_env"
ENV METRICS_PORT=8001

RUN apt-get update && \
    apt-get install -y curl \
    git \
    cmake \
    build-essential \
    gcc \
    apt-transport-https \
    software-properties-common \
    wget \
    npm \
    && \
    rm -rf /var/lib/apt/lists/*
RUN wget https://github.com/ethereum/solidity/releases/download/v0.8.24/solc-static-linux -O /usr/local/bin/solc
RUN chmod +x /usr/local/bin/solc && solc --version

RUN npm install -g yarn && npm install -g n
RUN n lts

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /opt/app
COPY . /opt/app

WORKDIR /opt/app/api/hardhat_env
RUN yarn install

WORKDIR /opt/app/api
RUN cargo build --release
RUN chmod +x ./docker_run.sh

EXPOSE 8000

ENTRYPOINT [ "./docker_run.sh" ]
