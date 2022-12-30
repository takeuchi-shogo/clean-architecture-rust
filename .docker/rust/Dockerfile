FROM rustlang/rust:nightly as builder

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development

WORKDIR /clean-archtecture-rust

COPY app/Cargo.toml /clean-archtecture-rust/Cargo.toml
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY app/src ./src

# COPY ./app var/www/html
RUN rm -f target/release/deps/clean-archtecture-rust*
RUN cargo build --release

# RUN rustup default nightly

# WORKDIR /var/www/html/

# RUN cargo build

FROM debian:10.4

ENV ROCKET_ENV=production

EXPOSE 8000

# builderイメージからアプリのコードのみをコピーして /clean-archtecture-rustに配置する
COPY --from=builder /clean-archtecture-rust/target/release/clean-archtecture-rust /usr/local/bin/clean-archtecture-rust
# CMD ["cargo", "run"]
CMD ["clean-archtecture-rust"]
# カレントディレクトリで「docker build -t clean-archtecture-rust:1 -f .docker/rust/Dockerfile .」とすると
# ./docker/rust/Dockerfileでのビルドがカレントディレクトリで実行され、上位ディレクトリ（/app）が参照できるようになる
