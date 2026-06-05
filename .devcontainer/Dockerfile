FROM mcr.microsoft.com/devcontainers/rust:latest

USER root

RUN mkdir -p /lesson/target /usr/local/cargo/registry \
    && chown -R vscode:vscode /lesson /usr/local/cargo \
    && rustup component add clippy rust-src

USER vscode

WORKDIR /lesson
