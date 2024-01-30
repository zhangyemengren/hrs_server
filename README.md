- [业务架构图](#业务架构图)
- [运行命令](#运行命令)
- [项目结构](#项目结构)
- [环境变量](#环境变量)
- [技术选型](#技术选型)

# 业务架构图
<img src="https://raw.githubusercontent.com/zhangyemengren/hrs_server/main/assets/business_arch.png" alt="业务架构图">

# 运行命令

## 启动DB
- docker pull postgres
- docker run -d  
  --name hrs  
  -e POSTGRES_USER=postgres  
  -e POSTGRES_PASSWORD=qwer1234  
  -e POSTGRES_DB=hrs
  -v /YOUR_LOCAL_PATH:/var/lib/postgresql/data  
  -h localhost  
  -p 5432:5432  
  postgres  

## DB迁移
- cargo install sqlx-cli --no-default-features --features postgres
- sqlx database create
- sqlx migrate run

## 本地运行服务
- ENV_TYPE=prod cargo run --release
## 服务器运行服务
- ENV_TYPE=prod cargo build --release 
- sudo systemctl daemon-reload
- sudo systemctl enable hrs_app
- sudo systemctl start hrs_app

## 执行测试
- cargo test

## 执行脚本
采用cargo script功能 尚未稳定 需要 rustc 1.76.0-nightly 以上
```run
cd script
cargo +nightly -Zscript name.rs
```
example: 执行print_test.rs
```run
cargo +nightly -Zscript print_test.rs --config /path
```

## 生成文档
- cargo doc --no-deps --open

# 项目结构
```text
.
├── .github # github action
├── .env # sqlx编译时验证必须
├── Cargo.lock
├── Cargo.toml
├── hrs_app.service # systemd配置文件
├── LICENSE
├── README.md
├── assets # 资源文件
├── config # 配置文件
├── crates # 库封装
├── locales # 国际化文案
├── migrations # 数据库迁移文件
├── script # 脚本文件
├── src # 主体代码
├── target # 编译目标文件
└── tests # 测试文件
```

# 环境变量
- .env 为sqlx编译时验证所必须
- config/* 存放固定配置文件
- 运行时写入/高优先级/用于CI中 例如:  APP_APPLICATION_PORT=9001 cargo run 格式：前缀_路径=值

# 技术选型
| 名称                | 描述       |
|-------------------|----------|
| [`axum`]          | web框架    |
| [`tokio`]         | 异步运行时    |
| [`sqlx`]          | 数据库操作    |
| [`tracing`]       | 日志       |
| [`serde`]         | 序列化/反序列化 |
| [`anyhow`]        | 错误处理     |
| [`docker`]        | 容器       |
| [`postgresql`]    | 储存数据库    |
| [`github action`] | CI       |

[`axum`]: http://crates.io/crates/axum
[`tokio`]: http://crates.io/crates/tokio
[`sqlx`]: http://crates.io/crates/sqlx
[`tracing`]: http://crates.io/crates/tracing
[`serde`]: http://crates.io/crates/serde
[`anyhow`]: http://crates.io/crates/anyhow
[`docker`]: https://www.docker.com/
[`postgresql`]: https://www.postgresql.org/
[`github action`]: https://docs.github.com/en/actions


