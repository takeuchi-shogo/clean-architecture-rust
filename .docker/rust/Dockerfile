FROM rust:1.66 as builder

ENV ROCKET_ADDRESS=localhost
ENV ROCKET_PORT=8000

WORKDIR /var/www/html

COPY ./app/Cargo.toml /var/www/html/
RUN mkdir src

RUN echo "fn main() {}" > src/main.rs

RUN rustup default nightly

RUN cargo build --release

COPY ./app/src ./src

RUN rm -f target/release/deps/var/www/html*

RUN cargo build --release

FROM debian:10.4

COPY --from=builder /var/www/html/target/release/ /usr/local/bin/clean-archtecture-rust

# CMD ["cargo", "run"]
CMD [ "clean-archtecture-rust" ]
# カレントディレクトリで「docker build -t clean-archtecture-rust:1 -f .docker/rust/Dockerfile .」とすると
# ./docker/rust/Dockerfileでのビルドがカレントディレクトリで実行され、上位ディレクトリ（/app）が参照できるようになる
