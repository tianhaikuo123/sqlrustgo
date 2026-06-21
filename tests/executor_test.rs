//! Executor 模块完整测试
//! 覆盖 CREATE / INSERT / SELECT / UPDATE / DELETE / DROP 全流程
//! 以及 WHERE 条件过滤、多行操作、错误处理等

use sqlrustgo::{parse, ExecutionEngine, Value};

// ==================== CREATE TABLE ====================

#[test]
fn test_executor_create_table() {
    let mut engine = ExecutionEngine::new();
    let result = engine.execute(parse("CREATE TABLE ex_users (id INTEGER, name TEXT)").unwrap());
    assert!(result.is_ok());
    assert!(engine.get_table("ex_users").is_some());
}

#[test]
fn test_executor_create_table_no_columns() {
    let mut engine = ExecutionEngine::new();
    let result = engine.execute(parse("CREATE TABLE simple_table").unwrap());
    assert!(result.is_ok());
    assert!(engine.get_table("simple_table").is_some());
}

#[test]
fn test_executor_create_multiple_tables() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_t1 (id INTEGER)").unwrap())
        .unwrap();
    engine
        .execute(parse("CREATE TABLE ex_t2 (id INTEGER, val TEXT)").unwrap())
        .unwrap();

    assert!(engine.get_table("ex_t1").is_some());
    assert!(engine.get_table("ex_t2").is_some());
}

// ==================== DROP TABLE ====================

#[test]
fn test_executor_drop_table() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_drop_me (id INTEGER)").unwrap())
        .unwrap();
    assert!(engine.get_table("ex_drop_me").is_some());

    engine
        .execute(parse("DROP TABLE ex_drop_me").unwrap())
        .unwrap();
    assert!(engine.get_table("ex_drop_me").is_none());
}

#[test]
fn test_executor_drop_nonexistent_table() {
    let mut engine = ExecutionEngine::new();
    // drop_table 实现为幂等操作，不存在的表也返回成功
    let result = engine.execute(parse("DROP TABLE nonexistent_table").unwrap());
    assert!(result.is_ok());
}

// ==================== INSERT ====================

#[test]
fn test_executor_insert_single_row() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_ins1 (id INTEGER, name TEXT)").unwrap())
        .unwrap();

    let result = engine.execute(
        parse("INSERT INTO ex_ins1 VALUES (1, 'Alice')").unwrap(),
    );
    assert!(result.is_ok());
    let exec_result = result.unwrap();
    assert_eq!(exec_result.rows_affected, 1);
}

#[test]
fn test_executor_insert_multiple_rows() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_ins2 (id INTEGER, name TEXT)").unwrap())
        .unwrap();

    engine
        .execute(parse("INSERT INTO ex_ins2 VALUES (1, 'Alice')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_ins2 VALUES (2, 'Bob')").unwrap())
        .unwrap();

    let table = engine.get_table("ex_ins2").unwrap();
    assert_eq!(table.rows.len(), 2);
}

#[test]
fn test_executor_insert_nonexistent_table() {
    let mut engine = ExecutionEngine::new();
    let result = engine.execute(
        parse("INSERT INTO no_such_table VALUES (1, 'test')").unwrap(),
    );
    assert!(result.is_err());
}

// ==================== SELECT ====================

#[test]
fn test_executor_select_empty_table() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_sel_empty (id INTEGER, name TEXT)").unwrap())
        .unwrap();

    let result = engine.execute(parse("SELECT * FROM ex_sel_empty").unwrap());
    assert!(result.is_ok());
    let exec_result = result.unwrap();
    assert_eq!(exec_result.rows.len(), 0);
}

#[test]
fn test_executor_select_star() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_sel1 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_sel1 VALUES (1, 'Alice')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_sel1 VALUES (2, 'Bob')").unwrap())
        .unwrap();

    let result = engine
        .execute(parse("SELECT * FROM ex_sel1").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 2);
    assert_eq!(result.columns, vec!["id", "name"]);
}

#[test]
fn test_executor_select_specific_column() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_sel2 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_sel2 VALUES (1, 'Alice')").unwrap())
        .unwrap();

    let result = engine
        .execute(parse("SELECT name FROM ex_sel2").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.columns, vec!["name"]);
    assert_eq!(result.rows[0][0], Value::Text("Alice".to_string()));
}

#[test]
fn test_executor_select_nonexistent_table() {
    let mut engine = ExecutionEngine::new();
    let result = engine.execute(parse("SELECT * FROM ghost_table").unwrap());
    assert!(result.is_err());
}

// ==================== SELECT WHERE ====================

#[test]
fn test_executor_select_where_equal() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_wh1 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh1 VALUES (1, 'Alice')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh1 VALUES (2, 'Bob')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh1 VALUES (3, 'Charlie')").unwrap())
        .unwrap();

    let result = engine
        .execute(parse("SELECT * FROM ex_wh1 WHERE id = 2").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0][0], Value::Integer(2));
    assert_eq!(result.rows[0][1], Value::Text("Bob".to_string()));
}

#[test]
fn test_executor_select_where_greater() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_wh2 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh2 VALUES (1, 'Alice')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh2 VALUES (2, 'Bob')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh2 VALUES (3, 'Charlie')").unwrap())
        .unwrap();

    let result = engine
        .execute(parse("SELECT * FROM ex_wh2 WHERE id > 1").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 2);
}

#[test]
fn test_executor_select_where_less() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_wh3 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh3 VALUES (10, 'A')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh3 VALUES (20, 'B')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh3 VALUES (30, 'C')").unwrap())
        .unwrap();

    let result = engine
        .execute(parse("SELECT * FROM ex_wh3 WHERE id < 20").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0][0], Value::Integer(10));
}

#[test]
fn test_executor_select_where_no_match() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_wh4 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh4 VALUES (1, 'Alice')").unwrap())
        .unwrap();

    let result = engine
        .execute(parse("SELECT * FROM ex_wh4 WHERE id = 999").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 0);
}

#[test]
fn test_executor_select_where_gte() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_wh5 (id INTEGER, val TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh5 VALUES (1, 'a')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh5 VALUES (2, 'b')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_wh5 VALUES (3, 'c')").unwrap())
        .unwrap();

    // >= 2 should return rows with id 2 and 3
    let result = engine
        .execute(parse("SELECT * FROM ex_wh5 WHERE id >= 2").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 2);
}

// ==================== UPDATE ====================

#[test]
fn test_executor_update_with_where() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_upd1 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_upd1 VALUES (1, 'Alice')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_upd1 VALUES (2, 'Bob')").unwrap())
        .unwrap();

    let result = engine.execute(
        parse("UPDATE ex_upd1 SET name = 'Updated' WHERE id = 1").unwrap(),
    );
    assert!(result.is_ok());

    // Verify: Alice → Updated, Bob unchanged
    let select = engine
        .execute(parse("SELECT * FROM ex_upd1 WHERE id = 1").unwrap())
        .unwrap();
    assert_eq!(select.rows[0][1], Value::Text("Updated".to_string()));

    let bob = engine
        .execute(parse("SELECT * FROM ex_upd1 WHERE id = 2").unwrap())
        .unwrap();
    assert_eq!(bob.rows[0][1], Value::Text("Bob".to_string()));
}

#[test]
fn test_executor_update_all_rows() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_upd2 (id INTEGER, status TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_upd2 VALUES (1, 'active')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_upd2 VALUES (2, 'active')").unwrap())
        .unwrap();

    // UPDATE without WHERE → updates all rows
    let result = engine.execute(
        parse("UPDATE ex_upd2 SET status = 'inactive'").unwrap(),
    );
    assert!(result.is_ok());
    let exec_result = result.unwrap();
    assert_eq!(exec_result.rows_affected, 2);
}

#[test]
fn test_executor_update_nonexistent_table() {
    let mut engine = ExecutionEngine::new();
    let result = engine.execute(
        parse("UPDATE ghost SET name = 'x' WHERE id = 1").unwrap(),
    );
    assert!(result.is_err());
}

// ==================== DELETE ====================

#[test]
fn test_executor_delete_with_where() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_del1 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_del1 VALUES (1, 'Alice')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_del1 VALUES (2, 'Bob')").unwrap())
        .unwrap();

    let result = engine.execute(
        parse("DELETE FROM ex_del1 WHERE id = 1").unwrap(),
    );
    assert!(result.is_ok());
    let exec_result = result.unwrap();
    assert_eq!(exec_result.rows_affected, 1);

    // Verify only Bob remains
    let table = engine.get_table("ex_del1").unwrap();
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0][0], Value::Integer(2));
}

#[test]
fn test_executor_delete_all() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_del2 (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_del2 VALUES (1, 'A')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_del2 VALUES (2, 'B')").unwrap())
        .unwrap();

    // DELETE without WHERE → deletes all rows
    let result = engine.execute(parse("DELETE FROM ex_del2").unwrap());
    assert!(result.is_ok());

    let table = engine.get_table("ex_del2").unwrap();
    assert_eq!(table.rows.len(), 0);
}

#[test]
fn test_executor_delete_nonexistent_table() {
    let mut engine = ExecutionEngine::new();
    let result = engine.execute(parse("DELETE FROM ghost_table WHERE id = 1").unwrap());
    assert!(result.is_err());
}

// ==================== 完整工作流 ====================

#[test]
fn test_executor_full_crud_workflow() {
    let mut engine = ExecutionEngine::new();

    // 1. Create
    engine
        .execute(parse("CREATE TABLE ex_crud (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    assert!(engine.get_table("ex_crud").is_some());

    // 2. Insert
    engine
        .execute(parse("INSERT INTO ex_crud VALUES (1, 'Alice')").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_crud VALUES (2, 'Bob')").unwrap())
        .unwrap();

    // 3. Read
    let result = engine
        .execute(parse("SELECT * FROM ex_crud").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 2);

    // 4. Update
    engine
        .execute(parse("UPDATE ex_crud SET name = 'Charlie' WHERE id = 1").unwrap())
        .unwrap();
    let updated = engine
        .execute(parse("SELECT * FROM ex_crud WHERE id = 1").unwrap())
        .unwrap();
    assert_eq!(updated.rows[0][1], Value::Text("Charlie".to_string()));

    // 5. Delete
    engine
        .execute(parse("DELETE FROM ex_crud WHERE id = 2").unwrap())
        .unwrap();
    let remaining = engine
        .execute(parse("SELECT * FROM ex_crud").unwrap())
        .unwrap();
    assert_eq!(remaining.rows.len(), 1);

    // 6. Drop
    engine
        .execute(parse("DROP TABLE ex_crud").unwrap())
        .unwrap();
    assert!(engine.get_table("ex_crud").is_none());
}

#[test]
fn test_executor_multi_insert_then_filter() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_filter (id INTEGER, age INTEGER)").unwrap())
        .unwrap();

    // Insert 5 rows
    for i in 1..=5 {
        let sql = format!("INSERT INTO ex_filter VALUES ({}, {})", i, i * 10);
        engine.execute(parse(&sql).unwrap()).unwrap();
    }

    // Select all
    let all = engine
        .execute(parse("SELECT * FROM ex_filter").unwrap())
        .unwrap();
    assert_eq!(all.rows.len(), 5);

    // Select where age > 30 (should get id=4(age=40), id=5(age=50))
    let filtered = engine
        .execute(parse("SELECT * FROM ex_filter WHERE age > 30").unwrap())
        .unwrap();
    assert_eq!(filtered.rows.len(), 2);
}

// ==================== Value 类型验证 ====================

#[test]
fn test_executor_value_types() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_types (id INTEGER, name TEXT)").unwrap())
        .unwrap();
    engine
        .execute(parse("INSERT INTO ex_types VALUES (42, 'hello')").unwrap())
        .unwrap();

    let result = engine
        .execute(parse("SELECT * FROM ex_types").unwrap())
        .unwrap();
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0][0], Value::Integer(42));
    assert_eq!(result.rows[0][1], Value::Text("hello".to_string()));
}

// ==================== ANALYZE ====================

#[test]
fn test_executor_analyze() {
    let mut engine = ExecutionEngine::new();
    engine
        .execute(parse("CREATE TABLE ex_analyze (id INTEGER)").unwrap())
        .unwrap();

    let result = engine.execute(parse("ANALYZE ex_analyze").unwrap());
    assert!(result.is_ok());
}
