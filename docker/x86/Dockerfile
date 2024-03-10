FROM debian:stable-slim

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt update && \
    apt install wget curl jq -y && \
    wget https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb -O packages-microsoft-prod.deb && \
    dpkg -i packages-microsoft-prod.deb && \
    rm packages-microsoft-prod.deb && \
    apt update && \
    apt install -y dotnet-sdk-6.0 && \
    apt clean && rm -rf /var/lib/apt/lists/* && \
    useradd runner && \
    echo "runner:runner" | chpasswd && \
    chsh -s /usr/bin/bash runner && \
    usermod -aG sudo runner && \
    mkdir /actions-runner && \
    chown runner:runner /actions-runner

USER runner
WORKDIR /actions-runner

RUN curl -fsSL -o actions-runner.tar.gz https://github.com/actions/runner/releases/download/v2.313.0/actions-runner-linux-x64-2.313.0.tar.gz && \
    tar xf actions-runner.tar.gz && \
    rm actions-runner.tar.gz

ENTRYPOINT ["/bin/sh", "-c", "./run.sh --jitconfig ${JIT_CONFIG}"]
