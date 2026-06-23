# 架构设计

## 系统架构

```
┌─────────────────────────────────────┐
│           main.rs (REPL)             │
├─────────────────────────────────────┤
│           executor/                 │  ← 查询执行
├─────────────────────────────────────┤
│           parser/                    │  ← SQL → AST
│           lexer/                    │  ← SQL → Tokens
├─────────────────────────────────────┤
│           storage/                   │  ← Page, BufferPool, B+ Tree
├─────────────────────────────────────┤
│         transaction/                 │  ← WAL, TxManager
├─────────────────────────────────────┤
│           network/                   │  ← TCP Server/Client
├─────────────────────────────────────┤
│           types/                     │  ← Value, SqlError
└─────────────────────────────────────┘
```

## 模块说明

### Lexer（词法分析器）
将SQL文本转换为Token流。支持关键字、标识符、字面量、运算符的识别。

### Parser（语法分析器）
将Token流解析为AST（抽象语法树）。支持SELECT/INSERT/UPDATE/DELETE/CREATE/DROP语句。

### Executor（执行器）
执行AST并返回结果。包含TableScan、Filter、Projection等算子的实现。

### Storage（存储引擎）
- **Page**: 4KB固定大小数据页
- **BufferPool**: LRU缓存淘汰策略的缓冲区管理器
- **B+ Tree**: 索引结构
- **DiskManager**: 磁盘文件读写

### Transaction（事务管理）
- **WriteAheadLog (WAL)**: 先写日志保证事务持久性
- **TransactionManager**: 管理事务的BEGIN/COMMIT/ROLLBACK
