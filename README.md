# rust-restapi

> 一个 Rust 语言实现的 Restful API 脚手架项目，基于 Actix 和 Diesel。

## 特性

- Actix 2.x HTTP Server
- Multi-Database Support (CockroachDB, Postgres, MySQL, Sqlite)
- JWT Support
- Websocket Support
- Multi-Env Config Support
- Custom ErrorHandler
- Async Caching Layer with a Simple API
- Public and Secure Static File Service
- Diesel Database Operations are Non-Blocking
- Filesystem Organized for Scale
- .env for Local Development
- Integrated Application State with a Simple API
- Lazy Static Config struct
- Built-in Healthcheck (includes cargo version info)
- Listeners configured for TDD
- Custom Errors and HTTP Payload/Json Validation
- Secure Argon2i Password Hashing
- CORS Support
- Unit and Integration Tests
- Test Coverage Reports
- Dockerfile for Running the Server in a Container
- TravisCI Integration

> 在原来的脚手架项目上剔除了 redis，并新增了配置文件和 websocket 模块，支持多环
> 境下的配置。日志库改为了 log4rs，直接记录到文件中。

## 使用

1. 运行本地的数据库环境，默认 MySQL，也推荐 MySQL。
2. 在 `config/default.json` 文件中更改数据库配置。
3. 打开终端，直接运行`cargo run`。

具体详细使用可以参考[原文档](./FEATURES.md)。

## 其它

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

- 从 https://github.com/ddimaria/rust-actix-example.git fork 和改编优化。
