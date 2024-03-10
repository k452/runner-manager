FROM debian:stable-slim

ENV DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt-get update && \
    apt-get install wget curl jq libc6 libicu-dev -y && \
    wget https://dot.net/v1/dotnet-install.sh -O dotnet-install.sh && \
    chmod +x ./dotnet-install.sh && \
    ./dotnet-install.sh --version latest && \
    rm ./dotnet-install.sh && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    useradd runner && \
    echo "runner:runner" | chpasswd && \
    chsh -s /usr/bin/bash runner && \
    usermod -aG sudo runner && \
    mkdir /actions-runner && \
    chown runner:runner /actions-runner

USER runner
WORKDIR /actions-runner

RUN curl -fsSL -o actions-runner.tar.gz https://github.com/actions/runner/releases/download/v2.314.1/actions-runner-linux-arm64-2.314.1.tar.gz && \
    tar xf actions-runner.tar.gz && \
    rm actions-runner.tar.gz

ENTRYPOINT ["/bin/sh", "-c", "./run.sh --jitconfig ${JIT_CONFIG}"]
