# docker build -t tectonic_svg .
# docker run -it tectonic_svg
FROM ubuntu:latest

SHELL ["/bin/bash", "-c"]

RUN yes | unminimize
RUN apt-get install -y curl wget build-essential pkg-config vim
RUN apt-get install -y libfreetype6-dev libgraphite2-dev libicu-dev libpng-dev zlib1g-dev libfontconfig-dev
RUN apt-get install -y dvisvgm

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get install -y libssl-dev

WORKDIR /home
RUN cargo install tectonic

COPY src src
COPY Cargo.toml Cargo.toml
COPY test.tex test.tex

RUN tectonic -X compile test.tex && rm test.pdf

RUN cargo build
CMD ["/bin/bash"]
