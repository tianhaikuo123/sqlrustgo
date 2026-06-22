# Architecture

SQLRustGo follows a layered architecture:

```
┌─────────────────────────────┐
│         REPL / Network       │  ← User Interface
├─────────────────────────────┤
│         Parser / Lexer       │  ← SQL → AST
├─────────────────────────────┤
│         Planner              │  ← Logical → Physical Plan
├─────────────────────────────┤
│         Executor             │  ← Query Execution
├─────────────────────────────┤
│         Storage Engine       │  ← Page, BufferPool, B+Tree
├─────────────────────────────┤
│         Transaction (WAL)    │  ← ACID, WAL
└─────────────────────────────┘
```

## Modules

| Module | Path | Description |
|--------|------|-------------|
| `lexer` | `src/lexer/` | Tokenizes SQL strings |
| `parser` | `src/parser/` | Parses tokens into AST |
| `planner` | `src/planner/` | Query planning and optimization |
| `executor` | `src/executor/` | Executes query plans |
| `storage` | `src/storage/` | Page, BufferPool, B+Tree, FileStorage |
| `transaction` | `src/transaction/` | WAL and transaction manager |
| `network` | `src/network/` | TCP server/client |
| `types` | `src/types/` | Value types and errors |

## Storage Engine

- **Page**: 4KB fixed-size data pages
- **BufferPool**: In-memory LRU cache for pages
- **DiskManager**: File-based persistent storage
- **B+Tree**: Index structure for efficient lookups
