FROM rust:1.64.0 as builder

ARG VECTOR_BUILD_DESC

WORKDIR /source

COPY . .

ENV CARGO_HTTP_DEBUG=true
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_TERM_VERBOSE=true
ENV CARGO_TERM_PROGRESS_WHEN=always
ENV CARGO_TERM_PROGRESS_WIDTH=80

ENV GIT_CURL_VERBOSE=1
ENV GIT_TRACE=1

ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

ENV VECTOR_BUILD_DESC=${VECTOR_BUILD_DESC}

RUN bash /source/build/bootstrap-ubuntu.sh && bash cargo build --release

FROM debian:bullseye-slim as runtime

RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list && apt-get update && apt-get install -y ca-certificates tzdata systemd && rm -rf /var/lib/apt/lists/*

WORKDIR /source

VOLUME /var/lib/vector

COPY /config /etc/vector
COPY --from=builder /source/target/release/vector /usr/bin

ENTRYPOINT ["/usr/bin/vector"]
CMD ["--config-dir", "/etc/vector/"]
