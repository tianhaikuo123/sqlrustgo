# 安装指南

## 环境要求

| 项目 | 最低要求 |
|------|----------|
| Rust | 1.75+ |
| 内存 | 4GB+ |
| 磁盘 | 1GB+ |

## 安装步骤

### 1. 克隆项目

```bash
git clone https://github.com/tianhaikuo123/sqlrustgo.git
cd sqlrustgo
```

### 2. 编译

```bash
cargo build --all-features
```

### 3. 运行测试

```bash
cargo test --all-features
```

### 4. 启动 REPL

```bash
cargo run --bin sqlrustgo
```

## 验证安装

启动后应看到：
```
SQLRustGo v1.0.0
Type "help" for more information.
sql>
```
