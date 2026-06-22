# Quick Start

## Start the REPL

```bash
cargo run --bin sqlrustgo
```

## Basic Operations

```sql
-- Create a table
CREATE TABLE users (id INT, name TEXT, age INT);

-- Insert data
INSERT INTO users VALUES (1, 'Alice', 25);
INSERT INTO users VALUES (2, 'Bob', 30);

-- Query data
SELECT * FROM users;
SELECT name, age FROM users WHERE age > 25;

-- Update data
UPDATE users SET age = 26 WHERE id = 1;

-- Delete data
DELETE FROM users WHERE id = 2;

-- Drop table
DROP TABLE users;
```

## Transactions

```sql
BEGIN;
INSERT INTO users VALUES (3, 'Charlie', 28);
COMMIT;

BEGIN;
INSERT INTO users VALUES (4, 'Diana', 22);
ROLLBACK;
```
