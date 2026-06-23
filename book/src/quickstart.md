# 快速开始

## 启动数据库

```bash
cargo run --bin sqlrustgo
```

## 基本操作

### 创建表

```sql
CREATE TABLE students (id INT, name TEXT, score FLOAT);
```

### 插入数据

```sql
INSERT INTO students VALUES (1, '张三', 95.5);
INSERT INTO students VALUES (2, '李四', 88.0);
INSERT INTO students VALUES (3, '王五', 92.3);
```

### 查询数据

```sql
-- 查询全部
SELECT * FROM students;

-- 条件查询
SELECT name, score FROM students WHERE score > 90;

-- 排序
SELECT * FROM students ORDER BY score DESC;
```

### 更新数据

```sql
UPDATE students SET score = 96.0 WHERE id = 1;
```

### 删除数据

```sql
DELETE FROM students WHERE id = 3;
```

### 删除表

```sql
DROP TABLE students;
```
