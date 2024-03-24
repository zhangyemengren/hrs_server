- [业务架构图](#业务架构图)
- [运行命令](#运行命令)
- [服务器部署](#服务器部署)
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
release模式 + prod环境
- ENV_TYPE=prod cargo run --release
dev模式 + dev环境
- cargo run

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

## 格式化 用到了unstable功能 imports_granularity = "Crate"
```run
cargo +nightly fmt
```

## 生成文档
- cargo doc --no-deps --open

# 服务器部署
## 目前没有自动化部署流程 手动执行以下步骤
服务器中执行(ubuntu 22 LTS) root用户 hrs项目路径执行
- 拉取代码
```run
git clone https://github.com/zhangyemengren/hrs_web.git
```

- 安装docker
```run
# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update

sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

- 验证 docker
```run
sudo docker run hello-world
```

- 安装rust
```run
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- 安装rust nightly
```run
rustup install nightly
```

- 执行初始化脚本
```run
cargo +nightly -Zscript ./script/init.rs
```

- 编译项目(线上环境变量在systemd配置文件中)
```run
cargo build --release
```

- systemd 可以按需修改hrs_app.service配置
```run
cp hrs_app.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable hrs_app
sudo systemctl start hrs_app
```

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


