# hrs

## 业务架构图
<img src="https://raw.githubusercontent.com/zhangyemengren/hrs_server/main/assets/business_arch.png" alt="业务架构图">

## 运行步骤

### 启动DB
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

### DB迁移
- cargo install sqlx-cli --no-default-features --features postgres
- sqlx database create
- sqlx migrate run

### 运行服务
- cargo run

### 执行测试
- cargo test

### 执行脚本
采用cargo script功能 尚未稳定 需要 rustc 1.76.0-nightly 以上
```run
cd script
cargo +nightly -Zscript name.rs
```
example: 执行print_test.rs
```
cargo +nightly -Zscript print_test.rs --config /path
```

## 项目结构
```
.
├── .github # github action
├── .env # sqlx编译时验证必须
├── Cargo.lock
├── Cargo.toml
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

### 环境变量
- .env 为sqlx编译时验证所必须
- config/* 存放固定配置文件
- 运行时写入/高优先级/用于CI中 例如:  APP_APPLICATION_PORT=9001 cargo run 格式：前缀_路径=值
