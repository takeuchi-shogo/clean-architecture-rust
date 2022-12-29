
FROM mysql:8.0.31

# 起動時に実行してくれるディレクトリ
COPY ./mysql/sql/01_init_db.sql /docker-entrypoint-initdb.d/01_init_db.sql

ENV MYSQL_ROOT_PASSWORD = pass
