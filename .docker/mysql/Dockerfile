
FROM mysql:8.0.31

EXPOSE 3306

COPY ./mysql/config/my.conf /etc/mysql/conf.d/my.cnf
# 起動時に実行してくれるディレクトリ
COPY ./mysql/sql/01_init_db.sql /docker-entrypoint-initdb.d/01_init_db.sql
