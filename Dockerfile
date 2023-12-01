FROM ubuntu:latest

RUN apt update &&\
    DEBIAN_FRONTEND=noninteractive apt install -y \
        curl \
        build-essential

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -vy &&\
    . ~/.bashrc
