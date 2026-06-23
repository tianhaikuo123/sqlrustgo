# SQL 参考

## 支持的语句

### SELECT

```sql
SELECT [column_list | *]
FROM table_name
[WHERE condition]
[ORDER BY column [ASC|DESC]]
```

### INSERT

```sql
INSERT INTO table_name [(col1, col2, ...)]
VALUES (val1, val2, ...)
```

### UPDATE

```sql
UPDATE table_name
SET col1 = val1 [, col2 = val2 ...]
[WHERE condition]
```

### DELETE

```sql
DELETE FROM table_name
[WHERE condition]
```

### CREATE TABLE

```sql
CREATE TABLE table_name (
    col1 TYPE,
    col2 TYPE,
    ...
)
```

### DROP TABLE

```sql
DROP TABLE table_name
```
