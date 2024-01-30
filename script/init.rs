#!/usr/bin/env cargo

use std::process::Command;

// 初始化脚本 需要安装 Docker Rust(stable和nightly)
fn main() {
    // 拉取 PostgreSQL Docker 镜像
    Command::new("docker")
        .args(&["pull", "postgres"])
        .status()
        .expect("Failed to pull PostgreSQL Docker image");

    // 运行 PostgreSQL Docker 容器
    Command::new("docker")
        .args(&[
            "run", "-d",
            "--name", "hrs",
            "-e", "POSTGRES_USER=postgres",
            "-e", "POSTGRES_PASSWORD=qwer1234",
            "-e", "POSTGRES_DB=hrs",
            "-v", "/YOUR_LOCAL_PATH:/var/lib/postgresql/data",
            "-h", "localhost",
            "-p", "5432:5432",
            "postgres",
        ])
        .status()
        .expect("Failed to start PostgreSQL Docker container");
    // 尝试运行 sqlx-cli，检查是否已安装
    let sqlx_check = Command::new("sqlx")
        .arg("--version")
        .output();

    match sqlx_check {
        Ok(output) => {
            if output.status.success() {
                println!("sqlx-cli is already installed.");
            } else {
                // 如果 sqlx-cli 没有安装，安装它
                install_sqlx_cli();
            }
        }
        Err(_) => {
            install_sqlx_cli();
        }
    }

    // 执行数据库创建和迁移
    Command::new("sqlx")
        .arg("database")
        .arg("create")
        .status()
        .expect("Failed to create database");

    Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .status()
        .expect("Failed to run migrations");
}

fn install_sqlx_cli() {
    println!("Installing sqlx-cli...");
    let install_sqlx = Command::new("cargo")
        .args(&["install", "sqlx-cli", "--no-default-features", "--features", "postgres"])
        .status()
        .expect("Failed to install sqlx-cli");

    if !install_sqlx.success() {
        panic!("Failed to install sqlx-cli");
    }
}