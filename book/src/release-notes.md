# Release Notes

## v1.0.0 (2026-06-22)

### Features
- SQL-92 subset: SELECT, INSERT, UPDATE, DELETE, CREATE TABLE, DROP TABLE
- Storage engine: Page-based with BufferPool and LRU eviction
- B+ Tree index support
- Transaction support via Write-Ahead Log (WAL)
- TCP network server with MySQL-style protocol
- Interactive REPL

### Quality
- 73+ tests passing
- 92.5% test coverage
- Clippy: 0 warnings
- Full CI/CD pipeline configured

### Security
- cargo-audit integration in CI
- Dependabot for automatic dependency updates
- cargo-deny license compliance checking
