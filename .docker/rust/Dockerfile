FROM rustlang/rust:nightly as builder

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development

WORKDIR /clean-architecture-rust

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
# RUN apt-get update && apt-get install -y \
# 	# curl libmysqlclient-dev \
# 	libmariadb3 \
# 	build-essential

# RUN cargo install diesel_cli --no-default-features --features mysql

COPY app/Cargo.toml /clean-architecture-rust/Cargo.toml
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY app/src ./src

# COPY ./app var/www/html
RUN rm -f target/release/deps/clean-architecture-rust*
RUN cargo build --release

# RUN rustup default nightly

# WORKDIR /var/www/html/

# RUN cargo build

FROM debian:11.6

ENV ROCKET_ENV=production

EXPOSE 8000

RUN apt-get update && apt-get install -y default-libmysqlclient-dev

# builderイメージからアプリのコードのみをコピーして /clean-archtecture-rustに配置する
COPY --from=builder /clean-architecture-rust/target/release/clean-architecture-rust /usr/local/bin/clean-architecture-rust

ENV RUST_BACKTRACE=full
# RUN apk add --no-cache ca-certificates
# CMD ["cargo", "run"]
CMD ["clean-architecture-rust"]
# カレントディレクトリで「docker build -t clean-archtecture-rust:1 -f .docker/rust/Dockerfile .」とすると
# ./docker/rust/Dockerfileでのビルドがカレントディレクトリで実行され、上位ディレクトリ（/app）が参照できるようになる
