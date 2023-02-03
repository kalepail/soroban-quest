FROM gitpod/workspace-full:2022-12-30-17-11-09
LABEL version="1.1.15"

RUN mkdir -p ~/.local/bin
RUN curl -L -o ~/.local/bin/soroban https://github.com/stellar/soroban-tools/releases/download/v0.4.0/soroban-cli-0.4.0-x86_64-unknown-linux-gnu
RUN chmod +x ~/.local/bin/soroban
RUN curl -L https://github.com/mozilla/sccache/releases/download/v0.3.3/sccache-v0.3.3-x86_64-unknown-linux-musl.tar.gz | tar xz --strip-components 1 -C ~/.local/bin sccache-v0.3.3-x86_64-unknown-linux-musl/sccache
RUN chmod +x ~/.local/bin/sccache

RUN curl -LO https://github.com/denoland/deno/releases/download/v1.30.1/deno-x86_64-unknown-linux-gnu.zip
RUN unzip deno-x86_64-unknown-linux-gnu.zip -d ~/.local/bin

RUN git clone https://github.com/tyvdh/soroban-quest--pioneer.git ~/.local/_tmp/soroban-quest && \
    mv ~/.local/_tmp/soroban-quest/_client ~/.local && \
    cd ~/.local/_tmp/soroban-quest/_squirtle && \
    mv bash-hook ~/.local && \
    npm run package && \
    cd ~/.local && \
    rm -rf ~/.local/_tmp

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=5G
ENV SCCACHE_DIR=/workspace/.sccache

RUN rustup install stable
RUN rustup target add --toolchain stable wasm32-unknown-unknown
RUN rustup component add --toolchain stable rust-src
RUN rustup install nightly
RUN rustup target add --toolchain nightly wasm32-unknown-unknown
RUN rustup component add --toolchain nightly rust-src
RUN rustup default stable

RUN sudo apt-get update && sudo apt-get install -y binaryen
