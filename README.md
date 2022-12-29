
# Rust application

## Requirement
- container Docker
- languege Rust
- database Mysql
- web_server Nginx


## Overall structure

### Docker

_Dockerfile is written in each directory under ./docker._

<pre>
.
├── docker-compose.yml
├── mysql
│   └── Dockerfile
├── nginx
│   └── Dockerfile
└── rust
    └── Dockerfile
</pre>

### Application

<pre>
.
├── src <- application folder
├── target
├── .gitignore
├── Cargo.lock
└── Cargo.toml
</pre>

#### using languege and framwork

Languege __Rust__

Framwork
- __Rocket__
- __Diesel__

#### Clean archtecture

<pre>
./src
├── domain
│   ├── mod.rs
│   ├── user_profiles.rs
│   └── users.rs
├── infrastructure
│   ├── config.rs
│   ├── db.rs
│   ├── mod.rs
│   └── routing.rs
├── interface
│   ├── controllers
│   │   ├── mod.rs
│   │   └── product
│   │       ├── mod.rs
│   │       └── users_controller.rs
│   ├── gateways
│   │   ├── database
│   │   │   └── mod.rs
│   │   ├── gateways
│   │   │   └── mod.rs
│   │   └── mod.rs
│   └── mod.rs
├── lib.rs
├── main.rs
└── usecase
    ├── mod.rs
    ├── product
    │   ├── mod.rs
    │   └── user_interactor.rs
    └── repositories
        └── mod.rs
</pre>

__There is a possibility that it will increase in the future.__
Currently the above contents.

### Database

- Mysql

<pre>
.
├── config
│   └── my.conf
└── sql
    └── 01_init_db.sql
</pre>

### Web server

- Nginx

<pre>
.
└── default.conf
</pre>


## Description
Rust and Mysql and Nginx sample application.


## Author

[Twitter Account](https://twitter.com/shogo_mthr123)
