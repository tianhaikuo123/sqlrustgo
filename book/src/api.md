# API Reference

Generate full API documentation with:

```bash
cargo doc --no-deps --open
```

## Core Types

### Value

```rust
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
    Blob(Vec<u8>),
}
```

### Statement

```rust
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    CreateTable(CreateTableStatement),
    DropTable(DropTableStatement),
    Begin,
    Commit,
    Rollback,
}
```

## Public API

```rust
use sqlrustgo::*;

// Initialize
init();

// Tokenize SQL
let tokens = tokenize("SELECT * FROM users");

// Parse SQL
let stmt = parse("SELECT * FROM users").unwrap();

// Execute
let mut engine = ExecutionEngine::new();
let result = engine.execute(stmt);
```

## Storage API

```rust
use sqlrustgo::storage::{Page, BufferPool, BPlusTree};

let page = Page::new(1);
let mut pool = BufferPool::new(10);
pool.add_page(page).unwrap();

let mut tree = BPlusTree::new();
tree.insert(10, 100);
```
