# API 文档

## 生成 API 文档

```bash
cargo doc --no-deps --open
```

## 核心模块

### sqlrustgo 库入口

```rust
// src/lib.rs
pub mod lexer;      // 词法分析
pub mod parser;     // 语法分析
pub mod executor;   // 查询执行
pub mod storage;    // 存储引擎
pub mod transaction; // 事务管理
pub mod types;      // 类型系统

pub fn init();      // 初始化数据库
pub fn pr_demo() -> &'static str;  // PR演示函数
```

### 关键类型

| 类型 | 所在模块 | 说明 |
|------|----------|------|
| `Token` | `lexer` | SQL Token 枚举 |
| `Statement` | `parser` | SQL 语句 AST |
| `Value` | `types` | 数据类型枚举 |
| `Page` | `storage` | 4KB 数据页 |
| `BufferPool` | `storage` | LRU 缓存管理器 |
| `WriteAheadLog` | `transaction` | WAL 事务日志 |

完整的 API 文档请使用 `cargo doc --open` 在浏览器中查看。
