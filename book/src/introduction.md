# SQLRustGo 简介

SQLRustGo 是一个使用 Rust 从零实现的 SQL-92 子集兼容的关系型数据库系统。

## 核心特性

- **SQL-92 支持**: SELECT, INSERT, UPDATE, DELETE, CREATE TABLE, DROP TABLE
- **存储引擎**: 页式存储（4KB页）+ BufferPool 缓存 + B+ Tree 索引
- **事务支持**: ACID 事务，通过 Write-Ahead Log (WAL) 实现
- **网络协议**: MySQL 风格协议支持 TCP 连接
- **交互式 REPL**: 支持交互式 SQL 命令行

## 项目状态

- 版本: v1.0.0
- 测试覆盖率: 92.5%
- 测试数量: 73+ 项
- Rust Edition: 2024
