# hrs

## 业务架构图
<img src="https://raw.githubusercontent.com/zhangyemengren/hrs_server/main/assets/business_arch.png" alt="业务架构图">

## 项目结构
```
.
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

### script文件
采用cargo script功能 尚未稳定 需要 rustc 1.76.0-nightly 以上
```run
cd script
cargo +nightly -Zscript name.rs
```
example: 执行print_test.rs
```
cargo +nightly -Zscript print_test.rs --config /path
```

### .env 为sqlx编译时验证所必须
