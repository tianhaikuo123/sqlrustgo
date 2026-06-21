//! Parser 模块完整测试
//! 覆盖 SELECT / INSERT / UPDATE / DELETE / CREATE TABLE / DROP TABLE / ANALYZE
//! 以及 WHERE 表达式、错误处理和边界情况

use sqlrustgo::parse;
use sqlrustgo::parser::{Expression, Statement};

// ==================== SELECT ====================

#[test]
fn test_parse_select_star() {
    let stmt = parse("SELECT * FROM users").unwrap();
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.columns.len(), 1);
            assert_eq!(s.columns[0].name, "*");
            assert_eq!(s.table, "users");
            assert!(s.where_clause.is_none());
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_single_column() {
    let stmt = parse("SELECT id FROM users").unwrap();
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.columns.len(), 1);
            assert_eq!(s.columns[0].name, "id");
            assert_eq!(s.table, "users");
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_multiple_columns() {
    let stmt = parse("SELECT id, name, age FROM users").unwrap();
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.columns.len(), 3);
            assert_eq!(s.columns[0].name, "id");
            assert_eq!(s.columns[1].name, "name");
            assert_eq!(s.columns[2].name, "age");
            assert_eq!(s.table, "users");
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_where_equal() {
    let stmt = parse("SELECT * FROM users WHERE id = 1").unwrap();
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.table, "users");
            assert!(s.where_clause.is_some());
            match s.where_clause.unwrap() {
                Expression::BinaryOp(left, op, right) => {
                    assert_eq!(op, "=");
                    assert!(matches!(*left, Expression::Identifier(ref n) if n == "id"));
                    assert!(matches!(*right, Expression::Literal(ref v) if v == "1"));
                }
                _ => panic!("Expected BinaryOp"),
            }
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_where_greater() {
    let stmt = parse("SELECT * FROM users WHERE age > 18").unwrap();
    match stmt {
        Statement::Select(s) => {
            assert!(s.where_clause.is_some());
            match s.where_clause.unwrap() {
                Expression::BinaryOp(_, op, _) => assert_eq!(op, ">"),
                _ => panic!("Expected BinaryOp"),
            }
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_where_less_equal() {
    let stmt = parse("SELECT * FROM users WHERE age <= 65").unwrap();
    match stmt {
        Statement::Select(s) => {
            match s.where_clause.unwrap() {
                Expression::BinaryOp(_, op, _) => assert_eq!(op, "<="),
                _ => panic!("Expected BinaryOp"),
            }
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_where_not_equal() {
    let stmt = parse("SELECT * FROM users WHERE status != 0").unwrap();
    match stmt {
        Statement::Select(s) => {
            match s.where_clause.unwrap() {
                Expression::BinaryOp(_, op, _) => assert_eq!(op, "!="),
                _ => panic!("Expected BinaryOp"),
            }
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_where_and() {
    let stmt = parse("SELECT * FROM users WHERE age > 18 AND age < 65").unwrap();
    match stmt {
        Statement::Select(s) => {
            match s.where_clause.unwrap() {
                Expression::BinaryOp(_, op, _) => assert_eq!(op, "AND"),
                _ => panic!("Expected BinaryOp with AND"),
            }
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_where_or() {
    let stmt = parse("SELECT * FROM users WHERE id = 1 OR id = 2").unwrap();
    match stmt {
        Statement::Select(s) => {
            match s.where_clause.unwrap() {
                Expression::BinaryOp(_, op, _) => assert_eq!(op, "OR"),
                _ => panic!("Expected BinaryOp with OR"),
            }
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_select_where_string() {
    let stmt = parse("SELECT * FROM users WHERE name = 'Alice'").unwrap();
    match stmt {
        Statement::Select(s) => {
            match s.where_clause.unwrap() {
                Expression::BinaryOp(_, op, right) => {
                    assert_eq!(op, "=");
                    assert!(matches!(*right, Expression::Literal(ref v) if v == "'Alice'"));
                }
                _ => panic!("Expected BinaryOp"),
            }
        }
        _ => panic!("Expected SELECT"),
    }
}

// ==================== INSERT ====================

#[test]
fn test_parse_insert_single_value() {
    let stmt = parse("INSERT INTO users VALUES (1)").unwrap();
    match stmt {
        Statement::Insert(i) => {
            assert_eq!(i.table, "users");
            assert_eq!(i.values.len(), 1);
            assert_eq!(i.values[0].len(), 1);
            assert!(i.columns.is_empty());
        }
        _ => panic!("Expected INSERT"),
    }
}

#[test]
fn test_parse_insert_multiple_values() {
    let stmt = parse("INSERT INTO users VALUES (1, 'Alice', 25)").unwrap();
    match stmt {
        Statement::Insert(i) => {
            assert_eq!(i.table, "users");
            assert_eq!(i.values.len(), 1);
            assert_eq!(i.values[0].len(), 3);
        }
        _ => panic!("Expected INSERT"),
    }
}

#[test]
fn test_parse_insert_with_columns() {
    let stmt = parse("INSERT INTO users (id, name) VALUES (1, 'Alice')").unwrap();
    match stmt {
        Statement::Insert(i) => {
            assert_eq!(i.table, "users");
            assert_eq!(i.columns, vec!["id", "name"]);
            assert_eq!(i.values.len(), 1);
            assert_eq!(i.values[0].len(), 2);
        }
        _ => panic!("Expected INSERT"),
    }
}

#[test]
fn test_parse_insert_multi_row() {
    let stmt = parse("INSERT INTO users VALUES (1, 'Alice'), (2, 'Bob'), (3, 'Charlie')").unwrap();
    match stmt {
        Statement::Insert(i) => {
            assert_eq!(i.table, "users");
            assert_eq!(i.values.len(), 3);
            for row in &i.values {
                assert_eq!(row.len(), 2);
            }
        }
        _ => panic!("Expected INSERT"),
    }
}

#[test]
fn test_parse_insert_null_value() {
    let stmt = parse("INSERT INTO users VALUES (1, NULL)").unwrap();
    match stmt {
        Statement::Insert(i) => {
            assert_eq!(i.values[0].len(), 2);
            // NULL is parsed as Expression::Literal("NULL")
            assert!(matches!(&i.values[0][1], Expression::Literal(ref v) if v == "NULL"));
        }
        _ => panic!("Expected INSERT"),
    }
}

#[test]
fn test_parse_insert_negative_value() {
    let stmt = parse("INSERT INTO users VALUES (-1, 'Test')").unwrap();
    match stmt {
        Statement::Insert(i) => {
            assert!(matches!(&i.values[0][0], Expression::Literal(ref v) if v == "-1"));
        }
        _ => panic!("Expected INSERT"),
    }
}

// ==================== UPDATE ====================

#[test]
fn test_parse_update_simple() {
    let stmt = parse("UPDATE users SET name = 'Bob'").unwrap();
    match stmt {
        Statement::Update(u) => {
            assert_eq!(u.table, "users");
            assert_eq!(u.set_clauses.len(), 1);
            assert_eq!(u.set_clauses[0].0, "name");
            assert!(u.where_clause.is_none());
        }
        _ => panic!("Expected UPDATE"),
    }
}

#[test]
fn test_parse_update_with_where() {
    let stmt = parse("UPDATE users SET name = 'Bob' WHERE id = 1").unwrap();
    match stmt {
        Statement::Update(u) => {
            assert_eq!(u.table, "users");
            assert_eq!(u.set_clauses.len(), 1);
            assert!(u.where_clause.is_some());
        }
        _ => panic!("Expected UPDATE"),
    }
}

#[test]
fn test_parse_update_multiple_set() {
    let stmt = parse("UPDATE users SET name = 'Bob', age = 30 WHERE id = 1").unwrap();
    match stmt {
        Statement::Update(u) => {
            assert_eq!(u.set_clauses.len(), 2);
            assert_eq!(u.set_clauses[0].0, "name");
            assert_eq!(u.set_clauses[1].0, "age");
            assert!(u.where_clause.is_some());
        }
        _ => panic!("Expected UPDATE"),
    }
}

// ==================== DELETE ====================

#[test]
fn test_parse_delete_all() {
    let stmt = parse("DELETE FROM users").unwrap();
    match stmt {
        Statement::Delete(d) => {
            assert_eq!(d.table, "users");
            assert!(d.where_clause.is_none());
        }
        _ => panic!("Expected DELETE"),
    }
}

#[test]
fn test_parse_delete_with_where() {
    let stmt = parse("DELETE FROM users WHERE id = 1").unwrap();
    match stmt {
        Statement::Delete(d) => {
            assert_eq!(d.table, "users");
            assert!(d.where_clause.is_some());
        }
        _ => panic!("Expected DELETE"),
    }
}

#[test]
fn test_parse_delete_where_comparison() {
    let stmt = parse("DELETE FROM users WHERE age > 100").unwrap();
    match stmt {
        Statement::Delete(d) => {
            match d.where_clause.unwrap() {
                Expression::BinaryOp(_, op, _) => assert_eq!(op, ">"),
                _ => panic!("Expected BinaryOp"),
            }
        }
        _ => panic!("Expected DELETE"),
    }
}

// ==================== CREATE TABLE ====================

#[test]
fn test_parse_create_table_simple() {
    let stmt = parse("CREATE TABLE users").unwrap();
    match stmt {
        Statement::CreateTable(c) => {
            assert_eq!(c.name, "users");
            assert!(c.columns.is_empty());
        }
        _ => panic!("Expected CREATE TABLE"),
    }
}

#[test]
fn test_parse_create_table_with_columns() {
    let stmt = parse("CREATE TABLE users (id INTEGER, name TEXT)").unwrap();
    match stmt {
        Statement::CreateTable(c) => {
            assert_eq!(c.name, "users");
            assert_eq!(c.columns.len(), 2);
            assert_eq!(c.columns[0].name, "id");
            assert_eq!(c.columns[0].data_type, "INTEGER");
            assert_eq!(c.columns[1].name, "name");
            assert_eq!(c.columns[1].data_type, "TEXT");
        }
        _ => panic!("Expected CREATE TABLE"),
    }
}

#[test]
fn test_parse_create_table_three_columns() {
    let stmt = parse("CREATE TABLE products (id INTEGER, name TEXT, price FLOAT)").unwrap();
    match stmt {
        Statement::CreateTable(c) => {
            assert_eq!(c.name, "products");
            assert_eq!(c.columns.len(), 3);
            assert_eq!(c.columns[2].name, "price");
        }
        _ => panic!("Expected CREATE TABLE"),
    }
}

// ==================== DROP TABLE ====================

#[test]
fn test_parse_drop_table() {
    let stmt = parse("DROP TABLE users").unwrap();
    match stmt {
        Statement::DropTable(d) => {
            assert_eq!(d.name, "users");
        }
        _ => panic!("Expected DROP TABLE"),
    }
}

#[test]
fn test_parse_drop_table_different_name() {
    let stmt = parse("DROP TABLE orders").unwrap();
    match stmt {
        Statement::DropTable(d) => {
            assert_eq!(d.name, "orders");
        }
        _ => panic!("Expected DROP TABLE"),
    }
}

// ==================== ANALYZE ====================

#[test]
fn test_parse_analyze_with_table() {
    let stmt = parse("ANALYZE users").unwrap();
    match stmt {
        Statement::Analyze(a) => {
            assert_eq!(a.table_name, Some("users".to_string()));
        }
        _ => panic!("Expected ANALYZE"),
    }
}

// ==================== 错误处理 ====================

#[test]
fn test_parse_invalid_sql() {
    let result = parse("INVALID SQL SYNTAX");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_input() {
    let result = parse("");
    assert!(result.is_err());
}

#[test]
fn test_parse_incomplete_select() {
    // SELECT without FROM should fail
    let result = parse("SELECT id");
    assert!(result.is_err());
}

#[test]
fn test_parse_insert_missing_values() {
    let result = parse("INSERT INTO users");
    assert!(result.is_err());
}

// ==================== 大小写不敏感 ====================

#[test]
fn test_parse_case_insensitive_keywords() {
    let stmt = parse("select id from users where id = 1").unwrap();
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.table, "users");
            assert_eq!(s.columns.len(), 1);
            assert!(s.where_clause.is_some());
        }
        _ => panic!("Expected SELECT"),
    }
}

#[test]
fn test_parse_mixed_case() {
    let stmt = parse("Select id From users").unwrap();
    assert!(matches!(stmt, Statement::Select(_)));
}
