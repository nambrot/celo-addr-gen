FROM ubuntu:16.04 as rustbuilder
RUN apt update && apt install -y curl musl-tools
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=$PATH:~/.cargo/bin
RUN $HOME/.cargo/bin/rustup install 1.41.0 && $HOME/.cargo/bin/rustup default 1.41.0 && $HOME/.cargo/bin/rustup target add x86_64-unknown-linux-musl
ADD . /tmp/celo-addr-gen
RUN cd /tmp/celo-addr-gen && $HOME/.cargo/bin/cargo build --target x86_64-unknown-linux-musl --release

CMD ["/tmp/addr-gen/target/x86_64-unknown-linux-musl/release/celo-addr-gen"]