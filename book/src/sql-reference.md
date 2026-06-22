# SQL Reference

SQLRustGo supports a subset of the SQL-92 standard.

## DDL Statements

### CREATE TABLE

```sql
CREATE TABLE table_name (
    column1 TYPE,
    column2 TYPE,
    ...
);
```

Supported types: `INT`, `TEXT`, `FLOAT`, `BOOLEAN`

### DROP TABLE

```sql
DROP TABLE table_name;
```

## DML Statements

### SELECT

```sql
SELECT column1, column2, ...
FROM table_name
[WHERE condition]
[ORDER BY column [ASC|DESC]];
```

### INSERT

```sql
INSERT INTO table_name
[(column1, column2, ...)]
VALUES (value1, value2, ...);
```

### UPDATE

```sql
UPDATE table_name
SET column1 = value1, column2 = value2, ...
[WHERE condition];
```

### DELETE

```sql
DELETE FROM table_name
[WHERE condition];
```

## Transaction Control

```sql
BEGIN;
COMMIT;
ROLLBACK;
```

## Operators

| Type | Operators |
|------|-----------|
| Comparison | `=`, `!=`, `<`, `>`, `<=`, `>=` |
| Arithmetic | `+`, `-`, `*`, `/` |
| Logical | `AND`, `OR`, `NOT` |
