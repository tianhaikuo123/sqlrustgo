# SQLRustGo — Introduction

**SQLRustGo** is a lightweight relational database system implemented in Rust. It supports a subset of the SQL-92 standard and serves as an educational project for understanding database internals.

## Key Features

| Feature | Description |
|---------|-------------|
| SQL-92 Support | SELECT, INSERT, UPDATE, DELETE, CREATE TABLE, DROP TABLE |
| Storage Engine | Page-based storage with BufferPool and LRU eviction |
| B+ Tree Index | Efficient key-value lookups |
| Transaction Support | Write-Ahead Log (WAL) for ACID compliance |
| Network Protocol | TCP server with MySQL-style protocol |
| Interactive REPL | Command-line SQL interface |

## Project Status

- **Version**: v1.0.0
- **Test Coverage**: 92.5%
- **Tests**: 73+ passing
- **Maturity**: L3 Product Ready

## Quick Links

- [GitHub Repository](https://github.com/tianhaikuo123/sqlrustgo)
- [Installation Guide](./installation.md)
- [SQL Reference](./sql-reference.md)
